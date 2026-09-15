#!/usr/bin/env perl
use strict;
use warnings;

use Cwd qw(abs_path);
use Digest::SHA qw(sha256_hex);
use File::Basename qw(dirname);
use File::Path qw(make_path);
use File::Spec;
use File::Temp qw(tempdir);
use JSON::PP;

my $root = abs_path(File::Spec->catdir(dirname(abs_path($0)), '..'));
my $checker = File::Spec->catfile($root, 'scripts', 'check_live_document_size.pl');
my $generated = File::Spec->catdir($root, 'generated');
make_path($generated);

my $json = JSON::PP->new->canonical(1);
my @dimensions = qw(files lines_each bytes_each lines_total bytes_total line_bytes_each);
my $quiet = 0;
if (@ARGV) {
    die "Usage: $0 [--quiet]\n" if @ARGV != 1 || $ARGV[0] ne '--quiet';
    $quiet = 1;
}
my $test_number = 0;
my $failures = 0;

sub path_in {
    my ($directory, $relative) = @_;
    return File::Spec->catfile($directory, split m{/}, $relative);
}

sub write_text {
    my ($directory, $relative, $content, $mode) = @_;
    my $path = path_in($directory, $relative);
    make_path(dirname($path));
    open my $fh, '>:raw', $path or die "cannot write fixture '$path': $!\n";
    print {$fh} $content;
    close $fh or die "cannot close fixture '$path': $!\n";
    chmod($mode, $path) or die "cannot chmod fixture '$path': $!\n" if defined $mode;
}

sub read_text {
    my ($directory, $relative) = @_;
    my $path = path_in($directory, $relative);
    open my $fh, '<:raw', $path or die "cannot read fixture '$path': $!\n";
    local $/;
    my $content = <$fh> // '';
    close $fh;
    return $content;
}

sub dimensions_for {
    my ($directory, @paths) = @_;
    my %metrics = map { $_ => 0 } @dimensions;
    $metrics{files} = scalar @paths;
    for my $path (@paths) {
        my $content = read_text($directory, $path);
        my $bytes = length($content);
        my $lines = ($content =~ tr/\n/\n/);
        my $max_line = 0;
        for my $line (split /\n/, $content, -1) {
            $line =~ s/\r\z//;
            $max_line = length($line) if length($line) > $max_line;
        }
        $metrics{lines_total} += $lines;
        $metrics{bytes_total} += $bytes;
        $metrics{lines_each} = $lines if $lines > $metrics{lines_each};
        $metrics{bytes_each} = $bytes if $bytes > $metrics{bytes_each};
        $metrics{line_bytes_each} = $max_line if $max_line > $metrics{line_bytes_each};
    }
    return \%metrics;
}

sub generous_dimensions {
    return {
        files => 16,
        lines_each => 100,
        bytes_each => 4_096,
        # A collection's aggregate is its file bound times its per-file bound, so a corpus of individually
        # legal files is never refused by a total no single file can see (ADR 0032).
        lines_total => 1_600,
        bytes_total => 65_536,
        line_bytes_each => 256,
    };
}

sub base_surface {
    my (%args) = @_;
    return {
        surface_id => $args{id},
        targets => $args{targets},
        locator => $args{locator} // 'file',
        lifecycle => $args{lifecycle},
        state => $args{state} // 'normal',
        owner => 'fixture-maintainers',
        health_targets => generous_dimensions(),
        enforcement_ceilings => generous_dimensions(),
        milestones => { warning_pct => 80, rollover_pct => 90 },
        verifier => 'builtin:fixture',
    };
}

sub save_jsonl {
    my ($path, $meta, $records) = @_;
    make_path(dirname($path));
    open my $fh, '>:raw', $path or die "cannot write fixture registry '$path': $!\n";
    print {$fh} $json->encode($meta), "\n";
    print {$fh} $json->encode($_), "\n" for @$records;
    close $fh or die "cannot close fixture registry '$path': $!\n";
}

sub save_registry {
    my ($fixture) = @_;
    save_jsonl(
        path_in($fixture->{root}, 'control/surfaces.jsonl'),
        $fixture->{registry_meta},
        $fixture->{surfaces},
    );
}

sub save_authorities {
    my ($fixture) = @_;
    save_jsonl(
        path_in($fixture->{root}, 'control/authorities.jsonl'),
        $fixture->{authority_meta},
        $fixture->{authorities},
    );
}

