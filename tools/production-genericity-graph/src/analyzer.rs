use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path};

use quote::ToTokens;
use sha2::{Digest, Sha256};
use syn::visit::{self, Visit};
use syn::visit_mut::{self, VisitMut};
use syn::{Attribute, Block, Expr, ImplItem, Item, Stmt, TraitItem, UseTree};

use crate::config::{Configuration, path_attribute};
use crate::inventory::{InventoryRow, read_inventory};
use crate::metadata::{TargetContext, load_target_contexts};
use crate::model::{
    CallEdge, GRAPH_SCHEMA_VERSION, ImportEdge, InventoryFile, ItemNode, MacroNode, ModuleNode,
    ProductionGraph, TargetNode,
};

#[derive(Clone)]
pub(crate) struct ModuleUnit {
    pub(crate) id: String,
    pub(crate) target: String,
    pub(crate) module_path: Vec<String>,
    pub(crate) source_path: String,
    module_dir: String,
    attribute_dir: String,
    pub(crate) inline: bool,
    pub(crate) items: Vec<Item>,
}

pub(crate) struct AnalyzedProgram {
    pub(crate) graph: ProductionGraph,
    pub(crate) modules: BTreeMap<String, ModuleUnit>,
}

#[derive(Debug, Clone)]
struct RawImport {
    module: String,
    module_path: Vec<String>,
    source: Vec<String>,
    binding: Option<String>,
    glob: bool,
    ordinal: usize,
}

#[derive(Debug, Clone)]
enum RawCallKind {
    Path(Vec<String>),
    Method(String),
    CallableExpression,
}

#[derive(Debug, Clone)]
struct RawCall {
    caller: String,
    module: String,
    module_path: Vec<String>,
    owner: Option<String>,
    syntax: String,
    kind: RawCallKind,
    ordinal: usize,
}

#[derive(Debug, Clone)]
struct RawMacro {
    module: String,
    containing_item: Option<String>,
    name: String,
    kind: String,
    ordinal: usize,
}

#[derive(Debug, Clone)]
struct IndexedItem {
    node: ItemNode,
    module_path: Vec<String>,
}

#[derive(Debug, Clone)]
struct UseLeaf {
    source: Vec<String>,
    binding: Option<String>,
    glob: bool,
}

pub(crate) fn analyze(root: &Path) -> Result<ProductionGraph, String> {
    Ok(analyze_program(root)?.graph)
}

pub(crate) fn analyze_program(root: &Path) -> Result<AnalyzedProgram, String> {
    let root = root
        .canonicalize()
        .map_err(|error| format!("cannot resolve repository root: {error}"))?;
    let inventory = read_inventory(&root)?;
    let contexts = load_target_contexts(&root)?;
    let parsed = parse_inventory_files(&root, &inventory)?;

    let mut configurations = BTreeMap::new();
    for context in &contexts {
        configurations.insert(
            context.id.clone(),
            Configuration::from_rustc(
                &root,
                context.features.clone(),
                context.declared_features.clone(),
            )?,
        );
    }

    let mut modules = BTreeMap::new();
    let mut reachable_files = BTreeSet::new();
    for context in &contexts {
        if !inventory.contains_key(&context.root) {
            return Err(format!(
                "Cargo target '{}' root '{}' is absent from module_inventory.tsv",
                context.id, context.root
            ));
        }
        let config = &configurations[&context.id];
        let file = parsed
            .get(&context.root)
            .expect("inventoried target root was parsed");
        let mut file = file.clone();
        sanitize_file(&mut file, config)?;
        let root_unit = ModuleUnit {
            id: module_id(&context.id, &["crate".to_owned()]),
            target: context.id.clone(),
            module_path: vec!["crate".to_owned()],
            source_path: context.root.clone(),
            module_dir: parent_path(&context.root)?,
            attribute_dir: parent_path(&context.root)?,
            inline: false,
            items: file.items,
        };
        walk_modules(
            &root,
            context,
            config,
            root_unit,
            &inventory,
            &parsed,
            &mut modules,
            &mut reachable_files,
        )?;
    }

    let mut excluded_test_support_files = Vec::new();
    for (path, row) in &inventory {
        if reachable_files.contains(path) {
            continue;
        }
        if row.current_plane != "test_support" {
            return Err(format!(
                "inventoried non-test module '{path}' is unreachable from all production Cargo targets"
            ));
        }
        excluded_test_support_files.push(path.clone());
    }

    let mut items = BTreeMap::new();
    let mut raw_imports = Vec::new();
    let mut raw_calls = Vec::new();
    let mut raw_macros = Vec::new();
    for unit in modules.values() {
        collect_module_surface(
            unit,
            &mut items,
            &mut raw_imports,
            &mut raw_calls,
            &mut raw_macros,
        )?;
    }

    let module_nodes: Vec<_> = modules
        .values()
        .map(|unit| ModuleNode {
            id: unit.id.clone(),
            target: unit.target.clone(),
            module_path: unit.module_path.join("::"),
            source_path: unit.source_path.clone(),
            inline: unit.inline,
        })
        .collect();
    let context_by_id: BTreeMap<_, _> = contexts
        .iter()
        .map(|context| (context.id.clone(), context))
        .collect();
    let imports = resolve_imports(&raw_imports, &modules, &items, &context_by_id)?;
    let calls = resolve_calls(&raw_calls, &modules, &items, &imports, &context_by_id)?;
    let macros = resolve_macros(&raw_macros, &imports, &context_by_id)?;

    let target_nodes = contexts
        .iter()
        .map(|context| TargetNode {
            id: context.id.clone(),
            package: context.package.clone(),
            target: context.target.clone(),
            kind: context.kind.clone(),
            root: context.root.clone(),
            enabled_features: context.features.iter().cloned().collect(),
        })
        .collect();
    let graph = ProductionGraph {
        schema_version: GRAPH_SCHEMA_VERSION,
        inventory_files: inventory.values().map(InventoryFile::from).collect(),
        reachable_files: reachable_files.into_iter().collect(),
        excluded_test_support_files,
        targets: target_nodes,
        modules: module_nodes,
        items: items.values().map(|item| item.node.clone()).collect(),
        imports,
        calls,
        macros,
    };
    Ok(AnalyzedProgram { graph, modules })
}

fn parse_inventory_files(
    root: &Path,
    inventory: &BTreeMap<String, InventoryRow>,
) -> Result<BTreeMap<String, syn::File>, String> {
    let mut parsed = BTreeMap::new();
    for path in inventory.keys() {
        let source = fs::read_to_string(root.join(path))
            .map_err(|error| format!("cannot read inventoried module '{path}': {error}"))?;
        let file = syn::parse_file(&source)
            .map_err(|error| format!("cannot parse inventoried module '{path}': {error}"))?;
        parsed.insert(path.clone(), file);
    }
    Ok(parsed)
}

