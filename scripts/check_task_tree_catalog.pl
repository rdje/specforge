#!/usr/bin/env perl
use strict;
use warnings;
use utf8;

use Cwd qw(abs_path);
use Encode qw(decode_utf8 encode_utf8);
use Fcntl qw(O_CREAT O_EXCL O_WRONLY);
use File::Basename qw(basename dirname);
use File::Spec;
use FindBin qw($Bin);

binmode STDOUT, ':encoding(UTF-8)';
binmode STDERR, ':encoding(UTF-8)';

my $START = '<!-- task_catalog:start -->';
my $END = '<!-- task_catalog:end -->';
my $MAX_TASKS = 160;
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
    print "task-tree-catalog: 7/7 parser and bound tests pass.\n";
    exit 0;
}

my $ok = eval {
    my @tasks = collect_tasks($root);
    my $expected = render_catalog(\@tasks);
    my $index_path = File::Spec->catfile($root, 'docs', 'TASK_TREE.md');
    my $current = slurp_utf8($index_path);

    if ($mode eq 'write') {
        my $updated = replace_catalog($current, $expected);
        atomic_write($index_path, $updated) if $updated ne $current;
        print "task-tree-catalog: wrote " . scalar(@tasks)
          . " concise task routes to docs/TASK_TREE.md.\n";
    } else {
        my $actual = extract_catalog($current);
        die "derived catalog differs; run perl scripts/check_task_tree_catalog.pl --write\n"
          if $actual ne $expected;
        print "task-tree-catalog: " . scalar(@tasks)
          . " real task trees are linked exactly once in the bounded derived catalog.\n";
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
    die "catalog has " . scalar(@names) . " task trees; limit is $MAX_TASKS\n"
      if @names > $MAX_TASKS;

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

sub render_catalog {
    my ($tasks) = @_;
    my @lines = (
        $START,
        '## Task Tree Catalog',
        '',
        'This derived catalog is navigation, not execution history. Read `MEMORY.md` for the single',
        'resume pointer, then open the owning tree for its frontier, decisions, evidence, and commits.',
        'The author template is linked separately and is never classified as active work.',
        '',
        '| Tree | Status | Purpose | File |',
        '| --- | --- | --- | --- |',
    );
    for my $task (@$tasks) {
        my $title = $task->{title};
        $title =~ s/\\/\\\\/g;
        $title =~ s/\|/\\|/g;
        my $row = '| `' . $task->{id} . '` | `' . $task->{status} . '` | '
          . $title . ' | [open](tasks/' . $task->{name} . ') |';
        die "$task->{name} catalog row exceeds $MAX_ROW_BYTES UTF-8 bytes\n"
          if length(encode_utf8($row)) > $MAX_ROW_BYTES;
        push @lines, $row;
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
}
