#!/usr/bin/env perl
use strict;
use warnings;
use utf8;

use Cwd qw(abs_path);
use Encode qw(decode_utf8 encode_utf8);
use Fcntl qw(O_CREAT O_EXCL O_WRONLY);
use File::Basename qw(basename dirname);
use File::Path qw(make_path);
use File::Spec;
use FindBin qw($Bin);

binmode STDOUT, ':encoding(UTF-8)';
binmode STDERR, ':encoding(UTF-8)';

my $START = '<!-- task_catalog:start -->';
my $END = '<!-- task_catalog:end -->';
# The landing carries only the trees a session can still act on; finished trees route to derived parts, the
# way the Knowledge Map routes questions to shards and the fact-card catalog routes titles to parts. The
# landing therefore grows with concurrent work in flight, not with project lifetime (ADR 0045,
# LIVE-DOCUMENT-PRESSURE-HEADROOM.2c). Nothing is deleted and nothing is archived: every tree appears in
# exactly one part, and the parts are derived, never hand-edited.
my $PART_DIRECTORY = File::Spec->catdir('docs', 'task-catalog');
my $PART_PREFIX = 'catalog-';
my $TREES_PER_PART = 56;
my $MAX_PARTS = 16;
my %TERMINAL_STATUS = map { $_ => 1 } qw(done superseded);
# There is deliberately no task-count cap here. It was a second, independent enforcer of the same 160 that
# doctrine/live_document_size/surfaces.jsonl declared, so a registry-only removal would have read as delivered
# while this literal still refused the 161st tree. The cardinality bound now lives in exactly one place — the
# registry, which nulls it behind a declared, checker-enforced exemption (ADR 0045,
# LIVE-DOCUMENT-PRESSURE-HEADROOM.2a). The bounds below stay: they bound content, not cardinality.
my $MAX_SECTION_BYTES = 49_152;
my $MAX_ROW_BYTES = 512;
my %VALID_STATUS = map { $_ => 1 }
  qw(proposed active pending in_progress blocked done deferred superseded);

my $root = abs_path(File::Spec->catdir($Bin, '..'));
my $mode = 'check';
while (@ARGV) {
    my $arg = shift @ARGV;
    if ($arg eq '--check') {
        $mode = 'check';
    } elsif ($arg eq '--write') {
        $mode = 'write';
    } elsif ($arg eq '--self-test') {
        $mode = 'self-test';
    } elsif ($arg eq '--root') {
        die "task-tree-catalog: --root requires a path\n" if !@ARGV;
        $root = abs_path(shift @ARGV);
        die "task-tree-catalog: root does not exist\n" if !defined $root;
    } else {
        die "Usage: $0 [--check|--write|--self-test] [--root PROJECT_ROOT]\n";
    }
}

if ($mode eq 'self-test') {
    run_self_test();
    print "task-tree-catalog: 13/13 parser, partition, and bound tests pass.\n";
    exit 0;
}

my $ok = eval {
    my @tasks = collect_tasks($root);
    my @parts = plan_parts(\@tasks);
    my $expected = render_catalog(\@tasks, \@parts);
    my $index_path = File::Spec->catfile($root, 'docs', 'TASK_TREE.md');
    my $current = slurp_utf8($index_path);
    my $open_count = scalar grep { !$TERMINAL_STATUS{ $_->{status} } } @tasks;

    if ($mode eq 'write') {
        my $updated = replace_catalog($current, $expected);
        atomic_write($index_path, $updated) if $updated ne $current;
        my $part_root = File::Spec->catdir($root, split m{/}, 'docs/task-catalog');
        make_path($part_root) if !-d $part_root;
        for my $part (@parts) {
            my $part_path = File::Spec->catfile($root, split m{/}, $part->{path});
            my $existing = -f $part_path ? slurp_utf8($part_path) : '';
            atomic_write($part_path, $part->{content}) if $existing ne $part->{content};
        }
        remove_stale_parts($root, \@parts);
        print "task-tree-catalog: wrote $open_count open routes to docs/TASK_TREE.md and "
          . scalar(@tasks) . " trees across " . scalar(@parts) . " derived catalog parts.\n";
    } else {
        my $actual = extract_catalog($current);
        die "derived landing differs; run perl scripts/check_task_tree_catalog.pl --write\n"
          if $actual ne $expected;
        for my $part (@parts) {
            my $part_path = File::Spec->catfile($root, split m{/}, $part->{path});
            die "derived catalog part is missing: $part->{path}\n" if !-f $part_path;
            die "derived catalog part differs: $part->{path}; run --write\n"
              if slurp_utf8($part_path) ne $part->{content};
        }
        my @stale = stale_parts($root, \@parts);
        die "derived catalog part is not planned: $stale[0]; run --write\n" if @stale;
        print "task-tree-catalog: " . scalar(@tasks) . " real task trees are linked exactly once across "
          . scalar(@parts) . " derived parts, with $open_count open on the bounded landing.\n";
    }
    1;
};
if (!$ok) {
    my $error = $@ || 'unknown failure';
    $error =~ s/\s+\z//;
    print STDERR "task-tree-catalog: $error\n";
    exit 1;
}

