#!/usr/bin/env perl
use strict;
use warnings;
use utf8;

use Cwd qw(abs_path);
use File::Basename qw(dirname);
use File::Find;
use File::Spec;
use File::Temp qw(tempfile);
use FindBin qw($Bin);
use JSON::PP;

binmode STDOUT, ':encoding(UTF-8)';
binmode STDERR, ':encoding(UTF-8)';

my $root = abs_path(File::Spec->catdir($Bin, '..'));
my $mode = 'check';
my $legacy_root;
my @external_source_roots;
my $execute = 0;

while (@ARGV) {
    my $arg = shift @ARGV;
    if ($arg eq '--check') {
        $mode = 'check';
    } elsif ($arg eq '--report') {
        $mode = 'report';
    } elsif ($arg eq '--self-test') {
        $mode = 'self-test';
    } elsif ($arg eq '--root') {
        die "persisted-artifact-paths: --root requires a path\n" if !@ARGV;
        $root = abs_path(shift @ARGV);
        die "persisted-artifact-paths: root does not exist\n" if !defined $root;
    } elsif ($arg eq '--migrate-legacy-root') {
        die "persisted-artifact-paths: --migrate-legacy-root requires a path\n" if !@ARGV;
        $legacy_root = shift @ARGV;
        $mode = 'migrate';
    } elsif ($arg eq '--external-source-root') {
        die "persisted-artifact-paths: --external-source-root requires a path\n" if !@ARGV;
        push @external_source_roots, shift @ARGV;
    } elsif ($arg eq '--execute') {
        $execute = 1;
    } else {
        die usage();
    }
}

die "persisted-artifact-paths: --execute requires --migrate-legacy-root\n"
    if $execute && $mode ne 'migrate';

if ($mode eq 'self-test') {
    run_self_test();
    print "persisted-artifact-paths: self-test 12/12 passed.\n";
    exit 0;
}

my @errors;
validate_producer_contract($root, \@errors);

if ($mode eq 'migrate') {
    my @artifacts = canonical_artifacts($root, \@errors);
    my $migration = migrate_legacy_root(
        $root,
        \@artifacts,
        $legacy_root,
        \@external_source_roots,
        $execute,
        \@errors,
    );
    if (@errors) {
        print STDERR "persisted-artifact-paths: $_\n" for @errors;
        exit 1;
    }
    print join(
        ' ',
        'persisted-artifact-paths:',
        $execute ? 'migration-executed' : 'migration-dry-run',
        "artifact_files=$migration->{artifact_files}",
        "changed_files=$migration->{changed_files}",
        "changed_values=$migration->{changed_values}",
        "origin_labels_added=$migration->{origin_labels_added}",
        "net_removed_bytes=$migration->{net_removed_bytes}",
    ), "\n";
    exit 0;
}

my @artifacts = persisted_json_artifacts($root, \@errors);
my $stats = scan_artifacts($root, \@artifacts, \@errors);
if (@errors) {
    print STDERR "persisted-artifact-paths: $_\n" for @errors;
    exit 1;
}

if ($mode eq 'report') {
    print JSON::PP->new->canonical->encode({
        schema_version => 1,
        artifact_files => $stats->{artifact_files},
        path_values => $stats->{path_values},
        authorized_external_paths => $stats->{authorized_external_paths},
        absolute_repository_paths => 0,
        producer_files => $stats->{producer_files},
    }), "\n";
} else {
    print join(
        ' ',
        'persisted-artifact-paths: PASS —',
        "$stats->{artifact_files} persisted JSON artifacts,",
        "$stats->{path_values} path values,",
        "$stats->{authorized_external_paths} authorized external absolute paths,",
        'zero repository-owned absolute paths',
    ), "\n";
}

sub usage {
    return <<'USAGE';
Usage: scripts/check_persisted_artifact_paths.pl [--check|--report|--self-test]
       scripts/check_persisted_artifact_paths.pl --migrate-legacy-root ABSOLUTE_PATH
           --external-source-root ABSOLUTE_PATH [--external-source-root ...] [--execute]
USAGE
}