#[allow(clippy::too_many_arguments)]
fn walk_modules(
    root: &Path,
    context: &TargetContext,
    config: &Configuration,
    unit: ModuleUnit,
    inventory: &BTreeMap<String, InventoryRow>,
    parsed: &BTreeMap<String, syn::File>,
    modules: &mut BTreeMap<String, ModuleUnit>,
    reachable_files: &mut BTreeSet<String>,
) -> Result<(), String> {
    if modules.contains_key(&unit.id) {
        return Err(format!("duplicate module identity '{}'", unit.id));
    }
    reachable_files.insert(unit.source_path.clone());
    let mut children = Vec::new();
    for item in &unit.items {
        let Item::Mod(module) = item else {
            continue;
        };
        if !config.attributes_are_active(&module.attrs)? {
            continue;
        }
        let name = module.ident.to_string();
        let mut module_path = unit.module_path.clone();
        module_path.push(name.clone());
        let id = module_id(&context.id, &module_path);
        if let Some((_, items)) = &module.content {
            let mut items = items.clone();
            sanitize_items(&mut items, config)?;
            children.push(ModuleUnit {
                id,
                target: context.id.clone(),
                module_path,
                source_path: unit.source_path.clone(),
                module_dir: join_relative(&unit.module_dir, &name)?,
                attribute_dir: join_relative(&unit.module_dir, &name)?,
                inline: true,
                items,
            });
            continue;
        }
        let source_path = resolve_module_file(
            root,
            &unit.attribute_dir,
            &unit.module_dir,
            &name,
            &module.attrs,
        )?;
        if !inventory.contains_key(&source_path) {
            return Err(format!(
                "module '{}' resolves to uninventoried source '{source_path}'",
                module_path.join("::")
            ));
        }
        let mut file = parsed
            .get(&source_path)
            .expect("inventoried module source was parsed")
            .clone();
        sanitize_file(&mut file, config)?;
        children.push(ModuleUnit {
            id,
            target: context.id.clone(),
            module_path,
            module_dir: module_directory(&source_path)?,
            attribute_dir: parent_path(&source_path)?,
            source_path,
            inline: false,
            items: file.items,
        });
    }
    modules.insert(unit.id.clone(), unit);
    children.sort_by(|left, right| left.id.cmp(&right.id));
    for child in children {
        walk_modules(
            root,
            context,
            config,
            child,
            inventory,
            parsed,
            modules,
            reachable_files,
        )?;
    }
    Ok(())
}

fn resolve_module_file(
    root: &Path,
    attribute_dir: &str,
    module_dir: &str,
    name: &str,
    attrs: &[Attribute],
) -> Result<String, String> {
    if let Some(path) = path_attribute(attrs)? {
        let candidate = join_relative(attribute_dir, &path)?;
        if !root.join(&candidate).is_file() {
            return Err(format!(
                "module path attribute resolves to missing '{candidate}'"
            ));
        }
        return Ok(candidate);
    }
    let candidates = [
        join_relative(module_dir, &format!("{name}.rs"))?,
        join_relative(module_dir, &format!("{name}/mod.rs"))?,
    ];
    let existing: Vec<_> = candidates
        .into_iter()
        .filter(|candidate| root.join(candidate).is_file())
        .collect();
    match existing.as_slice() {
        [path] => Ok(path.clone()),
        [] => Err(format!(
            "module '{name}' has no source below '{module_dir}'"
        )),
        _ => Err(format!("module '{name}' is ambiguous below '{module_dir}'")),
    }
}

fn module_directory(source_path: &str) -> Result<String, String> {
    let path = Path::new(source_path);
    let parent = path
        .parent()
        .ok_or_else(|| format!("module source '{source_path}' has no parent"))?;
    if path.file_name().is_some_and(|name| name == "mod.rs") {
        return path_string(parent);
    }
    let stem = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or_else(|| format!("module source '{source_path}' has no UTF-8 stem"))?;
    join_relative(&path_string(parent)?, stem)
}

fn parent_path(path: &str) -> Result<String, String> {
    path_string(
        Path::new(path)
            .parent()
            .ok_or_else(|| format!("path '{path}' has no parent"))?,
    )
}

fn join_relative(base: &str, child: &str) -> Result<String, String> {
    if Path::new(child).is_absolute() {
        return Err(format!("absolute module path '{child}' is forbidden"));
    }
    let mut components = Vec::new();
    for component in Path::new(base).join(child).components() {
        match component {
            Component::Normal(value) => {
                let value = value
                    .to_str()
                    .ok_or_else(|| "module path contains non-UTF-8 bytes".to_owned())?;
                components.push(value.to_owned());
            }
            Component::CurDir => {}
            Component::ParentDir => {
                if components.pop().is_none() {
                    return Err(format!("module path '{child}' escapes the repository"));
                }
            }
            Component::RootDir | Component::Prefix(_) => {
                return Err(format!("module path '{child}' is not repository-relative"));
            }
        }
    }
    if components.is_empty() {
        return Err(format!(
            "module path '{child}' resolves to the repository root"
        ));
    }
    Ok(components.join("/"))
}

fn path_string(path: &Path) -> Result<String, String> {
    path.to_str()
        .map(|value| value.replace('\\', "/"))
        .ok_or_else(|| "module path contains non-UTF-8 bytes".to_owned())
}

fn module_id(target: &str, path: &[String]) -> String {
    format!("{target}::{}", path.join("::"))
}

fn sanitize_file(file: &mut syn::File, config: &Configuration) -> Result<(), String> {
    let mut sanitizer = ProductionSanitizer::new(config);
    sanitizer.visit_file_mut(file);
    sanitizer.finish()?;
    reject_verbatim(file)
}

fn sanitize_items(items: &mut Vec<Item>, config: &Configuration) -> Result<(), String> {
    let mut file = syn::File {
        shebang: None,
        attrs: Vec::new(),
        items: std::mem::take(items),
    };
    sanitize_file(&mut file, config)?;
    *items = file.items;
    Ok(())
}

struct ProductionSanitizer<'a> {
    config: &'a Configuration,
    error: Option<String>,
}

impl<'a> ProductionSanitizer<'a> {
    fn new(config: &'a Configuration) -> Self {
        Self {
            config,
            error: None,
        }
    }

    fn keep(&mut self, attrs: &[Attribute]) -> bool {
        if self.error.is_some() {
            return false;
        }
        match self.config.attributes_are_active(attrs) {
            Ok(active) => active,
            Err(error) => {
                self.error = Some(error);
                false
            }
        }
    }

    fn finish(self) -> Result<(), String> {
        self.error.map_or(Ok(()), Err)
    }
}