sub collect_tasks {
    my ($project_root) = @_;
    my $task_dir = File::Spec->catdir($project_root, 'docs', 'tasks');
    die "task directory is missing: docs/tasks\n" if !-d $task_dir;

    opendir(my $dh, $task_dir) or die "cannot open docs/tasks: $!\n";
    my @names = sort grep { /\.md\z/ && $_ ne 'TEMPLATE.md' } readdir($dh);
    closedir($dh) or die "cannot close docs/tasks: $!\n";
    die "catalog has no task trees\n" if !@names;

    my @tasks;
    for my $name (@names) {
        die "unsafe task filename '$name'\n" if $name !~ /\A[A-Z0-9][A-Z0-9-]*\.md\z/;
        my $path = File::Spec->catfile($task_dir, $name);
        push @tasks, parse_task($name, slurp_utf8($path));
    }
    return @tasks;
}

sub parse_task {
    my ($name, $text) = @_;
    my ($expected_id) = $name =~ /\A(.+)\.md\z/;
    my ($id, $title) = $text =~ /\A# ([A-Z0-9][A-Z0-9-]*):[ \t]*(.+)\r?\n/;
    die "$name lacks canonical '# TREE-ID: title' H1\n" if !defined $id;
    die "$name H1 id '$id' does not match filename '$expected_id'\n"
      if $id ne $expected_id;
    die "$name title exceeds 240 UTF-8 bytes\n" if length(encode_utf8($title)) > 240;

    my ($status) = $text =~ /^- Status:[ \t]*(?:\*\*)?`([a-z_]+)`(?:\*\*)?/m;
    die "$name lacks a metadata '- Status: `...`' field\n" if !defined $status;
    die "$name has unknown status '$status'\n" if !$VALID_STATUS{$status};
    return { id => $id, title => $title, status => $status, name => $name };
}

sub catalog_row {
    my ($task, $prefix) = @_;
    my $title = $task->{title};
    $title =~ s/\\/\\\\/g;
    $title =~ s/\|/\\|/g;
    my $row = '| `' . $task->{id} . '` | `' . $task->{status} . '` | '
      . $title . ' | [open](' . $prefix . $task->{name} . ') |';
    die "$task->{name} catalog row exceeds $MAX_ROW_BYTES UTF-8 bytes\n"
      if length(encode_utf8($row)) > $MAX_ROW_BYTES;
    return $row;
}

sub plan_parts {
    my ($tasks) = @_;
    my @parts;
    my $index = 0;
    while ($index < @$tasks) {
        my $last = $index + $TREES_PER_PART - 1;
        $last = $#$tasks if $last > $#$tasks;
        my @members = @{$tasks}[$index .. $last];
        my $number = scalar(@parts) + 1;
        die "derived catalog needs more than $MAX_PARTS parts; raise the quantum or the part bound\n"
          if $number > $MAX_PARTS;
        my $name = sprintf('%s%04d.md', $PART_PREFIX, $number);
        my $path = join('/', 'docs', 'task-catalog', $name);
        push @parts, {
            number => $number,
            name => $name,
            path => $path,
            first => $members[0]{id},
            last => $members[-1]{id},
            count => scalar(@members),
            content => render_part($number, \@members),
        };
        $index = $last + 1;
    }
    return @parts;
}