sub surface {
    my ($fixture, $id) = @_;
    for my $candidate (@{ $fixture->{surfaces} }) {
        return $candidate if ($candidate->{surface_id} // '') eq $id;
    }
    die "fixture surface '$id' does not exist\n";
}

sub new_fixture {
    my $directory = tempdir(
        'live-document-size-tests.XXXXXX',
        DIR => $generated,
        CLEANUP => 1,
    );

    write_text($directory, 'snapshot.md', "# Snapshot\n[External part](external/part.md)\n");
    write_text($directory, 'ledger.md', "entry\n");
    write_text($directory, 'canonical/INDEX.md', "[Part](part.md)\n");
    write_text($directory, 'canonical/part.md', "# Canonical part\n");
    write_text($directory, 'query/part.md', "# Query part\n");
    write_text($directory, 'external/part.md', "# External-index part\n");
    write_text(
        $directory,
        'routed/INDEX.md',
        "# Routed landing\n[Collection README](README.md)\n[Titles 0001](../routed-titles/titles-0001.md)\n",
    );
    write_text($directory, 'routed/README.md', "# Routed collection guidance\n");
    write_text($directory, 'routed/part.md', "# Routed member\n");
    write_text(
        $directory,
        'routed-titles/titles-0001.md',
        "# Routed titles part 0001\n[Routed member](../routed/part.md)\n",
    );
    write_text($directory, 'canonical/source.txt', "canonical input\n");
    write_text($directory, 'generated.md', "# Generated\n");
    write_text($directory, 'archive.md', "# Archive descriptor\n");
    write_text($directory, 'control/archive-manifest.json', "{\"fixture\":true}\n");
    write_text($directory, 'frozen.md', "sealed\n");
    write_text($directory, 'book/SUMMARY.md', "[Part](part.md)\n");
    write_text($directory, 'book/part.md', "# Maintained part\n");
    write_text($directory, 'scripts/freshness-ok.sh', "#!/usr/bin/env bash\nexit 0\n", 0755);
    write_text($directory, 'scripts/currency-ok.sh', "#!/usr/bin/env bash\nexit 0\n", 0755);

    my $ledger_metrics = dimensions_for($directory, 'ledger.md');
    my $book_metrics = dimensions_for($directory, 'book/SUMMARY.md', 'book/part.md');
    my $frozen_sha = sha256_hex(read_text($directory, 'frozen.md'));

    my $snapshot = base_surface(
        id => 'snapshot',
        targets => ['snapshot.md'],
        lifecycle => 'bounded_snapshot',
    );
    $snapshot->{currency} = {
        status => 'enforced',
        owner => 'fixture-maintainers',
        verifier => 'scripts/currency-ok.sh',
    };

    my $ledger = base_surface(
        id => 'ledger',
        targets => ['ledger.md'],
        lifecycle => 'rolling_ledger',
        state => 'transition_debt',
    );
    $ledger->{baseline} = { %$ledger_metrics };
    $ledger->{transition} = {
        owner => 'fixture-transition',
        max_growth => { map { $_ => 0 } @dimensions },
    };

    my $canonical = base_surface(
        id => 'canonical',
        targets => ['canonical/*.md'],
        locator => 'collection',
        lifecycle => 'partitioned_canonical',
    );
    $canonical->{index} = 'canonical/INDEX.md';
    $canonical->{index_contract} = { kind => 'membership', verifier => 'builtin:markdown_links' };

    my $query = base_surface(
        id => 'query',
        targets => ['query/*.md'],
        locator => 'collection',
        lifecycle => 'partitioned_canonical',
    );
    $query->{index} = 'git:query';
    $query->{index_contract} = { kind => 'query', verifier => 'builtin:registry_targets' };

    my $external = base_surface(
        id => 'external',
        targets => ['external/*.md'],
        locator => 'collection',
        lifecycle => 'partitioned_canonical',
    );
    $external->{index} = 'snapshot.md';
    $external->{index_contract} = {
        kind => 'external_membership',
        verifier => 'builtin:markdown_links',
    };

    my $routed = base_surface(
        id => 'routed',
        targets => ['routed/*.md'],
        locator => 'collection',
        lifecycle => 'partitioned_canonical',
    );
    $routed->{index} = 'routed/INDEX.md';
    $routed->{index_contract} = {
        kind => 'routed_membership',
        verifier => 'builtin:markdown_links',
        route_surface => 'routed_titles',
    };

    my $routed_titles = base_surface(
        id => 'routed_titles',
        targets => ['routed-titles/*.md'],
        locator => 'collection',
        lifecycle => 'partitioned_canonical',
    );
    $routed_titles->{index} = 'routed/INDEX.md';
    $routed_titles->{index_contract} = {
        kind => 'external_membership',
        verifier => 'builtin:markdown_links',
    };

    my $generated_projection = base_surface(
        id => 'projection',
        targets => ['generated.md'],
        lifecycle => 'generated_projection',
    );
    $generated_projection->{canonical_inputs} = ['canonical/source.txt'];
    $generated_projection->{freshness_verifier} = 'scripts/freshness-ok.sh';

    my $archive = base_surface(
        id => 'archive',
        targets => ['archive.md'],
        lifecycle => 'archive_terminal',
        state => 'terminal',
    );
    $archive->{archive_manifest} = 'control/archive-manifest.json';
    $archive->{currency} = {
        status => 'transition_debt',
        owner => 'fixture-transition',
        verifier => 'debt:fixture-archive-currentness',
    };

    my $frozen = base_surface(
        id => 'frozen',
        targets => ['frozen.md'],
        lifecycle => 'frozen_legacy',
        state => 'frozen',
    );
    $frozen->{sha256} = $frozen_sha;

    my $maintained = base_surface(
        id => 'maintained',
        targets => ['book/*.md'],
        locator => 'collection',
        lifecycle => 'maintained_reference',
    );
    $maintained->{health_targets}{$_} = undef for qw(files lines_total bytes_total);
    $maintained->{enforcement_ceilings}{$_} = undef for qw(files lines_total bytes_total);
    $maintained->{index} = 'book/SUMMARY.md';
    $maintained->{index_contract} = { kind => 'membership', verifier => 'builtin:markdown_links' };
    $maintained->{reference_contract} = {
        mandatory_read => {
            path => 'book/SUMMARY.md',
            lines_ceiling => 8,
            bytes_ceiling => 1_024,
        },
        max_navigation_depth => 1,
        aggregate_change => {
            authority_id => 'fixture-maintained-reference',
            owner => 'fixture-maintainers',
            baseline => {
                files => $book_metrics->{files},
                lines_total => $book_metrics->{lines_total},
                bytes_total => $book_metrics->{bytes_total},
            },
            delta => { files => 0, lines_total => 0, bytes_total => 0 },
            rationale => 'Exercise exact maintained-reference aggregate authority.',
        },
    };

    my $fixture = {
        root => $directory,
        registry_meta => {
            record_type => 'registry',
            schema_version => 1,
            max_records => 16,
            max_bytes => 32_768,
            max_record_bytes => 4_096,
            max_array_items => 16,
            max_scalar_bytes => 512,
            milestones => { warning_pct => 80, rollover_pct => 90 },
        },
        surfaces => [
            $snapshot,
            $ledger,
            $canonical,
            $query,
            $external,
            $routed,
            $routed_titles,
            $generated_projection,
            $archive,
            $frozen,
            $maintained,
        ],
        authority_meta => {
            record_type => 'registry',
            schema_version => 1,
            max_records => 8,
            max_bytes => 16_384,
            max_record_bytes => 4_096,
            max_array_items => 16,
            max_scalar_bytes => 512,
            milestones => { warning_pct => 80, rollover_pct => 90 },
        },
        authorities => [],
    };
    save_registry($fixture);
    save_authorities($fixture);
    write_text(
        $directory,
        'control/routes.tsv',
        "target\troute_kind\towner\tlifecycle\tpressure_control\tterminal\n" .
        "snapshot.md\treader_navigation\tfixture\tmaintained_entrypoint\treview_on_behavior_change\tyes\n" .
        "git:history\tauthor_overflow\tfixture\tappend_only_history\tquery_with_git_log\tyes\n",
    );
    return $fixture;
}

sub run_checker {
    my ($fixture, $with_history) = @_;
    my $log_path = path_in($fixture->{root}, 'check-output.log');
    open my $log, '>:raw', $log_path or die "cannot write checker log: $!\n";
    my @command = (
        $^X,
        $checker,
        '--root', $fixture->{root},
        '--registry', 'control/surfaces.jsonl',
        '--routes', 'control/routes.tsv',
        '--authorities', 'control/authorities.jsonl',
    );
    push @command, '--no-history' if !$with_history;
    my $pid = fork();
    die "cannot fork checker test: $!\n" if !defined $pid;
    if ($pid == 0) {
        open STDOUT, '>&', $log or die "cannot redirect checker stdout: $!\n";
        open STDERR, '>&', $log or die "cannot redirect checker stderr: $!\n";
        exec { $command[0] } @command;
        exit 127;
    }
    waitpid($pid, 0);
    my $status = $? >> 8;
    close $log;
    return ($status, read_text($fixture->{root}, 'check-output.log'));
}

sub report_result {
    my ($name, $passed, $output) = @_;
    $test_number++;
    if ($passed) {
        print "ok $test_number - $name\n" if !$quiet;
        return;
    }
    $failures++;
    print "not ok $test_number - $name\n";
    $output =~ s/^/# /mg;
    print $output;
}

sub expect_case {
    my ($name, $expect_success, $pattern, $mutator) = @_;
    my $fixture = new_fixture();
    $mutator->($fixture) if $mutator;
    my ($status, $output) = run_checker($fixture, 0);
    my $passed = $expect_success ? $status == 0 : $status != 0;
    $passed &&= $output =~ $pattern if defined $pattern;
    report_result($name, $passed, $output);
}

# Some properties are about silence: a bound below its band must report nothing, and a suite that can
# only assert presence cannot tell "quiet because correct" from "quiet because unimplemented".
sub expect_absent_case {
    my ($name, $pattern, $mutator) = @_;
    my $fixture = new_fixture();
    $mutator->($fixture) if $mutator;
    my ($status, $output) = run_checker($fixture, 0);
    my $passed = $status == 0 && $output !~ $pattern;
    report_result($name, $passed, $output);
}

sub initialize_git_history {
    my ($fixture) = @_;
    my $directory = $fixture->{root};
    for my $command (
        ['git', '-C', $directory, 'init', '-q'],
        ['git', '-C', $directory, 'config', 'user.name', 'SpecForge Fixture'],
        ['git', '-C', $directory, 'config', 'user.email', 'fixture@specforge.invalid'],
        ['git', '-C', $directory, 'config', 'core.hooksPath', '.git/no-hooks'],
        ['git', '-C', $directory, 'add', '.'],
        ['git', '-C', $directory, 'commit', '-q', '-m', 'fixture baseline'],
    ) {
        system @$command;
        die "fixture Git command failed: @$command\n" if $? != 0;
    }
}

sub expect_history_case {
    my ($name, $expect_success, $pattern, $mutator) = @_;
    my $fixture = new_fixture();
    initialize_git_history($fixture);
    $mutator->($fixture);
    my ($status, $output) = run_checker($fixture, 1);
    my $passed = $expect_success ? $status == 0 : $status != 0;
    $passed &&= $output =~ $pattern if defined $pattern;
    report_result($name, $passed, $output);
}

for my $lifecycle (
    'bounded_snapshot',
    'rolling_ledger',
    'partitioned_canonical membership and query',
    'partitioned_canonical external membership',
    'partitioned_canonical routed membership',
    'generated_projection',
    'archive_terminal',
    'frozen_legacy',
    'maintained_reference',
) {
    expect_case("positive $lifecycle", 1, qr/11 governed surfaces/, undef);
}

expect_case('bounded snapshot rejects multiple files', 0, qr/bounded_snapshot must contain exactly one file/, sub {
    my ($fixture) = @_;
    write_text($fixture->{root}, 'snapshot-extra.md', "# Extra\n");
    push @{ surface($fixture, 'snapshot')->{targets} }, 'snapshot-extra.md';
    save_registry($fixture);
});
expect_case('rolling ledger rejects unowned live growth', 0, qr/must remain transition_debt/, sub {
    my ($fixture) = @_;
    my $surface = surface($fixture, 'ledger');
    $surface->{state} = 'normal';
    delete $surface->{baseline};
    delete $surface->{transition};
    save_registry($fixture);
});
expect_case('partition membership index rejects a stale member list', 0, qr/does not link member 'canonical\/part\.md'/, sub {
    my ($fixture) = @_;
    write_text($fixture->{root}, 'canonical/INDEX.md', "# Empty index\n");
});
expect_case('partition query index rejects the wrong query authority', 0, qr/query index must be git:query/, sub {
    my ($fixture) = @_;
    surface($fixture, 'query')->{index} = 'query:uncontrolled';
    save_registry($fixture);
});
expect_case('partition query index rejects an uncontrolled verifier', 0, qr/query index verifier must be builtin:registry_targets/, sub {
    my ($fixture) = @_;
    surface($fixture, 'query')->{index_contract}{verifier} = 'builtin:uncontrolled';
    save_registry($fixture);
});
expect_case('external membership index rejects a stale member list', 0, qr/does not link member 'external\/part\.md'/, sub {
    my ($fixture) = @_;
    write_text($fixture->{root}, 'snapshot.md', "# Empty external index\n");
});
expect_case('external membership rejects a missing index', 0, qr/external_membership index .* is not a classified Markdown surface|membership index .* is missing/, sub {
    my ($fixture) = @_;
    surface($fixture, 'external')->{index} = 'missing.md';
    save_registry($fixture);
});
expect_case('external membership rejects an off-root index', 0, qr/must be one safe repository-relative Markdown path/, sub {
    my ($fixture) = @_;
    surface($fixture, 'external')->{index} = '../escape.md';
    save_registry($fixture);
});
expect_case('external membership rejects an index inside the member surface', 0, qr/must be outside the surface/, sub {
    my ($fixture) = @_;
    surface($fixture, 'external')->{index} = 'external/part.md';
    save_registry($fixture);
});
expect_case('routed membership rejects a member no declared hop reaches', 0, qr/does not link member 'routed\/part\.md'/, sub {
    my ($fixture) = @_;
    write_text($fixture->{root}, 'routed-titles/titles-0001.md', "# Routed titles part 0001\n");
});
expect_case('routed membership rejects an unrouted route member', 0, qr/does not link route member 'routed-titles\/titles-0001\.md'/, sub {
    my ($fixture) = @_;
    write_text($fixture->{root}, 'routed/INDEX.md', "# Routed landing\n[Collection README](README.md)\n");
});
expect_case('routed membership rejects a missing route surface declaration', 0, qr/routed_membership lacks a route_surface/, sub {
    my ($fixture) = @_;
    delete surface($fixture, 'routed')->{index_contract}{route_surface};
    save_registry($fixture);
});
expect_case('routed membership rejects an unregistered route surface', 0, qr/route_surface 'absent_titles' is not a registered surface/, sub {
    my ($fixture) = @_;
    surface($fixture, 'routed')->{index_contract}{route_surface} = 'absent_titles';
    save_registry($fixture);
});
expect_case('routed membership rejects a self-referencing route surface', 0, qr/route_surface must name another surface/, sub {
    my ($fixture) = @_;
    surface($fixture, 'routed')->{index_contract}{route_surface} = 'routed';
    save_registry($fixture);
});
expect_case('routed membership rejects a chained route surface', 0, qr/route_surface 'routed_titles' must not itself route/, sub {
    my ($fixture) = @_;
    surface($fixture, 'routed_titles')->{index_contract} = {
        kind => 'routed_membership',
        verifier => 'builtin:markdown_links',
        route_surface => 'routed',
    };
    save_registry($fixture);
});
# A routed index that sits outside its surface is no longer refused for that alone — it may, when the index is
# itself a classified surface (LIVE-DOCUMENT-PRESSURE-HEADROOM.2c). It still has to prove membership, so the
# case that used to assert the location rule now asserts the rule that actually protects the reader.
expect_case('a routed membership index outside the surface still proves membership',
  0, qr/index 'snapshot\.md' does not link route member 'routed-titles\/titles-0001\.md'/, sub {
    my ($fixture) = @_;
    surface($fixture, 'routed')->{index} = 'snapshot.md';
    save_registry($fixture);
});
expect_case('route_surface is rejected on a direct membership contract', 0, qr/route_surface is only valid for routed_membership/, sub {
    my ($fixture) = @_;
    surface($fixture, 'canonical')->{index_contract}{route_surface} = 'routed_titles';
    save_registry($fixture);
});
expect_case('generated projection rejects a failing freshness verifier', 0, qr/freshness verifier .* failed/, sub {
    my ($fixture) = @_;
    write_text($fixture->{root}, 'scripts/freshness-ok.sh', "#!/usr/bin/env bash\nexit 1\n", 0755);
});
expect_case('generated projection rejects missing canonical inputs', 0, qr/must name canonical_inputs/, sub {
    my ($fixture) = @_;
    delete surface($fixture, 'projection')->{canonical_inputs};
    save_registry($fixture);
});
expect_case('generated projection rejects a missing freshness proof', 0, qr/freshness verifier .* missing or non-executable/, sub {
    my ($fixture) = @_;
    chmod 0644, path_in($fixture->{root}, 'scripts/freshness-ok.sh');
});
expect_case('generated projection collection accepts bounded indexed shards', 1, qr/11 governed surfaces/, sub {
    my ($fixture) = @_;
    write_text($fixture->{root}, 'generated.md', "# Generated index\n[Shard](generated-shard.md)\n");
    write_text($fixture->{root}, 'generated-shard.md', "# Generated shard\n");
    my $surface = surface($fixture, 'projection');
    $surface->{targets} = ['generated*.md'];
    $surface->{locator} = 'collection';
    $surface->{index} = 'generated.md';
    $surface->{index_contract} = { kind => 'membership', verifier => 'builtin:markdown_links' };
    save_registry($fixture);
});
expect_case('generated projection collection rejects a stale shard index', 0, qr/does not link member 'generated-shard\.md'/, sub {
    my ($fixture) = @_;
    write_text($fixture->{root}, 'generated-shard.md', "# Generated shard\n");
    my $surface = surface($fixture, 'projection');
    $surface->{targets} = ['generated*.md'];
    $surface->{locator} = 'collection';
    $surface->{index} = 'generated.md';
    $surface->{index_contract} = { kind => 'membership', verifier => 'builtin:markdown_links' };
    save_registry($fixture);
});
expect_case('archive terminal rejects a nonterminal state', 0, qr/archive terminal must have state=terminal/, sub {
    my ($fixture) = @_;
    surface($fixture, 'archive')->{state} = 'normal';
    save_registry($fixture);
});
expect_case('archive terminal rejects a missing manifest', 0, qr/archive manifest .* is missing/, sub {
    my ($fixture) = @_;
    surface($fixture, 'archive')->{archive_manifest} = 'control/missing-manifest.json';
    save_registry($fixture);
});
expect_case('frozen legacy rejects content drift', 0, qr/frozen hash changed/, sub {
    my ($fixture) = @_;
    write_text($fixture->{root}, 'frozen.md', "changed\n");
});
expect_case('maintained reference rejects stale aggregate authority', 0, qr/aggregate authority is stale for lines_total/, sub {
    my ($fixture) = @_;
    surface($fixture, 'maintained')->{reference_contract}{aggregate_change}{delta}{lines_total} = 1;
    save_registry($fixture);
});
expect_case('currency verifier failure fails closed', 0, qr/currency verifier .* failed/, sub {
    my ($fixture) = @_;
    write_text($fixture->{root}, 'scripts/currency-ok.sh', "#!/usr/bin/env bash\nexit 1\n", 0755);
});
expect_case('transition debt rejects baseline plus allowance overflow', 0, qr/exceeds immutable baseline plus transition allowance for lines_each/, sub {
    my ($fixture) = @_;
    write_text($fixture->{root}, 'ledger.md', "entry\nsecond\n");
});

# LIVE-DOCUMENT-PRESSURE-HEADROOM.22 — a registry bounded its own size with an unconditional error and
# nothing below it, so it was silent right up to the stop. These six cases hold the band in both
# directions: it must fire, it must escalate, it must cover the byte dimension, it must stay quiet below
# itself, and a registry must not be able to opt out of declaring one.
expect_case('registry without milestones fails closed', 0, qr/registry record must declare milestones/, sub {
    my ($fixture) = @_;
    delete $fixture->{registry_meta}{milestones};
    save_registry($fixture);
});
expect_case('registry rejects inverted milestones', 0, qr/invalid warning\/rollover milestones/, sub {
    my ($fixture) = @_;
    $fixture->{registry_meta}{milestones} = { warning_pct => 90, rollover_pct => 80 };
    save_registry($fixture);
});
expect_case('registry record pressure warns below its hard stop', 1, qr/surface registry records is at or above warning/, sub {
    my ($fixture) = @_;
    # 11 records against 13 is 84.6% — inside the band, still legal.
    $fixture->{registry_meta}{max_records} = 13;
    save_registry($fixture);
});
expect_case('registry record pressure escalates to rollover', 1, qr/surface registry records is at or above rollover \(91\.7%\) — 1 below its 12 max_records/, sub {
    my ($fixture) = @_;
    $fixture->{registry_meta}{max_records} = 12;
    save_registry($fixture);
});
expect_case('registry byte pressure warns on its own dimension', 1, qr/surface registry bytes is at or above rollover/, sub {
    my ($fixture) = @_;
    # Derive the bound from the file the fixture actually wrote, so the case measures pressure
    # rather than a number copied into it.
    save_registry($fixture);
    my $written = -s path_in($fixture->{root}, 'control/surfaces.jsonl');
    $fixture->{registry_meta}{max_bytes} = $written + 64;
    save_registry($fixture);
});
expect_absent_case('registry below its band reports no pressure', qr/surface registry records is at or above/, undef);

expect_case('registry rejects record-count overflow', 0, qr/more records than its declared max_records/, sub {
    my ($fixture) = @_;
    $fixture->{registry_meta}{max_records} = 7;
    save_registry($fixture);
});
expect_case('registry rejects total-byte overflow', 0, qr/exceeds its declared max_bytes/, sub {
    my ($fixture) = @_;
    $fixture->{registry_meta}{max_bytes} = 512;
    save_registry($fixture);
});
expect_case('registry rejects raw-record overflow', 0, qr/record above its declared max_record_bytes/, sub {
    my ($fixture) = @_;
    $fixture->{registry_meta}{max_record_bytes} = 256;
    save_registry($fixture);
});
expect_case('registry rejects array-cardinality overflow', 0, qr/more than 1 array items/, sub {
    my ($fixture) = @_;
    $fixture->{registry_meta}{max_array_items} = 1;
    push @{ surface($fixture, 'snapshot')->{targets} }, 'snapshot.md';
    save_registry($fixture);
});
expect_case('registry rejects scalar-byte overflow', 0, qr/exceeds the declared scalar byte limit 8/, sub {
    my ($fixture) = @_;
    $fixture->{registry_meta}{max_scalar_bytes} = 8;
    save_registry($fixture);
});
expect_case('registry rejects an unknown schema version', 0, qr/schema_version must be 1/, sub {
    my ($fixture) = @_;
    $fixture->{registry_meta}{schema_version} = 2;
    save_registry($fixture);
});
expect_case('registry rejects unknown fields', 0, qr/unknown field 'unreviewed_escape'/, sub {
    my ($fixture) = @_;
    surface($fixture, 'snapshot')->{unreviewed_escape} = 1;
    save_registry($fixture);
});
expect_case('registry rejects invalid identifier shape', 0, qr/invalid identifier shape/, sub {
    my ($fixture) = @_;
    surface($fixture, 'snapshot')->{surface_id} = 'Invalid Surface';
    save_registry($fixture);
});
expect_case('registry rejects unknown locator domain', 0, qr/unknown locator 'somewhere'/, sub {
    my ($fixture) = @_;
    surface($fixture, 'snapshot')->{locator} = 'somewhere';
    save_registry($fixture);
});
expect_case('registry rejects unknown lifecycle domain', 0, qr/unknown lifecycle 'append_forever'/, sub {
    my ($fixture) = @_;
    surface($fixture, 'snapshot')->{lifecycle} = 'append_forever';
    save_registry($fixture);
});

expect_case('coverage rejects an unclassified Markdown file', 0, qr/tracked Markdown 'orphan\.md' is not classified/, sub {
    my ($fixture) = @_;
    write_text($fixture->{root}, 'orphan.md', "# Orphan\n");
});
expect_case('coverage rejects a double-classified Markdown file', 0, qr/tracked Markdown 'snapshot\.md' is classified more than once/, sub {
    my ($fixture) = @_;
    push @{ surface($fixture, 'ledger')->{targets} }, 'snapshot.md';
    save_registry($fixture);
});
expect_case('coverage rejects a file locator over several paths', 0, qr/surface 'snapshot' declares a file locator but matches 2 paths/, sub {
    my ($fixture) = @_;
    write_text($fixture->{root}, 'snapshot-extra.md', "# Extra\n[External part](external/part.md)\n");
    push @{ surface($fixture, 'snapshot')->{targets} }, 'snapshot-extra.md';
    save_registry($fixture);
});
expect_case('a multi-file surface reports its aggregate pressure', 1, qr/surface 'canonical' lines_total is at or above rollover/, sub {
    my ($fixture) = @_;
    # A collection near its aggregate line target must warn, not stay silent until the hard ceiling. The
    # health band is tightened as a whole so it stays reachable — files times per-file equals the total —
    # because an aggregate below its own legal maximum is now itself a breach (ADR 0032).
    write_text($fixture->{root}, 'canonical/part.md', "# Canonical part\n" x 90);
    my $health = surface($fixture, 'canonical')->{health_targets};
    @{$health}{qw(files lines_each lines_total bytes_each bytes_total)} = (2, 50, 100, 4_096, 8_192);
    save_registry($fixture);
});
expect_case(
    'a single-file surface stays exempt from duplicate aggregate warnings',
    1, qr/\A(?:(?!surface 'snapshot' lines_total).)*\z/s,
    sub {
        my ($fixture) = @_;
        # snapshot.md is two lines against a two-line aggregate target: 100% of health, and silent
        # only because the surface genuinely holds one file, where lines_total repeats lines_each.
        surface($fixture, 'snapshot')->{health_targets}{lines_total} = 2;
        save_registry($fixture);
    },
);
expect_case('pressure names the distance to the hard ceiling, not only the health percentage',
    1, qr/lines_each is at or above rollover \(\d+\.\d%\) — \d+ below its 100 ceiling/, sub {
        my ($fixture) = @_;
        # A surface past its health target reports a percentage of a number it already blew; the actionable
        # fact is how many lines remain before the enforcement ceiling stops the next unrelated change.
        write_text($fixture->{root}, 'canonical/part.md', "# Canonical part\n" x 95);
        surface($fixture, 'canonical')->{health_targets}{lines_each} = 50;
        save_registry($fixture);
    });
expect_case('a collection aggregate below its own legal maximum is rejected',
    0, qr/lines_total 500 is below its own legal maximum 16 x 100/, sub {
        my ($fixture) = @_;
        surface($fixture, 'canonical')->{enforcement_ceilings}{lines_total} = 500;
        save_registry($fixture);
    });
expect_case('a collection byte aggregate below its own legal maximum is rejected',
    0, qr/bytes_total 16384 is below its own legal maximum 16 x 4096/, sub {
        my ($fixture) = @_;
        surface($fixture, 'canonical')->{enforcement_ceilings}{bytes_total} = 16_384;
        save_registry($fixture);
    });
expect_case('a tight health band is rejected even when the ceiling is reachable',
    0, qr/health_targets lines_total 500 is below its own legal maximum/, sub {
        my ($fixture) = @_;
        surface($fixture, 'canonical')->{health_targets}{lines_total} = 500;
        save_registry($fixture);
    });
expect_case('a file-locator surface is exempt from the collection reachability rule', 1, undef, sub {
        my ($fixture) = @_;
        # A file locator holds one document, so its aggregate merely repeats its per-file bound.
        surface($fixture, 'snapshot')->{enforcement_ceilings}{lines_total} = 5;
        surface($fixture, 'snapshot')->{health_targets}{lines_total} = 5;
        save_registry($fixture);
    });
expect_case('an exact aggregate composition permits a heterogeneous collection', 1, undef, sub {
        my ($fixture) = @_;
        my $canonical = surface($fixture, 'canonical');
        for my $band (qw(health_targets enforcement_ceilings)) {
            @{ $canonical->{$band} }{qw(files lines_each bytes_each lines_total bytes_total)} =
                (16, 100, 4_096, 1_510, 61_952);
        }
        $canonical->{aggregate_composition} = {
            rationale => 'one small index plus fifteen full members',
            members => [
                { role => 'index', count => 1,
                  health => { lines => 10, bytes => 512 }, ceiling => { lines => 10, bytes => 512 } },
                { role => 'member', count => 15,
                  health => { lines => 100, bytes => 4_096 }, ceiling => { lines => 100, bytes => 4_096 } },
            ],
        };
        save_registry($fixture);
    });
expect_case('an exact aggregate composition permits different health and ceiling counts', 1, undef, sub {
        my ($fixture) = @_;
        my $canonical = surface($fixture, 'canonical');
        @{ $canonical->{health_targets} }{qw(files lines_each bytes_each lines_total bytes_total)} =
            (16, 640, 86_016, 6_400, 786_432);
        @{ $canonical->{enforcement_ceilings} }{qw(files lines_each bytes_each lines_total bytes_total)} =
            (24, 896, 98_304, 9_600, 1_179_648);
        $canonical->{aggregate_composition} = {
            rationale => 'eight initial semantic parts plus a larger ceiling continuation pool',
            members => [
                { role => 'semantic', count => { health => 8, ceiling => 8 },
                  health => { lines => 640, bytes => 86_016 },
                  ceiling => { lines => 896, bytes => 98_304 } },
                { role => 'continuation', count => { health => 8, ceiling => 16 },
                  health => { lines => 160, bytes => 12_288 },
                  ceiling => { lines => 152, bytes => 24_576 } },
            ],
        };
        save_registry($fixture);
    });
expect_case('a band-specific aggregate composition requires both counts',
    0, qr/aggregate_composition member 'continuation' count lacks a positive 'ceiling'/, sub {
        my ($fixture) = @_;
        my $canonical = surface($fixture, 'canonical');
        $canonical->{aggregate_composition} = {
            rationale => 'a partial band count cannot prove either aggregate',
            members => [
                { role => 'index', count => 1,
                  health => { lines => 10, bytes => 512 }, ceiling => { lines => 10, bytes => 512 } },
                { role => 'continuation', count => { health => 15 },
                  health => { lines => 100, bytes => 4_096 }, ceiling => { lines => 100, bytes => 4_096 } },
            ],
        };
        save_registry($fixture);
    });
expect_case('a band-specific aggregate composition rejects an unknown count band',
    0, qr/aggregate_composition member 'continuation' count has unknown field 'future'/, sub {
        my ($fixture) = @_;
        my $canonical = surface($fixture, 'canonical');
        $canonical->{aggregate_composition} = {
            rationale => 'count bands are a closed schema',
            members => [
                { role => 'index', count => 1,
                  health => { lines => 10, bytes => 512 }, ceiling => { lines => 10, bytes => 512 } },
                { role => 'continuation', count => { health => 15, ceiling => 15, future => 15 },
                  health => { lines => 100, bytes => 4_096 }, ceiling => { lines => 100, bytes => 4_096 } },
            ],
        };
        save_registry($fixture);
    });
expect_case('an aggregate composition that does not sum to the declared total is rejected',
    0, qr/aggregate_composition ceiling lines sum to 1510, not lines_total 1509/, sub {
        my ($fixture) = @_;
        my $canonical = surface($fixture, 'canonical');
        for my $band (qw(health_targets enforcement_ceilings)) {
            @{ $canonical->{$band} }{qw(files lines_each bytes_each lines_total bytes_total)} =
                (16, 100, 4_096, 1_509, 61_952);
        }
        $canonical->{aggregate_composition} = {
            rationale => 'one small index plus fifteen full members',
            members => [
                { role => 'index', count => 1,
                  health => { lines => 10, bytes => 512 }, ceiling => { lines => 10, bytes => 512 } },
                { role => 'member', count => 15,
                  health => { lines => 100, bytes => 4_096 }, ceiling => { lines => 100, bytes => 4_096 } },
            ],
        };
        save_registry($fixture);
    });
expect_case('an aggregate composition whose counts miss the file bound is rejected',
    0, qr/aggregate_composition counts sum to 15, not the enforcement_ceilings files bound 16/, sub {
        my ($fixture) = @_;
        my $canonical = surface($fixture, 'canonical');
        for my $band (qw(health_targets enforcement_ceilings)) {
            @{ $canonical->{$band} }{qw(files lines_each bytes_each lines_total bytes_total)} =
                (16, 100, 4_096, 1_410, 57_856);
        }
        $canonical->{aggregate_composition} = {
            rationale => 'fourteen full members and one index, one member short',
            members => [
                { role => 'index', count => 1,
                  health => { lines => 10, bytes => 512 }, ceiling => { lines => 10, bytes => 512 } },
                { role => 'member', count => 14,
                  health => { lines => 100, bytes => 4_096 }, ceiling => { lines => 100, bytes => 4_096 } },
            ],
        };
        save_registry($fixture);
    });
expect_case('an aggregate composition whose largest member misses the per-file bound is rejected',
    0, qr/aggregate_composition largest ceiling member is 90 lines, not lines_each 100/, sub {
        my ($fixture) = @_;
        my $canonical = surface($fixture, 'canonical');
        for my $band (qw(health_targets enforcement_ceilings)) {
            @{ $canonical->{$band} }{qw(files lines_each bytes_each lines_total bytes_total)} =
                (16, 100, 4_096, 1_360, 61_952);
        }
        $canonical->{aggregate_composition} = {
            rationale => 'every member below the declared per-file bound',
            members => [
                { role => 'index', count => 1,
                  health => { lines => 10, bytes => 512 }, ceiling => { lines => 10, bytes => 512 } },
                { role => 'member', count => 15,
                  health => { lines => 90, bytes => 4_096 }, ceiling => { lines => 90, bytes => 4_096 } },
            ],
        };
        save_registry($fixture);
    });
expect_case('a single-role aggregate composition is rejected',
    0, qr/aggregate_composition members must list at least two member roles/, sub {
        my ($fixture) = @_;
        surface($fixture, 'canonical')->{aggregate_composition} = {
            rationale => 'a homogeneous collection needs no exemption',
            members => [
                { role => 'member', count => 16,
                  health => { lines => 100, bytes => 4_096 }, ceiling => { lines => 100, bytes => 4_096 } },
            ],
        };
        save_registry($fixture);
    });
expect_case('an aggregate composition without a rationale is rejected',
    0, qr/aggregate_composition lacks a nonempty rationale/, sub {
        my ($fixture) = @_;
        surface($fixture, 'canonical')->{aggregate_composition} = {
            members => [
                { role => 'index', count => 1,
                  health => { lines => 10, bytes => 512 }, ceiling => { lines => 10, bytes => 512 } },
                { role => 'member', count => 15,
                  health => { lines => 100, bytes => 4_096 }, ceiling => { lines => 100, bytes => 4_096 } },
            ],
        };
        save_registry($fixture);
    });
expect_case('an aggregate composition with a repeated role is rejected',
    0, qr/aggregate_composition repeats member role 'member'/, sub {
        my ($fixture) = @_;
        surface($fixture, 'canonical')->{aggregate_composition} = {
            rationale => 'duplicated roles hide which members were counted',
            members => [
                { role => 'member', count => 1,
                  health => { lines => 10, bytes => 512 }, ceiling => { lines => 10, bytes => 512 } },
                { role => 'member', count => 15,
                  health => { lines => 100, bytes => 4_096 }, ceiling => { lines => 100, bytes => 4_096 } },
            ],
        };
        save_registry($fixture);
    });
expect_case('an aggregate composition with an unknown field is rejected',
    0, qr/aggregate_composition has unknown field 'note'/, sub {
        my ($fixture) = @_;
        surface($fixture, 'canonical')->{aggregate_composition} = {
            rationale => 'unknown control-plane fields fail closed',
            note => 'x',
            members => [
                { role => 'index', count => 1,
                  health => { lines => 10, bytes => 512 }, ceiling => { lines => 10, bytes => 512 } },
                { role => 'member', count => 15,
                  health => { lines => 100, bytes => 4_096 }, ceiling => { lines => 100, bytes => 4_096 } },
            ],
        };
        save_registry($fixture);
    });
expect_case('an aggregate composition member missing a band is rejected',
    0, qr/aggregate_composition member 'member' lacks health bounds/, sub {
        my ($fixture) = @_;
        surface($fixture, 'canonical')->{aggregate_composition} = {
            rationale => 'both bands must be partitioned, not just the ceiling',
            members => [
                { role => 'index', count => 1,
                  health => { lines => 10, bytes => 512 }, ceiling => { lines => 10, bytes => 512 } },
                { role => 'member', count => 15, ceiling => { lines => 100, bytes => 4_096 } },
            ],
        };
        save_registry($fixture);
    });
expect_case('coverage rejects an off-root target', 0, qr/absolute, parent-relative, or off-root/, sub {
    my ($fixture) = @_;
    surface($fixture, 'snapshot')->{targets} = ['../escape.md'];
    save_registry($fixture);
});
expect_case('membership rejects a missing index', 0, qr/membership index .* is missing/, sub {
    my ($fixture) = @_;
    surface($fixture, 'canonical')->{index} = 'canonical/MISSING.md';
    save_registry($fixture);
});
expect_case('route integration rejects an unclassified Markdown target', 0, qr/README route 'missing\.md' is not a classified Markdown surface/, sub {
    my ($fixture) = @_;
    write_text(
        $fixture->{root},
        'control/routes.tsv',
        "target\troute_kind\towner\tlifecycle\tpressure_control\tterminal\n" .
        "missing.md\treader_navigation\tfixture\tmaintained_entrypoint\treview_on_behavior_change\tyes\n",
    );
});

for my $dimension (@dimensions) {
    expect_case("independent $dimension ceiling fails closed", 0, qr/exceeds \Q$dimension\E ceiling/, sub {
        my ($fixture) = @_;
        surface($fixture, 'snapshot')->{enforcement_ceilings}{$dimension} = 0;
        save_registry($fixture);
    });
}

expect_history_case('ceiling increase rejects missing exact authority', 0, qr/increased ceiling dimensions without exact authority: bytes_each/, sub {
    my ($fixture) = @_;
    surface($fixture, 'snapshot')->{enforcement_ceilings}{bytes_each}++;
    save_registry($fixture);
});
expect_history_case('ceiling increase accepts one exact fresh authority', 1, qr/11 governed surfaces/, sub {
    my ($fixture) = @_;
    my $surface = surface($fixture, 'snapshot');
    my $old = { %{ $surface->{enforcement_ceilings} } };
    $surface->{enforcement_ceilings}{bytes_each}++;
    my $new = { %{ $surface->{enforcement_ceilings} } };
    $fixture->{authorities} = [{
        record_type => 'increase',
        surface_id => 'snapshot',
        work_unit => 'fixture-ceiling-increase',
        owner => 'fixture-maintainers',
        rationale => 'Exercise exact one-change ceiling authority.',
        old => $old,
        new => $new,
    }];
    save_registry($fixture);
    save_authorities($fixture);
});
expect_history_case('ceiling history rejects unused banked authority', 0, qr/unused or banked ceiling-increase authority/, sub {
    my ($fixture) = @_;
    my $ceilings = surface($fixture, 'snapshot')->{enforcement_ceilings};
    $fixture->{authorities} = [{
        record_type => 'increase',
        surface_id => 'snapshot',
        work_unit => 'fixture-banked-authority',
        owner => 'fixture-maintainers',
        rationale => 'This unchanged authority must be rejected as banked.',
        old => { %$ceilings },
        new => { %$ceilings },
    }];
    save_authorities($fixture);
});
expect_history_case('maintained reference rejects reused aggregate authority', 0, qr/reused maintained-reference authority across aggregate change/, sub {
    my ($fixture) = @_;
    write_text($fixture->{root}, 'book/part.md', "# Maintained part\nextra\n");
    my $metrics = dimensions_for($fixture->{root}, 'book/SUMMARY.md', 'book/part.md');
    my $change = surface($fixture, 'maintained')->{reference_contract}{aggregate_change};
    $change->{delta}{lines_total} = $metrics->{lines_total} - $change->{baseline}{lines_total};
    $change->{delta}{bytes_total} = $metrics->{bytes_total} - $change->{baseline}{bytes_total};
    save_registry($fixture);
});
expect_history_case('maintained reference accepts fresh exact aggregate authority', 1, qr/11 governed surfaces/, sub {
    my ($fixture) = @_;
    write_text($fixture->{root}, 'book/part.md', "# Maintained part\nextra\n");
    my $metrics = dimensions_for($fixture->{root}, 'book/SUMMARY.md', 'book/part.md');
    my $change = surface($fixture, 'maintained')->{reference_contract}{aggregate_change};
    $change->{authority_id} = 'fixture-maintained-reference-next';
    $change->{delta}{lines_total} = $metrics->{lines_total} - $change->{baseline}{lines_total};
    $change->{delta}{bytes_total} = $metrics->{bytes_total} - $change->{baseline}{bytes_total};
    save_registry($fixture);
});
expect_history_case('transition debt rejects a moved immutable baseline', 0, qr/moved its immutable transition baseline/, sub {
    my ($fixture) = @_;
    surface($fixture, 'ledger')->{baseline}{bytes_each}++;
    surface($fixture, 'ledger')->{transition}{max_growth}{bytes_each} = 1;
    save_registry($fixture);
});

# Cardinality exemption (ADR 0045). A surface may drop its file-count bound, but only behind a declaration
# whose conditions are checked; a bare null must stay refused, or the exemption is a way to opt out of every
# cardinality control by editing one field. `routed_titles` is the fixture analogue of `task_evidence`: its
# declared index lives on a different surface, exactly as docs/TASK_TREE.md does.
sub declare_exemption {
    my ($fixture, %override) = @_;
    my $surface = surface($fixture, 'routed_titles');
    $surface->{health_targets}{files} = undef;
    $surface->{enforcement_ceilings}{files} = undef;
    $surface->{cardinality_exemption} = {
        authority => 'routed/README.md',
        work_unit => 'FIXTURE-EXEMPTION.0',
        route_surface_id => 'routed',
        rationale => 'fixture exemption',
        %override,
    };
    return $surface;
}

expect_case('a declared cardinality exemption with a bounded external route is accepted', 1, undef, sub {
    my ($fixture) = @_;
    declare_exemption($fixture);
    save_registry($fixture);
});
expect_case('a null file count without a declared exemption is rejected',
  0, qr/nulls files without a declared cardinality exemption/, sub {
    my ($fixture) = @_;
    my $surface = surface($fixture, 'routed_titles');
    $surface->{health_targets}{files} = undef;
    $surface->{enforcement_ceilings}{files} = undef;
    save_registry($fixture);
});
expect_case('a half-declared cardinality exemption is rejected',
  0, qr/must null files in both bands together/, sub {
    my ($fixture) = @_;
    my $surface = declare_exemption($fixture);
    $surface->{enforcement_ceilings}{files} = 16;
    save_registry($fixture);
});
expect_case('a cardinality exemption may not unbound a resource dimension',
  0, qr/may not unbound resource dimension 'lines_total'/, sub {
    my ($fixture) = @_;
    my $surface = declare_exemption($fixture);
    $surface->{health_targets}{lines_total} = undef;
    $surface->{enforcement_ceilings}{lines_total} = undef;
    save_registry($fixture);
});
expect_case('a cardinality exemption routed to an unregistered surface is rejected',
  0, qr/route surface 'absent_route' is not registered/, sub {
    my ($fixture) = @_;
    declare_exemption($fixture, route_surface_id => 'absent_route');
    save_registry($fixture);
});
expect_case('a cardinality exemption routed to an unbounded surface is rejected',
  0, qr/route 'maintained' is unbounded in 'files'/, sub {
    my ($fixture) = @_;
    declare_exemption($fixture, route_surface_id => 'maintained');
    save_registry($fixture);
});
expect_case('a cardinality exemption may not route to the exempt surface itself',
  0, qr/route must be a different registered surface/, sub {
    my ($fixture) = @_;
    declare_exemption($fixture, route_surface_id => 'routed_titles');
    save_registry($fixture);
});
expect_case('a cardinality exemption with an untracked authority is rejected',
  0, qr/authority 'routed\/absent\.md' is not a tracked repository file/, sub {
    my ($fixture) = @_;
    declare_exemption($fixture, authority => 'routed/absent.md');
    save_registry($fixture);
});

# A routed index may live outside its own collection (docs/TASK_TREE.md is not a task tree), but only when
# it is itself a classified surface — otherwise the landing floats outside every bound
# (LIVE-DOCUMENT-PRESSURE-HEADROOM.2c).
expect_case('a routed membership index outside the surface must itself be classified',
  0, qr/routed_membership index 'unclassified\/INDEX\.md' is outside the surface and is not a classified Markdown surface/, sub {
    my ($fixture) = @_;
    write_text($fixture->{root}, 'unclassified/INDEX.md', "# Floating landing\n");
    my $surface = surface($fixture, 'routed');
    $surface->{index} = 'unclassified/INDEX.md';
    save_registry($fixture);
});

print "1..$test_number\n" if !$quiet;
if ($failures) {
    print STDERR "live-document-size-tests: $failures of $test_number checks failed\n";
    exit 1;
}
# PRODUCTION-GRAPH-CENSUS-PIN.3 — `all $test_number ... pass` is a running counter with nothing
# to compare it against: delete a check and the line simply reports one fewer. The expected
# count is declared here, independently of the suite, so a check removed — or one added and not
# declared — fails instead of silently shrinking the coverage this reports.
my $expected_checks = 99;
die "live-document-size-tests: ran $test_number checks, declaration expects $expected_checks — "
    . "re-derive the declaration beside the suite\n"
    if $test_number != $expected_checks;
print STDERR "live-document-size-tests: $test_number/$expected_checks lifecycle and control-plane checks pass\n";
exit 0;