impl VisitMut for ProductionSanitizer<'_> {
    fn visit_file_mut(&mut self, file: &mut syn::File) {
        file.items.retain(|item| self.keep(item_attrs(item)));
        visit_mut::visit_file_mut(self, file);
    }

    fn visit_item_mod_mut(&mut self, module: &mut syn::ItemMod) {
        if let Some((_, items)) = &mut module.content {
            items.retain(|item| self.keep(item_attrs(item)));
        }
        visit_mut::visit_item_mod_mut(self, module);
    }

    fn visit_item_impl_mut(&mut self, item: &mut syn::ItemImpl) {
        item.items
            .retain(|member| self.keep(impl_item_attrs(member)));
        visit_mut::visit_item_impl_mut(self, item);
    }

    fn visit_item_enum_mut(&mut self, item: &mut syn::ItemEnum) {
        item.variants = std::mem::take(&mut item.variants)
            .into_iter()
            .filter(|variant| self.keep(&variant.attrs))
            .collect();
        visit_mut::visit_item_enum_mut(self, item);
    }

    fn visit_fields_named_mut(&mut self, fields: &mut syn::FieldsNamed) {
        fields.named = std::mem::take(&mut fields.named)
            .into_iter()
            .filter(|field| self.keep(&field.attrs))
            .collect();
        visit_mut::visit_fields_named_mut(self, fields);
    }

    fn visit_fields_unnamed_mut(&mut self, fields: &mut syn::FieldsUnnamed) {
        fields.unnamed = std::mem::take(&mut fields.unnamed)
            .into_iter()
            .filter(|field| self.keep(&field.attrs))
            .collect();
        visit_mut::visit_fields_unnamed_mut(self, fields);
    }

    fn visit_item_trait_mut(&mut self, item: &mut syn::ItemTrait) {
        item.items
            .retain(|member| self.keep(trait_item_attrs(member)));
        visit_mut::visit_item_trait_mut(self, item);
    }

    fn visit_block_mut(&mut self, block: &mut Block) {
        block.stmts.retain(|statement| {
            let attrs = match statement {
                Stmt::Local(local) => local.attrs.clone(),
                Stmt::Item(item) => item_attrs(item).to_vec(),
                Stmt::Expr(expression, _) => outer_attributes(expression),
                Stmt::Macro(macro_statement) => macro_statement.attrs.clone(),
            };
            self.keep(&attrs)
        });
        visit_mut::visit_block_mut(self, block);
    }

    fn visit_expr_match_mut(&mut self, expression: &mut syn::ExprMatch) {
        expression.arms.retain(|arm| self.keep(&arm.attrs));
        visit_mut::visit_expr_match_mut(self, expression);
    }

    fn visit_expr_struct_mut(&mut self, expression: &mut syn::ExprStruct) {
        expression.fields = std::mem::take(&mut expression.fields)
            .into_iter()
            .filter(|field| self.keep(&field.attrs))
            .collect();
        visit_mut::visit_expr_struct_mut(self, expression);
    }

    fn visit_attribute_mut(&mut self, attribute: &mut Attribute) {
        if self.error.is_some() {
            return;
        }
        if attribute.path().is_ident("cfg_attr") {
            self.error = Some("cfg_attr is unsupported in the production graph".to_owned());
            return;
        }
        if attribute.path().is_ident("cfg") {
            match self
                .config
                .attributes_are_active(std::slice::from_ref(attribute))
            {
                Ok(true) => {}
                Ok(false) => {
                    self.error = Some(
                        "inactive cfg attribute occurs at an unsupported syntax position"
                            .to_owned(),
                    );
                }
                Err(error) => self.error = Some(error),
            }
        }
        visit_mut::visit_attribute_mut(self, attribute);
    }
}

fn item_attrs(item: &Item) -> &[Attribute] {
    match item {
        Item::Const(item) => &item.attrs,
        Item::Enum(item) => &item.attrs,
        Item::ExternCrate(item) => &item.attrs,
        Item::Fn(item) => &item.attrs,
        Item::ForeignMod(item) => &item.attrs,
        Item::Impl(item) => &item.attrs,
        Item::Macro(item) => &item.attrs,
        Item::Mod(item) => &item.attrs,
        Item::Static(item) => &item.attrs,
        Item::Struct(item) => &item.attrs,
        Item::Trait(item) => &item.attrs,
        Item::TraitAlias(item) => &item.attrs,
        Item::Type(item) => &item.attrs,
        Item::Union(item) => &item.attrs,
        Item::Use(item) => &item.attrs,
        Item::Verbatim(_) => &[],
        _ => &[],
    }
}

fn impl_item_attrs(item: &ImplItem) -> &[Attribute] {
    match item {
        ImplItem::Const(item) => &item.attrs,
        ImplItem::Fn(item) => &item.attrs,
        ImplItem::Type(item) => &item.attrs,
        ImplItem::Macro(item) => &item.attrs,
        _ => &[],
    }
}

fn trait_item_attrs(item: &TraitItem) -> &[Attribute] {
    match item {
        TraitItem::Const(item) => &item.attrs,
        TraitItem::Fn(item) => &item.attrs,
        TraitItem::Type(item) => &item.attrs,
        TraitItem::Macro(item) => &item.attrs,
        _ => &[],
    }
}

fn outer_attributes(value: &impl ToTokens) -> Vec<Attribute> {
    use syn::parse::{Parse, ParseStream};

    struct AttributePrefix(Vec<Attribute>);

    impl Parse for AttributePrefix {
        fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
            let attrs = Attribute::parse_outer(input)?;
            while !input.is_empty() {
                let _: proc_macro2::TokenTree = input.parse()?;
            }
            Ok(Self(attrs))
        }
    }

    syn::parse2::<AttributePrefix>(value.to_token_stream())
        .map(|prefix| prefix.0)
        .unwrap_or_default()
}

fn reject_verbatim(file: &syn::File) -> Result<(), String> {
    let mut rejector = VerbatimRejector { error: None };
    rejector.visit_file(file);
    rejector.error.map_or(Ok(()), Err)
}

struct VerbatimRejector {
    error: Option<String>,
}

impl VerbatimRejector {
    fn reject(&mut self, kind: &str) {
        if self.error.is_none() {
            self.error = Some(format!(
                "unparsed {kind} survives as opaque Rust verbatim tokens"
            ));
        }
    }
}

impl<'ast> Visit<'ast> for VerbatimRejector {
    fn visit_item(&mut self, item: &'ast Item) {
        if matches!(item, Item::Verbatim(_)) {
            self.reject("item");
            return;
        }
        visit::visit_item(self, item);
    }

    fn visit_impl_item(&mut self, item: &'ast ImplItem) {
        if matches!(item, ImplItem::Verbatim(_)) {
            self.reject("implementation item");
            return;
        }
        visit::visit_impl_item(self, item);
    }

    fn visit_trait_item(&mut self, item: &'ast TraitItem) {
        if matches!(item, TraitItem::Verbatim(_)) {
            self.reject("trait item");
            return;
        }
        visit::visit_trait_item(self, item);
    }

    fn visit_foreign_item(&mut self, item: &'ast syn::ForeignItem) {
        if matches!(item, syn::ForeignItem::Verbatim(_)) {
            self.reject("foreign item");
            return;
        }
        visit::visit_foreign_item(self, item);
    }

    fn visit_expr(&mut self, expression: &'ast Expr) {
        if matches!(expression, Expr::Verbatim(_)) {
            self.reject("expression");
            return;
        }
        visit::visit_expr(self, expression);
    }

    fn visit_type(&mut self, ty: &'ast syn::Type) {
        if matches!(ty, syn::Type::Verbatim(_)) {
            self.reject("type");
            return;
        }
        visit::visit_type(self, ty);
    }