sub validate_producer_contract {
    my ($base, $errors) = @_;
    my %required = (
        'crates/specforge/src/persisted_path.rs' => [
            'pub(crate) fn resolve_existing(',
            'pub(crate) fn normalize_for_storage(',
            'pub(crate) fn resolve_repository_output(',
        ],
        'crates/specforge/src/ir/source.rs' => [
            'to_string_pretty(&self.persisted_clone()?)',
            'source_ir.runtime_clone()',
            'persisted.artifact_layout.normalize_for_storage()?;',
            'resolve_reference(&runtime.source.canonical_path, source_origin)?',
        ],
        'crates/specforge/src/ir/evidence.rs' => [
            'to_string_pretty(&self.persisted_clone()?)',
            'evidence_ir.runtime_clone()',
            'persisted.artifact_layout.normalize_for_storage()?;',
            'anchor.source_path = resolve_reference(&anchor.source_path, source_path_origin)?;',
        ],
        'crates/specforge/src/ir/figure_region.rs' => [
            'serialize_with = "serialize_repository_image_path"',
            'normalize_for_storage(path, PersistedPathOrigin::RepositoryOwned)',
            'resolve_reference(&path, PersistedPathOrigin::RepositoryOwned)',
        ],
        'crates/specforge/src/ir/semantic.rs' => [
            'to_string_pretty(&self.persisted_clone()?)',
            'semantic_ir.runtime_clone()',
            'persisted.artifact_layout.normalize_for_storage()?;',
        ],
        'crates/specforge/src/ir/intent.rs' => [
            'to_string_pretty(&self.persisted_clone()?)',
            'intent_ir.runtime_clone()',
            'persisted.artifact_layout.normalize_for_storage()?;',
        ],
        'crates/specforge/src/ir/adapters.rs' => [
            'to_string_pretty(&self.persisted_clone()?)',
            'artifact.runtime_clone()',
            'persisted.artifact_layout.normalize_for_storage()?;',
        ],
        'crates/specforge/src/ir/prior_memory.rs' => [
            'to_string_pretty(&self.persisted_clone()?)',
            'normalize_for_storage(&source.artifact_path, PersistedPathOrigin::RepositoryOwned)',
        ],
        'crates/specforge/src/commands/learn_priors.rs' => [
            'let pretty_json = corpus_memory.to_pretty_json()?;',
        ],
        'crates/specforge/src/commands/project_validation.rs' => [
            'resolve_existing(artifact, PersistedPathOrigin::RepositoryOwned)?',
            'artifact_path: repo_relative_display(&snapshot.artifact_path, repo_root)',
            'path: repo_relative_display(&input.path, repo_root)',
            'working_directory: ".".to_string()',
        ],
        'crates/specforge/src/commands/recover_register_bits.rs' => [
            'resolve_existing(&args.evidence_ir, PersistedPathOrigin::RepositoryOwned)?',
        ],
        'crates/specforge/src/ir/source/docling_backend.rs' => [
            'normalize_backend_metadata_paths(',
            'normalize_for_storage(source_path, source_path_origin)?',
            'normalize_for_storage(promoted_markdown_path, PersistedPathOrigin::RepositoryOwned)?',
        ],
    );

    for my $relative (sort keys %required) {
        my $path = File::Spec->catfile($base, split m{/}, $relative);
        if (!-f $path || -l $path) {
            push @$errors, "producer contract file is missing, unsafe, or a symlink: $relative";
            next;
        }
        my $raw = read_raw($path, $errors, $relative);
        next if !defined $raw;
        for my $needle (@{$required{$relative}}) {
            push @$errors, "producer contract '$relative' is missing required seam: $needle"
                if index($raw, $needle) < 0;
        }
    }
}

