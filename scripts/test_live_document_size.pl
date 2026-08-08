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
        lines_total => 500,
        bytes_total => 16_384,
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
        },
        surfaces => [
            $snapshot,
            $ledger,
            $canonical,
            $query,
            $external,
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
    'generated_projection',
    'archive_terminal',
    'frozen_legacy',
    'maintained_reference',
) {
    expect_case("positive $lifecycle", 1, qr/9 governed surfaces/, undef);
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
expect_case('generated projection collection accepts bounded indexed shards', 1, qr/9 governed surfaces/, sub {
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
expect_history_case('ceiling increase accepts one exact fresh authority', 1, qr/9 governed surfaces/, sub {
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
expect_history_case('maintained reference accepts fresh exact aggregate authority', 1, qr/9 governed surfaces/, sub {
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

print "1..$test_number\n" if !$quiet;
if ($failures) {
    print STDERR "live-document-size-tests: $failures of $test_number checks failed\n";
    exit 1;
}
print STDERR "live-document-size-tests: all $test_number lifecycle and control-plane checks pass\n";
exit 0;