    fn visit_pat(&mut self, pattern: &'ast syn::Pat) {
        if matches!(pattern, syn::Pat::Verbatim(_)) {
            self.reject("pattern");
            return;
        }
        visit::visit_pat(self, pattern);
    }
}

fn collect_module_surface(
    unit: &ModuleUnit,
    items: &mut BTreeMap<String, IndexedItem>,
    imports: &mut Vec<RawImport>,
    calls: &mut Vec<RawCall>,
    macros: &mut Vec<RawMacro>,
) -> Result<(), String> {
    {
        let mut attribute_collector = AttributeCollector {
            unit,
            macros,
            ordinal: 0,
        };
        for item in &unit.items {
            attribute_collector.visit_item(item);
        }
    }
    let mut import_ordinal = 0;
    for item in &unit.items {
        match item {
            Item::Use(item_use) => {
                let mut leaves = Vec::new();
                flatten_use_tree(Vec::new(), &item_use.tree, &mut leaves);
                for leaf in leaves {
                    imports.push(RawImport {
                        module: unit.id.clone(),
                        module_path: unit.module_path.clone(),
                        source: leaf.source,
                        binding: leaf.binding,
                        glob: leaf.glob,
                        ordinal: import_ordinal,
                    });
                    import_ordinal += 1;
                }
            }
            Item::Fn(function) => {
                let id =
                    insert_named_item(unit, items, "fn", &function.sig.ident.to_string(), None)?;
                collect_body(unit, &id, None, &function.block, calls, macros);
            }
            Item::Const(constant) => {
                let id =
                    insert_named_item(unit, items, "const", &constant.ident.to_string(), None)?;
                collect_expression(unit, &id, None, &constant.expr, calls, macros);
            }
            Item::Static(static_item) => {
                let id =
                    insert_named_item(unit, items, "static", &static_item.ident.to_string(), None)?;
                collect_expression(unit, &id, None, &static_item.expr, calls, macros);
            }
            Item::Struct(item) => {
                insert_named_item(unit, items, "struct", &item.ident.to_string(), None)?;
            }
            Item::Enum(item) => {
                insert_named_item(unit, items, "enum", &item.ident.to_string(), None)?;
            }
            Item::Union(item) => {
                insert_named_item(unit, items, "union", &item.ident.to_string(), None)?;
            }
            Item::Type(item) => {
                insert_named_item(unit, items, "type", &item.ident.to_string(), None)?;
            }
            Item::TraitAlias(item) => {
                insert_named_item(unit, items, "trait_alias", &item.ident.to_string(), None)?;
            }
            Item::Trait(item_trait) => {
                let trait_name = item_trait.ident.to_string();
                insert_named_item(unit, items, "trait", &trait_name, None)?;
                for member in &item_trait.items {
                    match member {
                        TraitItem::Fn(function) => {
                            let id = insert_associated_item(
                                unit,
                                items,
                                "trait_fn",
                                &trait_name,
                                &format!("trait:{trait_name}"),
                                &function.sig.ident.to_string(),
                            )?;
                            if let Some(block) = &function.default {
                                collect_body(unit, &id, Some(&trait_name), block, calls, macros);
                            }
                        }
                        TraitItem::Const(constant) => {
                            insert_associated_item(
                                unit,
                                items,
                                "trait_const",
                                &trait_name,
                                &format!("trait:{trait_name}"),
                                &constant.ident.to_string(),
                            )?;
                        }
                        TraitItem::Type(associated_type) => {
                            insert_associated_item(
                                unit,
                                items,
                                "trait_type",
                                &trait_name,
                                &format!("trait:{trait_name}"),
                                &associated_type.ident.to_string(),
                            )?;
                        }
                        TraitItem::Macro(item_macro) => {
                            push_item_macro(
                                unit,
                                None,
                                item_macro.mac.path.to_token_stream().to_string(),
                                item_macro.mac.path.is_ident("macro_rules"),
                                None,
                                macros,
                            );
                        }
                        _ => {}
                    }
                }
            }
            Item::Impl(item_impl) => {
                collect_impl_surface(unit, item_impl, items, calls, macros)?;
            }
            Item::Macro(item_macro) => {
                let is_definition = item_macro.mac.path.is_ident("macro_rules");
                let name = item_macro
                    .ident
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| item_macro.mac.path.to_token_stream().to_string());
                if is_definition {
                    insert_named_item(unit, items, "macro", &name, None)?;
                }
                push_item_macro(
                    unit,
                    None,
                    item_macro.mac.path.to_token_stream().to_string(),
                    is_definition,
                    is_definition.then_some(name),
                    macros,
                );
            }
            Item::Mod(_) | Item::ExternCrate(_) | Item::ForeignMod(_) | Item::Verbatim(_) => {}
            _ => {}
        }
    }
    Ok(())
}

fn collect_impl_surface(
    unit: &ModuleUnit,
    item_impl: &syn::ItemImpl,
    items: &mut BTreeMap<String, IndexedItem>,
    calls: &mut Vec<RawCall>,
    macros: &mut Vec<RawMacro>,
) -> Result<(), String> {
    let owner = owner_name(&item_impl.self_ty);
    let mut header = item_impl.clone();
    header.items.clear();
    let header = normalized_tokens(&header);
    let scope = format!("impl:{}", short_hash(&header));
    for member in &item_impl.items {
        match member {
            ImplItem::Fn(function) => {
                let id = insert_associated_item(
                    unit,
                    items,
                    "impl_fn",
                    &owner,
                    &scope,
                    &function.sig.ident.to_string(),
                )?;
                collect_body(unit, &id, Some(&owner), &function.block, calls, macros);
            }
            ImplItem::Const(constant) => {
                let id = insert_associated_item(
                    unit,
                    items,
                    "impl_const",
                    &owner,
                    &scope,
                    &constant.ident.to_string(),
                )?;
                collect_expression(unit, &id, Some(&owner), &constant.expr, calls, macros);
            }
            ImplItem::Type(associated_type) => {
                insert_associated_item(
                    unit,
                    items,
                    "impl_type",
                    &owner,
                    &scope,
                    &associated_type.ident.to_string(),
                )?;
            }
            ImplItem::Macro(item_macro) => {
                push_item_macro(
                    unit,
                    None,
                    item_macro.mac.path.to_token_stream().to_string(),
                    item_macro.mac.path.is_ident("macro_rules"),
                    None,
                    macros,
                );
            }
            _ => {}
        }
    }
    Ok(())
}

fn insert_named_item(
    unit: &ModuleUnit,
    items: &mut BTreeMap<String, IndexedItem>,
    kind: &str,
    name: &str,
    owner: Option<String>,
) -> Result<String, String> {
    let id = format!("{}::{kind}:{name}", unit.id);
    insert_item(unit, items, id, kind, name, owner)
}

fn insert_associated_item(
    unit: &ModuleUnit,
    items: &mut BTreeMap<String, IndexedItem>,
    kind: &str,
    owner: &str,
    scope: &str,
    name: &str,
) -> Result<String, String> {
    let id = format!("{}::{scope}::{kind}:{name}", unit.id);
    insert_item(unit, items, id, kind, name, Some(owner.to_owned()))
}