sub render_part {
    my ($number, $members) = @_;
    my @lines = (
        sprintf('# Task tree catalog part %04d', $number),
        '',
        '> **AUTO-GENERATED — DO NOT EDIT.** Regenerate with',
        '> `perl scripts/check_task_tree_catalog.pl --write`.',
        '',
        'Complete membership for this range. The bounded landing is',
        '[`docs/TASK_TREE.md`](../TASK_TREE.md); it carries the open trees and routes here for the rest.',
        '',
        '| Tree | Status | Purpose | File |',
        '| --- | --- | --- | --- |',
    );
    push @lines, catalog_row($_, '../tasks/') for @$members;
    push @lines, '';
    my $content = join("\n", @lines) . "\n";
    die sprintf("derived catalog part %04d exceeds %d UTF-8 bytes\n", $number, $MAX_SECTION_BYTES)
      if length(encode_utf8($content)) > $MAX_SECTION_BYTES;
    return $content;
}

sub stale_parts {
    my ($project_root, $parts) = @_;
    my %planned = map { $_->{name} => 1 } @$parts;
    my $directory = File::Spec->catdir($project_root, split m{/}, 'docs/task-catalog');
    return () if !-d $directory;
    opendir(my $dh, $directory) or die "cannot open docs/task-catalog: $!\n";
    my @names = sort grep { /\A\Q$PART_PREFIX\E[0-9]{4}\.md\z/ } readdir($dh);
    closedir($dh) or die "cannot close docs/task-catalog: $!\n";
    return grep { !$planned{$_} } @names;
}

sub remove_stale_parts {
    my ($project_root, $parts) = @_;
    for my $name (stale_parts($project_root, $parts)) {
        my $path = File::Spec->catfile($project_root, 'docs', 'task-catalog', $name);
        unlink $path or die "cannot remove stale catalog part '$name': $!\n";
    }
}

sub render_catalog {
    my ($tasks, $parts) = @_;
    my @open = grep { !$TERMINAL_STATUS{ $_->{status} } } @$tasks;
    my @lines = (
        $START,
        '## Task Tree Catalog',
        '',
        'This derived catalog is navigation, not execution history. Read `MEMORY.md` for the single',
        'resume pointer, then open the owning tree for its frontier, decisions, evidence, and commits.',
        'The author template is linked separately and is never classified as active work.',
        '',
        '### Open trees',
        '',
        'Every tree that is not `done` or `superseded` — the set a session can still act on. There is no',
        'limit on how many task-trees exist; a finished tree stays as project history in the parts below',
        '(ADR 0045).',
        '',
        '| Tree | Status | Purpose | File |',
        '| --- | --- | --- | --- |',
    );
    push @lines, catalog_row($_, 'tasks/') for @open;
    push @lines,
      '',
      '### Complete catalog',
      '',
      sprintf('All %d trees route through %d derived part(s); open a range to find an id.',
        scalar(@$tasks), scalar(@$parts)),
      '',
      '| Part | Trees | First id | Last id |',
      '| --- | ---: | --- | --- |';
    for my $part (@$parts) {
        push @lines, sprintf('| [%04d](task-catalog/%s) | %d | `%s` | `%s` |',
          $part->{number}, $part->{name}, $part->{count}, $part->{first}, $part->{last});
    }
    push @lines,
      '',
      'Authoring template: [`docs/tasks/TEMPLATE.md`](tasks/TEMPLATE.md).',
      '',
      $END;
    my $section = join("\n", @lines) . "\n";
    die "catalog section exceeds $MAX_SECTION_BYTES UTF-8 bytes\n"
      if length(encode_utf8($section)) > $MAX_SECTION_BYTES;
    return $section;
}

sub extract_catalog {
    my ($document) = @_;
    my $start_count = () = $document =~ /\Q$START\E/g;
    my $end_count = () = $document =~ /\Q$END\E/g;
    die "docs/TASK_TREE.md must contain exactly one catalog marker pair\n"
      if $start_count != 1 || $end_count != 1;
    my ($section) = $document =~ /(\Q$START\E\n.*?\Q$END\E\n)/s;
    die "docs/TASK_TREE.md catalog markers are malformed\n" if !defined $section;
    return $section;
}