sub canonical_artifacts {
    my ($base, $errors) = @_;
    my $generated = File::Spec->catdir($base, 'generated');
    return () if !-e $generated;
    if (!-d $generated || -l $generated) {
        push @$errors, 'generated root is not a real in-repository directory';
        return ();
    }

    my @artifacts;
    find({
        no_chdir => 1,
        wanted => sub {
            my $path = $File::Find::name;
            return if -d $path;
            my $relative = File::Spec->abs2rel($path, $base);
            $relative =~ tr{\\}{/};
            return if !defined artifact_kind($relative);
            if (-l $path || !-f $path) {
                push @$errors, "canonical artifact is not a real file: $relative";
                return;
            }
            push @artifacts, [$relative, $path, artifact_kind($relative)];
        },
    }, $generated);
    return sort { $a->[0] cmp $b->[0] } @artifacts;
}

sub persisted_json_artifacts {
    my ($base, $errors) = @_;
    my $generated = File::Spec->catdir($base, 'generated');
    return () if !-e $generated;
    if (!-d $generated || -l $generated) {
        push @$errors, 'generated root is not a real in-repository directory';
        return ();
    }

    my @artifacts;
    find({
        no_chdir => 1,
        wanted => sub {
            my $path = $File::Find::name;
            return if -d $path;
            return if $path !~ /\.json\z/;
            my $relative = File::Spec->abs2rel($path, $base);
            $relative =~ tr{\\}{/};
            if (-l $path || !-f $path) {
                push @$errors, "persisted JSON artifact is not a real file: $relative";
                return;
            }
            push @artifacts, [$relative, $path, artifact_kind($relative) // 'other'];
        },
    }, $generated);
    return sort { $a->[0] cmp $b->[0] } @artifacts;
}

sub artifact_kind {
    my ($relative) = @_;
    return 'source' if $relative =~ m{\Agenerated/source_ir/[^/]+/source_ir\.json\z};
    return 'source_metadata'
        if $relative =~ m{\Agenerated/source_ir/[^/]+/normalized/[^/]+\.meta\.json\z};
    return 'evidence' if $relative =~ m{\Agenerated/evidence_ir/[^/]+/evidence_ir\.json\z};
    return 'semantic' if $relative =~ m{\Agenerated/semantic_ir/[^/]+/semantic_ir\.json\z};
    return 'intent' if $relative =~ m{\Agenerated/intent_ir/[^/]+/intent_ir\.json\z};
    return 'adapter' if $relative =~ m{\Agenerated/adapters/isf/[^/]+/adapter\.json\z};
    return 'prior_memory' if $relative eq 'generated/prior_memory/corpus_memory.json';
    return;
}

sub scan_artifacts {
    my ($base, $artifacts, $errors) = @_;
    my $stats = {
        artifact_files => scalar(@$artifacts),
        path_values => 0,
        authorized_external_paths => 0,
        absolute_repository_paths => 0,
        producer_files => 12,
    };

    for my $artifact (@$artifacts) {
        my ($relative, $path, $kind) = @$artifact;
        my $meta = artifact_metadata($path, $errors, $relative);
        next if !defined $meta;
        open my $fh, '<:raw', $path or do {
            push @$errors, "cannot read canonical artifact '$relative': $!";
            next;
        };
        while (my $line = <$fh>) {
            while ($line =~ /"([A-Za-z][A-Za-z0-9_]*)"\s*:\s*"([^"]*)"/g) {
                my $key = $1;
                next if !is_path_key($key);
                my $value = decode_json_string($2, $errors, "$relative value");
                next if !defined $value;
                $stats->{path_values}++;
                next if !is_absolute_path($value);
                if (is_authorized_external($kind, $key, $meta)) {
                    $stats->{authorized_external_paths}++;
                    next;
                }
                $stats->{absolute_repository_paths}++;
                push @$errors, "canonical artifact '$relative' stores absolute $key: $value"
                    if $stats->{absolute_repository_paths} <= 20;
            }
        }
        close $fh or push @$errors, "cannot close canonical artifact '$relative': $!";
    }
    push @$errors,
        "$stats->{absolute_repository_paths} repository-owned absolute path value(s) remain across canonical artifacts"
        if $stats->{absolute_repository_paths} > 20;
    return $stats;
}