fn insert_item(
    unit: &ModuleUnit,
    items: &mut BTreeMap<String, IndexedItem>,
    id: String,
    kind: &str,
    name: &str,
    owner: Option<String>,
) -> Result<String, String> {
    let indexed = IndexedItem {
        node: ItemNode {
            id: id.clone(),
            module: unit.id.clone(),
            kind: kind.to_owned(),
            name: name.to_owned(),
            owner,
        },
        module_path: unit.module_path.clone(),
    };
    if items.insert(id.clone(), indexed).is_some() {
        return Err(format!("duplicate item identity '{id}'"));
    }
    Ok(id)
}

fn owner_name(self_ty: &syn::Type) -> String {
    normalized_tokens(self_ty)
}

pub(crate) fn normalized_tokens(value: &impl ToTokens) -> String {
    value.to_token_stream().to_string()
}

pub(crate) fn short_hash(value: &str) -> String {
    let digest = Sha256::digest(value.as_bytes());
    format!("{digest:x}")[..16].to_owned()
}

fn flatten_use_tree(prefix: Vec<String>, tree: &UseTree, leaves: &mut Vec<UseLeaf>) {
    match tree {
        UseTree::Path(path) => {
            let mut prefix = prefix;
            prefix.push(path.ident.to_string());
            flatten_use_tree(prefix, &path.tree, leaves);
        }
        UseTree::Name(name) => {
            let mut source = prefix;
            let name = name.ident.to_string();
            let binding = if name == "self" {
                source.last().cloned()
            } else {
                source.push(name.clone());
                Some(name)
            };
            leaves.push(UseLeaf {
                source,
                binding,
                glob: false,
            });
        }
        UseTree::Rename(rename) => {
            let mut source = prefix;
            source.push(rename.ident.to_string());
            let binding = rename.rename.to_string();
            leaves.push(UseLeaf {
                source,
                binding: (binding != "_").then_some(binding),
                glob: false,
            });
        }
        UseTree::Glob(_) => leaves.push(UseLeaf {
            source: prefix,
            binding: None,
            glob: true,
        }),
        UseTree::Group(group) => {
            for tree in &group.items {
                flatten_use_tree(prefix.clone(), tree, leaves);
            }
        }
    }
}

fn collect_body(
    unit: &ModuleUnit,
    caller: &str,
    owner: Option<&str>,
    block: &Block,
    calls: &mut Vec<RawCall>,
    macros: &mut Vec<RawMacro>,
) {
    let mut collector = BodyCollector::new(unit, caller, owner, calls, macros);
    collector.visit_block(block);
}

fn collect_expression(
    unit: &ModuleUnit,
    caller: &str,
    owner: Option<&str>,
    expression: &Expr,
    calls: &mut Vec<RawCall>,
    macros: &mut Vec<RawMacro>,
) {
    let mut collector = BodyCollector::new(unit, caller, owner, calls, macros);
    collector.visit_expr(expression);
}

struct BodyCollector<'a> {
    unit: &'a ModuleUnit,
    caller: &'a str,
    owner: Option<&'a str>,
    calls: &'a mut Vec<RawCall>,
    macros: &'a mut Vec<RawMacro>,
    call_ordinal: usize,
    macro_ordinal: usize,
}

impl<'a> BodyCollector<'a> {
    fn new(
        unit: &'a ModuleUnit,
        caller: &'a str,
        owner: Option<&'a str>,
        calls: &'a mut Vec<RawCall>,
        macros: &'a mut Vec<RawMacro>,
    ) -> Self {
        Self {
            unit,
            caller,
            owner,
            calls,
            macros,
            call_ordinal: 0,
            macro_ordinal: 0,
        }
    }

    fn push_macro(&mut self, name: String, kind: &str) {
        self.macros.push(RawMacro {
            module: self.unit.id.clone(),
            containing_item: Some(self.caller.to_owned()),
            name,
            kind: kind.to_owned(),
            ordinal: self.macro_ordinal,
        });
        self.macro_ordinal += 1;
    }
}

impl<'ast> Visit<'ast> for BodyCollector<'_> {
    fn visit_expr_call(&mut self, expression: &'ast syn::ExprCall) {
        let (syntax, kind) = callable_path(&expression.func);
        self.calls.push(RawCall {
            caller: self.caller.to_owned(),
            module: self.unit.id.clone(),
            module_path: self.unit.module_path.clone(),
            owner: self.owner.map(ToOwned::to_owned),
            syntax,
            kind,
            ordinal: self.call_ordinal,
        });
        self.call_ordinal += 1;
        visit::visit_expr_call(self, expression);
    }

    fn visit_expr_method_call(&mut self, expression: &'ast syn::ExprMethodCall) {
        let name = expression.method.to_string();
        self.calls.push(RawCall {
            caller: self.caller.to_owned(),
            module: self.unit.id.clone(),
            module_path: self.unit.module_path.clone(),
            owner: self.owner.map(ToOwned::to_owned),
            syntax: format!(".{name}"),
            kind: RawCallKind::Method(name),
            ordinal: self.call_ordinal,
        });
        self.call_ordinal += 1;
        visit::visit_expr_method_call(self, expression);
    }

    fn visit_macro(&mut self, invocation: &'ast syn::Macro) {
        self.push_macro(invocation.path.to_token_stream().to_string(), "invocation");
        visit::visit_macro(self, invocation);
    }

    fn visit_item_macro(&mut self, item_macro: &'ast syn::ItemMacro) {
        if item_macro.mac.path.is_ident("macro_rules") {
            let name = item_macro
                .ident
                .as_ref()
                .map(ToString::to_string)
                .unwrap_or_else(|| "<anonymous>".to_owned());
            self.push_macro(name, "definition");
        } else {
            self.push_macro(
                item_macro.mac.path.to_token_stream().to_string(),
                "invocation",
            );
        }
    }
}

struct AttributeCollector<'a> {
    unit: &'a ModuleUnit,
    macros: &'a mut Vec<RawMacro>,
    ordinal: usize,
}

impl<'ast> Visit<'ast> for AttributeCollector<'_> {
    fn visit_attribute(&mut self, attribute: &'ast Attribute) {
        self.macros.push(RawMacro {
            module: self.unit.id.clone(),
            containing_item: None,
            name: attribute.path().to_token_stream().to_string(),
            kind: "attribute".to_owned(),
            ordinal: self.ordinal,
        });
        self.ordinal += 1;
        visit::visit_attribute(self, attribute);
    }
}

fn callable_path(expression: &Expr) -> (String, RawCallKind) {
    match expression {
        Expr::Path(path) => {
            let segments: Vec<_> = path
                .path
                .segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .collect();
            (segments.join("::"), RawCallKind::Path(segments))
        }
        Expr::Group(group) => callable_path(&group.expr),
        Expr::Paren(parenthesized) => callable_path(&parenthesized.expr),
        _ => (
            normalized_tokens(expression),
            RawCallKind::CallableExpression,
        ),
    }
}