sub replace_catalog {
    my ($document, $expected) = @_;
    if (index($document, $START) >= 0 || index($document, $END) >= 0) {
        my $actual = extract_catalog($document);
        $document =~ s/\Q$actual\E/$expected/;
        return $document;
    }

    my $changed = $document =~ s/
      \n\#\#[ ]Active[ ]Task[ ]Trees\n
      .*?
      (?=\n\#\#[ ]Directory[ ]Layout\n)
    /\n$expected/sx;
    die "docs/TASK_TREE.md lacks the legacy Active Task Trees section\n" if !$changed;
    return $document;
}

sub slurp_utf8 {
    my ($path) = @_;
    open(my $fh, '<:raw', $path) or die "cannot read '$path': $!\n";
    local $/;
    my $bytes = <$fh>;
    close($fh) or die "cannot close '$path': $!\n";
    return decode_utf8($bytes, 1);
}

sub atomic_write {
    my ($path, $text) = @_;
    my $temporary = $path . '.task-catalog.' . $$;
    sysopen(my $fh, $temporary, O_WRONLY | O_CREAT | O_EXCL, 0644)
      or die "cannot create same-directory temporary '$temporary': $!\n";
    binmode $fh, ':raw';
    print {$fh} encode_utf8($text) or die "cannot write '$temporary': $!\n";
    close($fh) or die "cannot close '$temporary': $!\n";
    rename($temporary, $path) or die "cannot replace '$path': $!\n";
}

sub run_self_test {
    my $normal = "# ALPHA: compact title\n\n## Metadata\n\n- Status: `done` (closed)\n";
    my $bold = "# BETA: title with | separator\n\n- Status: **`active`** — live\n";
    my $alpha = parse_task('ALPHA.md', $normal);
    die "normal parser case failed\n" if $alpha->{status} ne 'done';
    my $beta = parse_task('BETA.md', $bold);
    die "bold parser case failed\n" if $beta->{status} ne 'active';
    my $rendered = render_catalog([$alpha, $beta]);
    die "Markdown pipe escaping failed\n" if index($rendered, 'title with \\| separator') < 0;

    my $missing = eval { parse_task('GAMMA.md', "# GAMMA: title\n"); 1 };
    die "missing-status case did not fail\n" if $missing;
    my $unknown = eval {
        parse_task('DELTA.md', "# DELTA: title\n- Status: `finished`\n"); 1;
    };
    die "unknown-status case did not fail\n" if $unknown;
    my $mismatch = eval {
        parse_task('EPSILON.md', "# OTHER: title\n- Status: `done`\n"); 1;
    };
    die "filename/H1 mismatch case did not fail\n" if $mismatch;
    my $long = 'x' x 500;
    my $wide = eval {
        parse_task('ZETA.md', "# ZETA: $long\n- Status: `done`\n"); 1;
    };
    die "oversized-title case did not fail\n" if $wide;

    # Landing/part partition (LIVE-DOCUMENT-PRESSURE-HEADROOM.2c). The landing must carry open trees only,
    # every tree must appear in exactly one part, and the part quantum must actually bind.
    my @many = map {
        { id => sprintf('T%03d', $_), name => sprintf('T%03d.md', $_), title => 'fixture',
          status => ($_ % 2 ? 'done' : 'active') }
    } 1 .. ($TREES_PER_PART + 3);
    my @parts = plan_parts(\@many);
    die "part quantum did not bind\n" if scalar(@parts) != 2;
    die "first part is not full\n" if $parts[0]{count} != $TREES_PER_PART;
    die "second part has the wrong remainder\n" if $parts[1]{count} != 3;
    my $total = 0;
    $total += $_->{count} for @parts;
    die "parts do not cover every tree exactly once\n" if $total != scalar(@many);

    my $landing = render_catalog(\@many, \@parts);
    my $open_rows = () = $landing =~ /^\| `T\d{3}` \| `active`/mg;
    my $closed_rows = () = $landing =~ /^\| `T\d{3}` \| `done`/mg;
    die "landing must list every open tree\n"
      if $open_rows != scalar grep { $_->{status} eq 'active' } @many;
    die "landing must not list a terminal tree\n" if $closed_rows;
    die "landing must route to its parts\n" if index($landing, 'task-catalog/catalog-0002.md') < 0;
    die "part must route back to the landing\n"
      if index($parts[0]{content}, '../TASK_TREE.md') < 0;
    die "part rows must resolve from the part directory\n"
      if index($parts[0]{content}, '[open](../tasks/T001.md)') < 0;

    my $overflow = eval {
        plan_parts([ map { { id => "U$_", name => "U$_.md", title => 'x', status => 'done' } }
                     1 .. ($TREES_PER_PART * $MAX_PARTS + 1) ]);
        1;
    };
    die "part-count overflow case did not fail\n" if $overflow;
}