sub artifact_metadata {
    my ($path, $errors, $relative) = @_;
    open my $fh, '<:raw', $path or do {
        push @$errors, "cannot read canonical artifact '$relative': $!";
        return;
    };
    my %meta;
    while (my $line = <$fh>) {
        $meta{path_origin} = $1
            if !defined($meta{path_origin})
            && $line =~ /"path_origin"\s*:\s*"(repository_owned|external_input)"/;
        $meta{source_path_origin} = $1
            if !defined($meta{source_path_origin})
            && $line =~ /"source_path_origin"\s*:\s*"(repository_owned|external_input)"/;
        $meta{source_kind} = $1
            if !defined($meta{source_kind})
            && $line =~ /"source_kind"\s*:\s*"([a-z_]+)"/;
        last if defined($meta{path_origin}) && defined($meta{source_path_origin}) && defined($meta{source_kind});
    }
    close $fh or push @$errors, "cannot close canonical artifact '$relative': $!";
    return \%meta;
}

sub is_path_key {
    my ($key) = @_;
    return $key eq 'working_directory' || $key =~ /(?:\A|_)(?:path|root)\z/;
}

sub is_absolute_path {
    my ($value) = @_;
    return 1 if $value =~ m{\A/};
    return 1 if $value =~ m{\A\\\\};
    return 1 if $value =~ m{\A[A-Za-z]:[\\/]};
    return 0;
}