fn push_item_macro(
    unit: &ModuleUnit,
    containing_item: Option<&str>,
    path: String,
    is_definition: bool,
    definition_name: Option<String>,
    macros: &mut Vec<RawMacro>,
) {
    macros.push(RawMacro {
        module: unit.id.clone(),
        containing_item: containing_item.map(ToOwned::to_owned),
        name: definition_name.unwrap_or(path),
        kind: if is_definition {
            "definition".to_owned()
        } else {
            "invocation".to_owned()
        },
        ordinal: macros.len(),
    });
}

fn resolve_imports(
    raw_imports: &[RawImport],
    modules: &BTreeMap<String, ModuleUnit>,
    items: &BTreeMap<String, IndexedItem>,
    contexts: &BTreeMap<String, &TargetContext>,
) -> Result<Vec<ImportEdge>, String> {
    let mut raw_imports = raw_imports.to_vec();
    raw_imports
        .sort_by(|left, right| (&left.module, left.ordinal).cmp(&(&right.module, right.ordinal)));
    let mut binding_sources: BTreeMap<(String, String), String> = BTreeMap::new();
    for raw in &raw_imports {
        if let Some(binding) = &raw.binding {
            let source_path = raw.source.join("::");
            let key = (raw.module.clone(), binding.clone());
            if let Some(previous) = binding_sources.insert(key, source_path.clone())
                && previous != source_path
            {
                return Err(format!(
                    "ambiguous import alias '{binding}' in '{}' binds both '{previous}' and '{source_path}'",
                    raw.module
                ));
            }
        }
    }
    let mut bindings: BTreeMap<(String, String), (String, Vec<String>)> = BTreeMap::new();
    let mut globs: BTreeMap<String, Vec<(String, Vec<String>)>> = BTreeMap::new();
    let mut resolved = Vec::new();
    let mut pending = raw_imports;
    while !pending.is_empty() {
        let mut deferred = Vec::new();
        let mut progress = false;
        for raw in pending {
            let unit = &modules[&raw.module];
            let context = contexts[&unit.target];
            let source_path = raw.source.join("::");
            let direct = resolve_local_path(
                &raw.source,
                &raw.module_path,
                &unit.target,
                modules,
                items,
                context,
            );
            let path_resolution = match direct {
                Ok(PathResolution::Local(targets)) => Some(PathResolution::Local(targets)),
                Ok(PathResolution::External) => Some(PathResolution::External),
                Ok(PathResolution::Compiler) | Err(_) => {
                    resolve_import_reexport(&raw, unit, modules, items, context, &bindings, &globs)
                }
            };
            let Some(path_resolution) = path_resolution else {
                deferred.push(raw);
                continue;
            };
            let (resolution, mut targets) = match path_resolution {
                PathResolution::Local(targets) => ("local".to_owned(), targets),
                PathResolution::External => (
                    "external".to_owned(),
                    vec![format!("external:{source_path}")],
                ),
                PathResolution::Compiler => {
                    deferred.push(raw);
                    continue;
                }
            };
            targets.sort();
            targets.dedup();
            let binding_value = (resolution.clone(), targets.clone());
            if let Some(binding) = &raw.binding {
                bindings.insert((raw.module.clone(), binding.clone()), binding_value.clone());
            }
            if raw.glob {
                globs
                    .entry(raw.module.clone())
                    .or_default()
                    .push(binding_value);
            }
            resolved.push(ImportEdge {
                id: format!("{}::use:{}", raw.module, raw.ordinal),
                module: raw.module,
                source_path,
                binding: raw.binding,
                glob: raw.glob,
                resolution,
                targets,
            });
            progress = true;
        }
        if !progress {
            let first = deferred
                .first()
                .expect("pending imports are non-empty when no progress occurs");
            return Err(format!(
                "import '{}' in '{}' cannot be resolved through modules or aliases",
                first.source.join("::"),
                first.module
            ));
        }
        pending = deferred;
    }
    resolved.sort_by(|left, right| left.id.cmp(&right.id));
    Ok(resolved)
}

#[allow(clippy::too_many_arguments)]
fn resolve_import_reexport(
    raw: &RawImport,
    unit: &ModuleUnit,
    modules: &BTreeMap<String, ModuleUnit>,
    items: &BTreeMap<String, IndexedItem>,
    context: &TargetContext,
    bindings: &BTreeMap<(String, String), (String, Vec<String>)>,
    globs: &BTreeMap<String, Vec<(String, Vec<String>)>>,
) -> Option<PathResolution> {
    let canonical = canonical_import_path(&raw.source, &raw.module_path)?;
    for prefix_len in (1..canonical.len()).rev() {
        let prefix = &canonical[..prefix_len];
        let module = module_id(&unit.target, prefix);
        if !modules.contains_key(&module) {
            continue;
        }
        let binding = &canonical[prefix_len];
        if let Some((resolution, targets)) = bindings.get(&(module.clone(), binding.clone())) {
            if resolution == "external" {
                return Some(PathResolution::External);
            }
            let name = canonical
                .last()
                .expect("canonical import path is non-empty");
            let mut candidates = named_callable_candidates(items, &unit.target, name);
            if candidates.is_empty() {
                candidates = targets.clone();
            }
            return Some(PathResolution::Local(candidates));
        }
        if let Some(glob_bindings) = globs.get(&module) {
            if glob_bindings
                .iter()
                .any(|(resolution, _)| resolution == "external")
            {
                return Some(PathResolution::External);
            }
            let mut candidates = glob_bindings
                .iter()
                .flat_map(|(_, targets)| targets.iter().cloned())
                .collect::<Vec<_>>();
            candidates.sort();
            candidates.dedup();
            if !candidates.is_empty() {
                return Some(PathResolution::Local(candidates));
            }
        }
        return None;
    }
    if context.external_crates.contains(&raw.source[0]) {
        Some(PathResolution::External)
    } else {
        None
    }
}

fn canonical_import_path(source: &[String], current_module: &[String]) -> Option<Vec<String>> {
    if source.is_empty() {
        return None;
    }
    let mut index = 0;
    let mut base = if source[0] == "crate" {
        index = 1;
        vec!["crate".to_owned()]
    } else if source[0] == "self" {
        index = 1;
        current_module.to_vec()
    } else if source[0] == "super" {
        let mut base = current_module.to_vec();
        while index < source.len() && source[index] == "super" {
            if base.len() <= 1 {
                return None;
            }
            base.pop();
            index += 1;
        }
        base
    } else {
        vec!["crate".to_owned()]
    };
    base.extend(source[index..].iter().cloned());
    Some(base)
}

enum PathResolution {
    Local(Vec<String>),
    External,
    Compiler,
}

#[allow(clippy::too_many_arguments)]
fn resolve_local_path(
    source: &[String],
    current_module: &[String],
    target: &str,
    modules: &BTreeMap<String, ModuleUnit>,
    items: &BTreeMap<String, IndexedItem>,
    context: &TargetContext,
) -> Result<PathResolution, String> {
    if source.is_empty() {
        return Err("empty Rust path cannot be resolved".to_owned());
    }
    if context.external_crates.contains(&source[0]) {
        return Ok(PathResolution::External);
    }
    let mut index = 0;
    let mut base = if source[0] == "crate" {
        index = 1;
        vec!["crate".to_owned()]
    } else if source[0] == "self" {
        index = 1;
        current_module.to_vec()
    } else if source[0] == "super" {
        let mut base = current_module.to_vec();
        while index < source.len() && source[index] == "super" {
            if base.len() <= 1 {
                return Err("Rust path uses super above the crate root".to_owned());
            }
            base.pop();
            index += 1;
        }
        base
    } else if source[0] == "Self" {
        return Ok(PathResolution::Compiler);
    } else {
        current_module.to_vec()
    };
    base.extend(source[index..].iter().cloned());
    let mut targets = local_targets(&base, target, modules, items);
    if targets.is_empty() && source[0] != "crate" {
        let mut root_path = vec!["crate".to_owned()];
        root_path.extend(source.iter().cloned());
        targets = local_targets(&root_path, target, modules, items);
    }
    if targets.is_empty() {
        if matches!(source[0].as_str(), "crate" | "self" | "super") {
            return Err(format!(
                "explicit local path '{}' has no module or item target",
                source.join("::")
            ));
        }
        Ok(PathResolution::Compiler)
    } else {
        Ok(PathResolution::Local(targets))
    }
}

fn local_targets(
    canonical_path: &[String],
    target: &str,
    modules: &BTreeMap<String, ModuleUnit>,
    items: &BTreeMap<String, IndexedItem>,
) -> Vec<String> {
    let mut targets = Vec::new();
    let module = module_id(target, canonical_path);
    if modules.contains_key(&module) {
        targets.push(module);
    }
    if canonical_path.len() > 1 {
        let item_name = canonical_path.last().expect("non-empty canonical path");
        let item_module = &canonical_path[..canonical_path.len() - 1];
        targets.extend(
            items
                .values()
                .filter(|item| {
                    item.node.id.starts_with(&format!("{target}::"))
                        && item.module_path == item_module
                        && item.node.name == *item_name
                })
                .map(|item| item.node.id.clone()),
        );
    }
    targets.sort();
    targets.dedup();
    targets
}

fn resolve_calls(
    raw_calls: &[RawCall],
    modules: &BTreeMap<String, ModuleUnit>,
    items: &BTreeMap<String, IndexedItem>,
    imports: &[ImportEdge],
    contexts: &BTreeMap<String, &TargetContext>,
) -> Result<Vec<CallEdge>, String> {
    let import_bindings: BTreeMap<_, _> = imports
        .iter()
        .filter_map(|edge| {
            edge.binding
                .as_ref()
                .map(|binding| ((edge.module.clone(), binding.clone()), edge))
        })
        .collect();
    let mut raw_calls = raw_calls.to_vec();
    raw_calls
        .sort_by(|left, right| (&left.caller, left.ordinal).cmp(&(&right.caller, right.ordinal)));
    let mut calls = Vec::new();
    for raw in raw_calls {
        let unit = &modules[&raw.module];
        let context = contexts[&unit.target];
        let (resolution, mut targets) = match &raw.kind {
            RawCallKind::CallableExpression => {
                ("compiler_callable_expression".to_owned(), Vec::new())
            }
            RawCallKind::Method(name) => (
                "compiler_method_dispatch".to_owned(),
                named_callable_candidates(items, &unit.target, name),
            ),
            RawCallKind::Path(path) => resolve_call_path(
                path,
                &raw,
                unit,
                context,
                modules,
                items,
                &import_bindings,
                imports,
            )?,
        };
        targets.sort();
        targets.dedup();
        calls.push(CallEdge {
            id: format!("{}::call:{}", raw.caller, raw.ordinal),
            caller: raw.caller,
            syntax: raw.syntax,
            resolution,
            targets,
        });
    }
    Ok(calls)
}

#[allow(clippy::too_many_arguments)]
fn resolve_call_path(
    path: &[String],
    raw: &RawCall,
    unit: &ModuleUnit,
    context: &TargetContext,
    modules: &BTreeMap<String, ModuleUnit>,
    items: &BTreeMap<String, IndexedItem>,
    import_bindings: &BTreeMap<(String, String), &ImportEdge>,
    imports: &[ImportEdge],
) -> Result<(String, Vec<String>), String> {
    if path.is_empty() {
        return Err(format!("call '{}' has an empty path", raw.syntax));
    }
    if context.external_crates.contains(&path[0]) {
        return Ok(("external".to_owned(), Vec::new()));
    }
    if path[0] == "Self" {
        let Some(owner) = &raw.owner else {
            return Ok(("compiler_self_type".to_owned(), Vec::new()));
        };
        let name = path.last().expect("non-empty call path");
        let candidates = owner_callable_candidates(items, &unit.target, owner, name);
        return Ok(("compiler_self_type".to_owned(), candidates));
    }
    if let Some(import) = import_bindings.get(&(raw.module.clone(), path[0].clone())) {
        if import.resolution == "external" {
            return Ok(("external_import".to_owned(), Vec::new()));
        }
        if path.len() == 1 {
            return Ok(("exact_import".to_owned(), import.targets.clone()));
        }
        let name = path.last().expect("non-empty call path");
        let mut candidates = named_callable_candidates(items, &unit.target, name);
        candidates.retain(|candidate| {
            import
                .targets
                .iter()
                .any(|target| candidate.contains(target.rsplit("::").next().unwrap_or_default()))
        });
        if candidates.is_empty() {
            candidates = named_callable_candidates(items, &unit.target, name);
        }
        return Ok(("compiler_associated_import".to_owned(), candidates));
    }
    let explicit_local = matches!(path[0].as_str(), "crate" | "self" | "super");
    let local_resolution = resolve_local_path(
        path,
        &raw.module_path,
        &unit.target,
        modules,
        items,
        context,
    );
    match local_resolution {
        Err(error) => {
            if let Some(resolution) = resolve_call_reexport(path, raw, unit, imports) {
                return Ok(resolution);
            }
            if let Some(resolution) = resolve_associated_local_call(path, raw, unit, modules, items)
            {
                return Ok(resolution);
            }
            Err(error)
        }
        Ok(PathResolution::Local(targets)) => {
            let callable: Vec<_> = targets
                .into_iter()
                .filter(|target| !modules.contains_key(target))
                .collect();
            if callable.is_empty() {
                let name = path.last().expect("non-empty call path");
                Ok((
                    "compiler_associated_local".to_owned(),
                    named_callable_candidates(items, &unit.target, name),
                ))
            } else {
                Ok(("exact_local".to_owned(), callable))
            }
        }
        Ok(PathResolution::External) => Ok(("external".to_owned(), Vec::new())),
        Ok(PathResolution::Compiler) if explicit_local => {
            if let Some(resolution) = resolve_call_reexport(path, raw, unit, imports) {
                return Ok(resolution);
            }
            Err(format!(
                "explicit local call path '{}' cannot be resolved",
                path.join("::")
            ))
        }
        Ok(PathResolution::Compiler) => {
            let name = path.last().expect("non-empty call path");
            let candidates = named_callable_candidates(items, &unit.target, name);
            if candidates.is_empty() {
                Ok(("compiler_prelude_or_binding".to_owned(), Vec::new()))
            } else {
                Ok(("conservative_local".to_owned(), candidates))
            }
        }
    }
}