sub is_authorized_external {
    my ($kind, $key, $meta) = @_;
    if ($kind eq 'source' && ($meta->{path_origin} // '') eq 'external_input') {
        return 1 if $key eq 'requested_path' || $key eq 'canonical_path';
        return 1 if $key eq 'promoted_markdown_path' && ($meta->{source_kind} // '') eq 'markdown';
    }
    return 1
        if $kind eq 'source_metadata'
        && $key eq 'input_path'
        && ($meta->{path_origin} // '') eq 'external_input';
    return 1
        if $kind eq 'evidence'
        && $key eq 'source_path'
        && ($meta->{source_path_origin} // '') eq 'external_input';
    return 0;
}

sub decode_json_string {
    my ($encoded, $errors, $label) = @_;
    my $value = eval { JSON::PP->new->allow_nonref->decode(qq{"$encoded"}) };
    if ($@) {
        push @$errors, "cannot decode $label as a JSON string";
        return;
    }
    return $value;
}

sub migrate_legacy_root {
    my ($base, $artifacts, $legacy, $external_roots, $do_execute, $errors) = @_;
    if (!defined($legacy) || !File::Spec->file_name_is_absolute($legacy)) {
        push @$errors, 'legacy root must be an absolute path';
        return empty_migration($artifacts);
    }
    $legacy =~ s{/+\z}{};
    if ($legacy eq '' || $legacy eq $base || $legacy =~ /[\x00-\x1f"\\]/) {
        push @$errors, 'legacy root is unsafe or equals the current repository';
        return empty_migration($artifacts);
    }
    my @authorized_external_roots;
    for my $external (@$external_roots) {
        $external =~ s{/+\z}{};
        if (!File::Spec->file_name_is_absolute($external)
            || $external eq ''
            || $external eq $base
            || $external eq $legacy
            || $external =~ /[\x00-\x1f"\\]/
            || !-d $external)
        {
            push @$errors, "external source root is unsafe or unavailable: $external";
            next;
        }
        push @authorized_external_roots, $external;
    }

    my $source_origins = migration_source_origins(
        $base,
        $artifacts,
        $legacy,
        \@authorized_external_roots,
        $errors,
    );

    my $encoded_prefix = JSON::PP->new->allow_nonref->encode("$legacy/");
    $encoded_prefix =~ s/\A"//;
    $encoded_prefix =~ s/"\z//;
    my $needle = qq{"$encoded_prefix};
    my $migration = empty_migration($artifacts);

    for my $artifact (@$artifacts) {
        my ($relative, $path, $kind) = @$artifact;
        my $raw = read_raw($path, $errors, $relative);
        next if !defined $raw;
        my $all_occurrences = count_literal($raw, $legacy);
        my $path_occurrences = count_literal($raw, $needle);
        if ($all_occurrences != $path_occurrences) {
            push @$errors,
                "legacy root in '$relative' is not exclusively a JSON path-value prefix ($all_occurrences total, $path_occurrences migratable)";
            next;
        }
        my $updated = $raw;
        my $changed = ($updated =~ s/\Q$needle\E/"/g);
        if ($changed != $path_occurrences || index($updated, $legacy) >= 0) {
            push @$errors, "legacy-root rewrite was incomplete for '$relative'";
            next;
        }
        my $labels_added = add_origin_label(
            $relative,
            $kind,
            \$updated,
            $source_origins,
            $errors,
        );
        next if !defined $labels_added;
        next if $changed == 0 && $labels_added == 0;
        $migration->{changed_files}++;
        $migration->{changed_values} += $changed;
        $migration->{origin_labels_added} += $labels_added;
        $migration->{net_removed_bytes} += length($raw) - length($updated);
        atomic_write($path, $updated, $errors, $relative) if $do_execute;
    }
    return $migration;
}

sub empty_migration {
    my ($artifacts) = @_;
    return {
        artifact_files => scalar(@$artifacts),
        changed_files => 0,
        changed_values => 0,
        origin_labels_added => 0,
        net_removed_bytes => 0,
    };
}

sub migration_source_origins {
    my ($base, $artifacts, $legacy, $external_roots, $errors) = @_;
    my %origins;
    for my $artifact (@$artifacts) {
        my ($relative, $path, $kind) = @$artifact;
        next if $kind ne 'source';
        my ($document_key) = $relative =~ m{\Agenerated/source_ir/([^/]+)/source_ir\.json\z};
        my $raw = read_raw($path, $errors, $relative);
        next if !defined $raw;
        my $canonical = extract_json_string_field($raw, 'canonical_path', $errors, $relative);
        my $source_kind = extract_json_string_field($raw, 'source_kind', $errors, $relative);
        next if !defined $canonical || !defined $source_kind;
        my ($declared) = $raw =~ /"path_origin"\s*:\s*"(repository_owned|external_input)"/;
        my $origin = $declared;
        if (!defined $origin) {
            if (!is_absolute_path($canonical)
                || path_below($canonical, $legacy)
                || path_below($canonical, $base))
            {
                $origin = 'repository_owned';
            } else {
                my @matches = grep { path_below($canonical, $_) } @$external_roots;
                if (@matches != 1) {
                    push @$errors,
                        "unlabeled absolute source in '$relative' is not below exactly one authorized external root: $canonical";
                    next;
                }
                $origin = 'external_input';
            }
        }
        $origins{$document_key} = {
            source => $origin,
            promoted => $source_kind eq 'markdown' ? $origin : 'repository_owned',
        };
    }
    return \%origins;
}

sub add_origin_label {
    my ($relative, $kind, $raw_ref, $source_origins, $errors) = @_;
    if ($kind eq 'source') {
        return 0 if $$raw_ref =~ /"path_origin"\s*:/;
        my ($document_key) = $relative =~ m{\Agenerated/source_ir/([^/]+)/source_ir\.json\z};
        my $origin = $source_origins->{$document_key}{source};
        if (!defined $origin) {
            push @$errors, "cannot determine source origin for '$relative'";
            return;
        }
        my $changed = ($$raw_ref =~ s{
            ^(\s*"canonical_path"\s*:\s*"[^"]*",\n)
        }{$1 . qq{    "path_origin": "$origin",\n}}emx);
        if ($changed != 1) {
            push @$errors, "cannot insert source path_origin into '$relative'";
            return;
        }
        return 1;
    }
    if ($kind eq 'evidence') {
        return 0 if $$raw_ref =~ /"source_path_origin"\s*:/;
        my ($document_key) = $relative =~ m{\Agenerated/evidence_ir/([^/]+)/evidence_ir\.json\z};
        my $origin = $source_origins->{$document_key}{promoted};
        if (!defined $origin) {
            push @$errors, "cannot determine evidence source origin for '$relative'";
            return;
        }
        my $changed = ($$raw_ref =~ s{
            ^(\s*"source_ir_path"\s*:\s*"[^"]*",\n)
        }{$1 . qq{  "source_path_origin": "$origin",\n}}emx);
        if ($changed != 1) {
            push @$errors, "cannot insert source_path_origin into '$relative'";
            return;
        }
        return 1;
    }
    return 0;
}

sub extract_json_string_field {
    my ($raw, $field, $errors, $label) = @_;
    my @values = ($raw =~ /"\Q$field\E"\s*:\s*"([^"]*)"/g);
    if (@values != 1) {
        push @$errors, "expected exactly one $field in '$label', found " . scalar(@values);
        return;
    }
    return decode_json_string($values[0], $errors, "$label $field");
}

sub path_below {
    my ($path, $root_path) = @_;
    return 1 if $path eq $root_path;
    return index($path, "$root_path/") == 0;
}

sub atomic_write {
    my ($path, $raw, $errors, $relative) = @_;
    my $directory = dirname($path);
    my ($fh, $temporary) = tempfile('.persisted-path-migration.XXXXXX', DIR => $directory, UNLINK => 0);
    binmode $fh;
    if (!print {$fh} $raw) {
        push @$errors, "cannot write migration temporary for '$relative': $!";
        close $fh;
        unlink $temporary;
        return;
    }
    if (!close $fh) {
        push @$errors, "cannot close migration temporary for '$relative': $!";
        unlink $temporary;
        return;
    }
    my @stat = stat($path);
    if (!@stat || !chmod($stat[2] & 07777, $temporary) || !rename($temporary, $path)) {
        push @$errors, "cannot atomically replace '$relative': $!";
        unlink $temporary;
    }
}

sub read_raw {
    my ($path, $errors, $label) = @_;
    open my $fh, '<:raw', $path or do {
        push @$errors, "cannot read '$label': $!";
        return;
    };
    local $/;
    my $raw = <$fh>;
    close $fh or push @$errors, "cannot close '$label': $!";
    return $raw;
}

sub count_literal {
    my ($text, $literal) = @_;
    return 0 if !defined($literal) || $literal eq '';
    my $count = 0;
    my $offset = 0;
    while (1) {
        my $at = index($text, $literal, $offset);
        last if $at < 0;
        $count++;
        $offset = $at + length($literal);
    }
    return $count;
}

sub run_self_test {
    my @cases = (
        ['relative intent paths', 'intent', <<'JSON', 1],
{"semantic_ir_path":"generated/semantic_ir/doc/semantic_ir.json","artifact_root":"generated/intent_ir/doc"}
JSON
        ['current absolute path rejected', 'intent', <<'JSON', 0],
{"semantic_ir_path":"/repo/generated/semantic_ir/doc/semantic_ir.json"}
JSON
        ['retired absolute path rejected', 'adapter', <<'JSON', 0],
{"intent_ir_path":"/retired/specforge/generated/intent_ir/doc/intent_ir.json"}
JSON
        ['labeled external source accepted', 'source', <<'JSON', 1],
{"source":{"requested_path":"/inputs/spec.md","canonical_path":"/inputs/spec.md","path_origin":"external_input","source_kind":"markdown"},"promoted_markdown_path":"/inputs/spec.md"}
JSON
        ['unlabeled external source rejected', 'source', <<'JSON', 0],
{"source":{"requested_path":"/inputs/spec.md","canonical_path":"/inputs/spec.md","source_kind":"markdown"}}
JSON
        ['labeled external evidence accepted', 'evidence', <<'JSON', 1],
{"source_path_origin":"external_input","source_path":"/inputs/spec.md","source_ir_path":"generated/source_ir/doc/source_ir.json"}
JSON
        ['relative docling metadata accepted', 'source_metadata', <<'JSON', 1],
{"input_path":"corpus/spec.pdf","path_origin":"repository_owned","promoted_markdown_path":"generated/source_ir/doc/normalized/doc.md"}
JSON
        ['labeled external docling input accepted', 'source_metadata', <<'JSON', 1],
{"input_path":"/inputs/spec.pdf","path_origin":"external_input","promoted_markdown_path":"generated/source_ir/doc/normalized/doc.md"}
JSON
        ['unlabeled external docling input rejected', 'source_metadata', <<'JSON', 0],
{"input_path":"/inputs/spec.pdf","promoted_markdown_path":"generated/source_ir/doc/normalized/doc.md"}
JSON
        ['unknown path field fails closed', 'semantic', <<'JSON', 0],
{"raw_image_path":"/retired/specforge/generated/source_ir/doc/image.png"}
JSON
        ['Windows absolute path rejected', 'prior_memory', <<'JSON', 0],
{"artifact_path":"C:\\work\\specforge\\generated\\intent_ir\\doc\\intent_ir.json"}
JSON
        ['absolute working directory rejected', 'other', <<'JSON', 0],
{"working_directory":"/retired/specforge"}
JSON
    );

    for my $case (@cases) {
        my ($name, $kind, $raw, $should_pass) = @$case;
        my @errors;
        my $meta = metadata_from_text($raw);
        scan_text($kind, $raw, $meta, \@errors, $name);
        die "persisted-artifact-paths self-test '$name' failed\n"
            if ($should_pass && @errors) || (!$should_pass && !@errors);
    }

    my $legacy = '/retired/specforge';
    my $raw = '{"source_ir_path":"/retired/specforge/generated/source_ir/doc/source_ir.json",'
        . '"artifact_root":"/retired/specforge/generated/evidence_ir/doc"}';
    my $needle = qq{"$legacy/};
    my $updated = $raw;
    my $changed = ($updated =~ s/\Q$needle\E/"/g);
    die "persisted-artifact-paths self-test 'migration rewrite' failed\n"
        if $changed != 2 || index($updated, $legacy) >= 0;

    my $origins = {
        doc => {source => 'external_input', promoted => 'repository_owned'},
    };
    my $source = "{\n  \"source\": {\n    \"canonical_path\": \"/inputs/doc.pdf\",\n    \"source_kind\": \"pdf\"\n  }\n}\n";
    my @source_errors;
    my $source_added = add_origin_label(
        'generated/source_ir/doc/source_ir.json',
        'source',
        \$source,
        $origins,
        \@source_errors,
    );
    die "persisted-artifact-paths self-test 'source origin insertion' failed\n"
        if @source_errors || $source_added != 1 || $source !~ /"path_origin": "external_input"/;

    my $evidence = "{\n  \"source_ir_path\": \"generated/source_ir/doc/source_ir.json\",\n  \"stage\": \"evidence_ir\"\n}\n";
    my @evidence_errors;
    my $evidence_added = add_origin_label(
        'generated/evidence_ir/doc/evidence_ir.json',
        'evidence',
        \$evidence,
        $origins,
        \@evidence_errors,
    );
    die "persisted-artifact-paths self-test 'evidence origin insertion' failed\n"
        if @evidence_errors
        || $evidence_added != 1
        || $evidence !~ /"source_path_origin": "repository_owned"/;
}

sub metadata_from_text {
    my ($raw) = @_;
    my %meta;
    $meta{path_origin} = $1
        if $raw =~ /"path_origin"\s*:\s*"(repository_owned|external_input)"/;
    $meta{source_path_origin} = $1
        if $raw =~ /"source_path_origin"\s*:\s*"(repository_owned|external_input)"/;
    $meta{source_kind} = $1 if $raw =~ /"source_kind"\s*:\s*"([a-z_]+)"/;
    return \%meta;
}

sub scan_text {
    my ($kind, $raw, $meta, $errors, $label) = @_;
    while ($raw =~ /"([A-Za-z][A-Za-z0-9_]*)"\s*:\s*"([^"]*)"/g) {
        my $key = $1;
        next if !is_path_key($key);
        my $value = decode_json_string($2, $errors, "$label value");
        next if !defined $value || !is_absolute_path($value);
        next if is_authorized_external($kind, $key, $meta);
        push @$errors, "$label stores absolute $key";
    }
}