fn resolve_call_reexport(
    path: &[String],
    raw: &RawCall,
    unit: &ModuleUnit,
    imports: &[ImportEdge],
) -> Option<(String, Vec<String>)> {
    let canonical = canonical_import_path(path, &raw.module_path)?;
    for prefix_len in (1..canonical.len()).rev() {
        let module = module_id(&unit.target, &canonical[..prefix_len]);
        let binding = &canonical[prefix_len];
        let exact = imports
            .iter()
            .find(|import| import.module == module && import.binding.as_deref() == Some(binding));
        if let Some(import) = exact {
            return Some(if import.resolution == "external" {
                ("external_reexport".to_owned(), Vec::new())
            } else {
                ("compiler_local_reexport".to_owned(), import.targets.clone())
            });
        }
        let globs: Vec<_> = imports
            .iter()
            .filter(|import| import.module == module && import.glob)
            .collect();
        if globs.iter().any(|import| import.resolution == "external") {
            return Some(("external_glob_reexport".to_owned(), Vec::new()));
        }
        if !globs.is_empty() {
            let mut targets = globs
                .iter()
                .flat_map(|import| import.targets.iter().cloned())
                .collect::<Vec<_>>();
            targets.sort();
            targets.dedup();
            return Some(("compiler_local_glob_reexport".to_owned(), targets));
        }
    }
    None
}

fn resolve_associated_local_call(
    path: &[String],
    raw: &RawCall,
    unit: &ModuleUnit,
    modules: &BTreeMap<String, ModuleUnit>,
    items: &BTreeMap<String, IndexedItem>,
) -> Option<(String, Vec<String>)> {
    let canonical = canonical_import_path(path, &raw.module_path)?;
    for prefix_len in (1..canonical.len()).rev() {
        let module = module_id(&unit.target, &canonical[..prefix_len]);
        if !modules.contains_key(&module) {
            continue;
        }
        let name = canonical.last()?;
        let owner = canonical
            .get(canonical.len().saturating_sub(2))
            .map(String::as_str);
        let mut targets = owner
            .map(|owner| owner_callable_candidates(items, &unit.target, owner, name))
            .unwrap_or_default();
        if targets.is_empty() {
            targets = named_callable_candidates(items, &unit.target, name);
        }
        return Some(("compiler_associated_local".to_owned(), targets));
    }
    None
}

fn named_callable_candidates(
    items: &BTreeMap<String, IndexedItem>,
    target: &str,
    name: &str,
) -> Vec<String> {
    items
        .values()
        .filter(|item| {
            item.node.id.starts_with(&format!("{target}::"))
                && item.node.name == name
                && matches!(
                    item.node.kind.as_str(),
                    "fn" | "impl_fn" | "trait_fn" | "const" | "static"
                )
        })
        .map(|item| item.node.id.clone())
        .collect()
}

fn owner_callable_candidates(
    items: &BTreeMap<String, IndexedItem>,
    target: &str,
    owner: &str,
    name: &str,
) -> Vec<String> {
    items
        .values()
        .filter(|item| {
            item.node.id.starts_with(&format!("{target}::"))
                && item.node.name == name
                && item.node.owner.as_deref() == Some(owner)
                && matches!(item.node.kind.as_str(), "impl_fn" | "trait_fn")
        })
        .map(|item| item.node.id.clone())
        .collect()
}

fn resolve_macros(
    raw_macros: &[RawMacro],
    imports: &[ImportEdge],
    contexts: &BTreeMap<String, &TargetContext>,
) -> Result<Vec<MacroNode>, String> {
    let mut raw_macros = raw_macros.to_vec();
    raw_macros.sort_by(|left, right| {
        (
            &left.module,
            &left.containing_item,
            &left.kind,
            left.ordinal,
            &left.name,
        )
            .cmp(&(
                &right.module,
                &right.containing_item,
                &right.kind,
                right.ordinal,
                &right.name,
            ))
    });
    let mut definitions: BTreeMap<(String, Option<String>, String), String> = BTreeMap::new();
    let mut nodes = Vec::new();
    for raw in &raw_macros {
        if raw.kind != "definition" {
            continue;
        }
        let id = macro_id(raw);
        let key = (
            raw.module.clone(),
            raw.containing_item.clone(),
            raw.name.clone(),
        );
        if definitions.insert(key, id.clone()).is_some() {
            return Err(format!(
                "duplicate local macro definition '{}' in '{}'",
                raw.name, raw.module
            ));
        }
        nodes.push(MacroNode {
            id: id.clone(),
            module: raw.module.clone(),
            containing_item: raw.containing_item.clone(),
            name: raw.name.clone(),
            kind: raw.kind.clone(),
            resolution: "local_definition".to_owned(),
            targets: vec![id],
        });
    }
    for raw in raw_macros.iter().filter(|raw| raw.kind != "definition") {
        let id = macro_id(raw);
        let (resolution, targets) = if raw.kind == "attribute" {
            ("compiler_attribute".to_owned(), Vec::new())
        } else {
            let bare_name = raw
                .name
                .split("::")
                .last()
                .unwrap_or(&raw.name)
                .trim()
                .to_owned();
            let local = definitions
                .get(&(
                    raw.module.clone(),
                    raw.containing_item.clone(),
                    bare_name.clone(),
                ))
                .or_else(|| definitions.get(&(raw.module.clone(), None, bare_name.clone())))
                .cloned();
            if let Some(target) = local {
                ("local".to_owned(), vec![target])
            } else if imports.iter().any(|import| {
                import.module == raw.module
                    && import.binding.as_deref() == Some(&bare_name)
                    && import.resolution == "local"
            }) {
                ("local_import".to_owned(), Vec::new())
            } else {
                let target = raw.module.split("::crate").next().unwrap_or_default();
                if !contexts.contains_key(target) {
                    return Err(format!(
                        "macro '{}' belongs to unknown target '{target}'",
                        raw.name
                    ));
                }
                ("external_or_builtin".to_owned(), Vec::new())
            }
        };
        nodes.push(MacroNode {
            id,
            module: raw.module.clone(),
            containing_item: raw.containing_item.clone(),
            name: raw.name.clone(),
            kind: raw.kind.clone(),
            resolution,
            targets,
        });
    }
    nodes.sort_by(|left, right| left.id.cmp(&right.id));
    let mut seen = BTreeSet::new();
    for node in &nodes {
        if !seen.insert(node.id.clone()) {
            return Err(format!("duplicate macro identity '{}'", node.id));
        }
    }
    Ok(nodes)
}

fn macro_id(raw: &RawMacro) -> String {
    let scope = raw
        .containing_item
        .as_deref()
        .map(short_hash)
        .unwrap_or_else(|| "module".to_owned());
    format!(
        "{}::{scope}::macro:{}:{}:{}",
        raw.module, raw.kind, raw.ordinal, raw.name
    )
}
