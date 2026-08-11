#!/usr/bin/env perl
# scripts/check_corpus_frontier_census.pl — the CORPUS-FRONTIER doctrine.
#
# CORPUS-COVERAGE.4.1. "How much of the corpus is still unrefreshed" must be a DERIVED quantity that
# a declaration is checked against, never a number carried from slice to slice.
#
# Why this exists. The `.2` refresh program tracked its own remaining work as prose that each slice
# decremented by one — 27, 26, 25, ... 6, 5 — across twenty-two consecutive refreshes. A decrement
# cannot detect an error at its base, so a denominator adjustment recorded at `.2.29` (for the
# project's own `ingest README.md` artifact, a cohort member that later left `generated/`) outlived
# what it described and silently shortened the queue. `nvme_base_specification_2_0a_2021_07_26` — a
# 454-page specification with no exception recorded against it anywhere — simply stopped being
# counted. `CORPUS-COVERAGE.4.0` re-derived the census and restored it; this check is what stops the
# same class of drift returning.
#
# Archetype: DERIVE-AND-DIFF (DOCTRINE_ENFORCEMENT.md §3). The census is recomputed from persisted
# artifacts and compared with an exact declaration, so a disagreement is a failure rather than a
# discrepancy nobody is looking for.
#
# The census, all three quantities derived from `generated/source_ir/<key>/source_ir.json`:
#   cohort     documents whose `source.requested_path` is NOT under a declared excluded prefix.
#              `corpus/` is the tracked in-repo gold/eval corpus — copied into the repository, never
#              on the host-local library, so it has no retired-volume provenance to refresh.
#   refreshed  cohort minus remaining.
#   remaining  DECLARED in the contract, then checked four ways below.
#
# The four checks, each catching a distinct failure:
#   1. IDENTITY      cohort == refreshed + remaining, and both match the declared `expected`.
#                    Catches a count that drifted from the artifacts in either direction.
#   2. MEMBERSHIP    every declared-remaining key exists, is in the cohort, and has NO retained
#                    normalized bundle. A refreshed document keeps its bundle, so a declared-remaining
#                    key that has one is a stale declaration.
#   3. OMISSION      no cohort member OUTSIDE the declared-remaining set still carries a retired-root
#                    `requested_path` without a retained bundle. This is the check that would have
#                    caught the original defect: a document dropped from the list is invisible to any
#                    check over that list, and only a scan of the whole cohort finds it.
#   4. PROSE         the root task file states exactly the declared counts. The frontier a human or
#                    agent reads and the census the artifacts support cannot say different things.
#
# Check 3 needs a retired-root prefix, which is workstation-shaped and will change again if the
# library moves. That is precisely why it lives in the DECLARATION with an owning leaf rather than in
# this executable: the rule is data an owner revises, not logic. No document, vendor, or protocol
# name participates in any check (ADR 0006) — NVMe is admitted by exactly the rule that admits the
# other five.
#
# Skips LOUDLY when the corpus root is absent: a fresh clone and a hosted CI runner have no
# `generated/`, and the doctrine does not govern them. Silence would read as a pass.
#
# Cost: a bounded prefix read of each SourceIR (the `source` object is the third top-level key), so
# roughly ten milliseconds for the whole corpus — gate-tier, unlike its CHAIN-CURRENCY sibling which
# needs a build and a full replay.
#
# Knobs: SPECFORGE_CORPUS_FRONTIER_CONTRACT=<file> to point at another declaration.
#        SPECFORGE_CORPUS_FRONTIER_ROOT=<dir>      to point at another corpus root.
#        Both are used by --self-test to exercise the core against fixtures.
use strict;
use warnings;
use JSON::PP;
use File::Basename qw(dirname);
use File::Path qw(make_path remove_tree);
use File::Temp qw(tempdir);

binmode STDOUT, ':encoding(UTF-8)';
binmode STDERR, ':encoding(UTF-8)';

my $DEFAULT_CONTRACT = 'doctrine/corpus_frontier/census.json';
my $PREFIX_BYTES     = 8192;
my $MAX_PREFIX_BYTES = 4 * 1024 * 1024;

my $root;
my $contract_rel = $ENV{SPECFORGE_CORPUS_FRONTIER_CONTRACT} // $DEFAULT_CONTRACT;
my $self_test    = 0;
my $report       = 0;

while (@ARGV) {
    my $arg = shift @ARGV;
    if    ($arg eq '--root')      { $root = shift @ARGV // usage() }
    elsif ($arg eq '--contract')  { $contract_rel = shift @ARGV // usage() }
    elsif ($arg eq '--self-test') { $self_test = 1 }
    elsif ($arg eq '--report')    { $report = 1 }
    elsif ($arg eq '--check')     { }
    else                          { usage() }
}
$root //= repository_root();

if ($self_test) { exit(self_test() ? 0 : 1) }

my @errors;
my $result = run_census($root, $contract_rel, \@errors);
if (@errors) {
    print STDERR "corpus-frontier-census: $_\n" for @errors;
    print STDERR "corpus-frontier-census: FAILED with " . scalar(@errors) . " violation(s).\n";
    exit 1;
}
if (!defined $result) {
    print "corpus-frontier-census: SKIPPED - no corpus root; the derivation does not govern this tree.\n";
    exit 0;
}
if ($report) {
    print JSON::PP->new->canonical(1)->pretty(1)->encode($result);
    exit 0;
}
printf
    "corpus-frontier-census: %d cohort = %d refreshed + %d remaining; declaration, retention, and the root frontier agree.\n",
    $result->{cohort}, $result->{refreshed}, $result->{remaining};
exit 0;

sub usage {
    print STDERR "usage: check_corpus_frontier_census.pl [--root DIR] [--contract FILE] "
        . "[--check|--report|--self-test]\n";
    exit 2;
}

sub repository_root {
    my $dir = dirname(__FILE__) . '/..';
    my $abs = eval { require Cwd; Cwd::abs_path($dir) } // $dir;
    return $abs;
}

sub problem { my ($errors, $message) = @_; push @$errors, $message; return }

# ---------------------------------------------------------------------------
# Reading one document's `source` object without parsing the whole artifact.
#
# A SourceIR is up to ~800 KB and only its `source` object is needed, so a bounded prefix is read and
# the object is located by an exact brace scan that tracks string and escape state. A naive scan
# would miscount a brace inside a path string; guessing the value with a regex would be worse. If the
# object is not complete within the prefix the read grows rather than giving up, so correctness never
# depends on the artifact's formatting.
# ---------------------------------------------------------------------------
sub extract_source_object {
    my ($path, $errors) = @_;
    my $size = -s $path;
    return undef if !defined $size;
    my $want = $PREFIX_BYTES;
    while (1) {
        $want = $size if $want > $size;
        open my $fh, '<:raw', $path or do {
            problem($errors, "cannot read '$path': $!");
            return undef;
        };
        read($fh, my $buffer, $want);
        close $fh;
        my $object = slice_json_object($buffer, '"source"');
        return $object if defined $object;
        last if $want >= $size || $want >= $MAX_PREFIX_BYTES;
        $want *= 2;
    }
    problem($errors, "'$path' has no complete top-level \"source\" object");
    return undef;
}

sub slice_json_object {
    my ($buffer, $key) = @_;
    my $at = index($buffer, $key);
    return undef if $at < 0;
    my $open = index($buffer, '{', $at + length($key));
    return undef if $open < 0;
    my ($depth, $in_string, $escaped) = (0, 0, 0);
    for my $i ($open .. length($buffer) - 1) {
        my $c = substr($buffer, $i, 1);
        if ($in_string) {
            if    ($escaped)     { $escaped = 0 }
            elsif ($c eq "\\")   { $escaped = 1 }
            elsif ($c eq '"')    { $in_string = 0 }
            next;
        }
        if    ($c eq '"') { $in_string = 1 }
        elsif ($c eq '{') { $depth++ }
        elsif ($c eq '}') {
            $depth--;
            return substr($buffer, $open, $i - $open + 1) if $depth == 0;
        }
    }
    return undef;
}

sub read_json_file {
    my ($path, $label, $errors) = @_;
    open my $fh, '<:raw', $path or do {
        problem($errors, "$label is missing or unreadable: $path");
        return undef;
    };
    local $/;
    my $raw = <$fh>;
    close $fh;
    my $decoded = eval { JSON::PP->new->decode($raw) };
    if (!defined $decoded) {
        problem($errors, "$label is not valid JSON: $path");
        return undef;
    }
    return $decoded;
}

sub has_prefix {
    my ($value, $prefixes) = @_;
    for my $prefix (@$prefixes) {
        return 1 if index($value, $prefix) == 0;
    }
    return 0;
}

sub validate_contract {
    my ($contract, $errors) = @_;
    for my $field (qw(schema_version contract_id owner_leaf authority declared_on)) {
        problem($errors, "contract field '$field' is missing")
            if !defined $contract->{$field} || ref($contract->{$field});
    }
    my $rule = $contract->{cohort_rule};
    if (ref($rule) ne 'HASH') { problem($errors, 'contract cohort_rule must be an object'); return 0 }
    for my $field (qw(corpus_root artifact_name source_field retention_contract)) {
        problem($errors, "contract cohort_rule.$field is missing")
            if !defined $rule->{$field} || ref($rule->{$field});
    }
    for my $field (qw(excluded_source_prefixes retired_source_prefixes)) {
        problem($errors, "contract cohort_rule.$field must be a non-empty array")
            if ref($rule->{$field}) ne 'ARRAY' || !@{$rule->{$field}};
    }
    my $expected = $contract->{expected};
    if (ref($expected) ne 'HASH') { problem($errors, 'contract expected must be an object'); return 0 }
    for my $field (qw(cohort refreshed)) {
        problem($errors, "contract expected.$field must be a non-negative integer")
            if !defined $expected->{$field} || $expected->{$field} !~ /\A\d+\z/;
    }
    problem($errors, 'contract remaining must be an array')
        if ref($contract->{remaining}) ne 'ARRAY';
    my $claim = $contract->{root_claim};
    if (ref($claim) ne 'HASH') { problem($errors, 'contract root_claim must be an object'); return 0 }
    for my $field (qw(path completed_sentence remaining_phrase)) {
        problem($errors, "contract root_claim.$field is missing")
            if !defined $claim->{$field} || ref($claim->{$field});
    }
    return !@$errors;
}

sub run_census {
    my ($base, $contract_rel, $errors) = @_;
    my $contract = read_json_file("$base/$contract_rel", 'corpus frontier contract', $errors);
    return undef if !defined $contract;
    return undef if !validate_contract($contract, $errors);

    my $rule = $contract->{cohort_rule};
    my $corpus_root = $ENV{SPECFORGE_CORPUS_FRONTIER_ROOT} // "$base/$rule->{corpus_root}";
    if (!-d $corpus_root) { return undef }

    my $retained = read_json_file("$base/$rule->{retention_contract}", 'retention contract', $errors);
    return undef if !defined $retained;
    if (ref($retained->{retained}) ne 'ARRAY') {
        problem($errors, 'retention contract has no retained array');
        return undef;
    }
    my %retained = map { $_ => 1 } @{$retained->{retained}};

    opendir(my $dh, $corpus_root) or do {
        problem($errors, "cannot list corpus root '$corpus_root': $!");
        return undef;
    };
    my @keys = sort grep { $_ !~ /\A\.\.?\z/ && -d "$corpus_root/$_" } readdir($dh);
    closedir $dh;

    my (@cohort, %retired_path);
    for my $key (@keys) {
        my $artifact = "$corpus_root/$key/$rule->{artifact_name}";
        next if !-f $artifact;
        my $raw_object = extract_source_object($artifact, $errors);
        next if !defined $raw_object;
        my $source = eval { JSON::PP->new->decode($raw_object) };
        if (ref($source) ne 'HASH') {
            problem($errors, "'$artifact' has an unreadable \"source\" object");
            next;
        }
        my $requested = $source->{ $rule->{source_field} };
        if (!defined $requested || ref($requested)) {
            problem($errors, "'$artifact' has no scalar source.$rule->{source_field}");
            next;
        }
        next if has_prefix($requested, $rule->{excluded_source_prefixes});
        push @cohort, $key;
        $retired_path{$key} = 1 if has_prefix($requested, $rule->{retired_source_prefixes});
    }
    return undef if @$errors;

    my %cohort = map { $_ => 1 } @cohort;
    my @declared = @{$contract->{remaining}};
    my %declared = map { $_ => 1 } @declared;
    problem($errors, 'contract remaining contains a duplicate key')
        if scalar(keys %declared) != scalar(@declared);

    # 1. IDENTITY
    my $cohort_n    = scalar(@cohort);
    my $remaining_n = scalar(keys %declared);
    my $refreshed_n = $cohort_n - $remaining_n;
    problem($errors, "derived cohort $cohort_n disagrees with declared expected.cohort $contract->{expected}{cohort}")
        if $cohort_n != $contract->{expected}{cohort};
    problem($errors, "derived refreshed $refreshed_n disagrees with declared expected.refreshed $contract->{expected}{refreshed}")
        if $refreshed_n != $contract->{expected}{refreshed};

    # 2. MEMBERSHIP
    for my $key (sort keys %declared) {
        if (!$cohort{$key}) {
            problem($errors, "declared-remaining '$key' is not a cohort document");
            next;
        }
        problem($errors, "declared-remaining '$key' retains a normalized bundle, so it is refreshed")
            if $retained{$key};
    }

    # 3. OMISSION — the check the original defect needed
    for my $key (@cohort) {
        next if $declared{$key};
        next if $retained{$key};
        next if !$retired_path{$key};
        problem($errors,
            "cohort document '$key' is unrefreshed (retired source path, no retained bundle) "
                . 'but is absent from the declared remaining set');
    }

    # 4. PROSE
    my $claim = $contract->{root_claim};
    my $claim_path = "$base/$claim->{path}";
    if (open my $fh, '<:raw', $claim_path) {
        local $/;
        my $text = <$fh>;
        close $fh;
        problem($errors, "root frontier '$claim->{path}' does not state \"$claim->{completed_sentence}\"")
            if index($text, $claim->{completed_sentence}) < 0;
        problem($errors, "root frontier '$claim->{path}' does not state \"$claim->{remaining_phrase}\"")
            if index($text, $claim->{remaining_phrase}) < 0;
    }
    else {
        problem($errors, "root frontier '$claim->{path}' is unreadable: $!");
    }

    return {
        cohort    => $cohort_n,
        refreshed => $refreshed_n,
        remaining => $remaining_n,
        remaining_keys => [sort keys %declared],
    };
}

# ---------------------------------------------------------------------------
# Self-test — every check above is proven to fail closed on a seeded breach, and the whole core is
# proven to pass on a clean fixture. A gate nobody has watched fail is a gate nobody should trust.
# ---------------------------------------------------------------------------
sub self_test {
    my $dir = tempdir(CLEANUP => 1);
    my $passed = 0;
    my $failed = 0;
    my $json = JSON::PP->new->canonical(1)->pretty(1);

    my $write = sub {
        my ($path, $content) = @_;
        make_path(dirname($path));
        open my $fh, '>:raw', $path or die "$path: $!";
        print $fh $content;
        close $fh;
    };

    # A fixture corpus: two in-repo documents outside the cohort, one refreshed by path, one
    # refreshed by retention alone, and two genuinely remaining.
    my $build = sub {
        my (%override) = @_;
        remove_tree("$dir/tree");
        my %docs = (
            gold_a      => 'corpus/vendor/gold_a.pdf',
            gold_b      => 'corpus/vendor/gold_b.pdf',
            moved       => '.cache/local-references/chipdoc/vendor/moved.pdf',
            kept_bundle => '/Users/someone/library/kept_bundle.pdf',
            left_one    => '/Users/someone/library/left_one.pdf',
            left_two    => '/Users/someone/library/left_two.pdf',
            %{ $override{docs} // {} },
        );
        for my $key (sort keys %docs) {
            $write->(
                "$dir/tree/generated/source_ir/$key/source_ir.json",
                $json->encode({
                    schema_version => 1,
                    stage          => 'source_ir',
                    source         => { requested_path => $docs{$key}, canonical_path => $docs{$key} },
                }),
            );
        }
        $write->(
            "$dir/tree/doctrine/chain_currency/retained_bundles.json",
            $json->encode({ retained => $override{retained} // ['kept_bundle'] }),
        );
        my $contract = {
            schema_version => 1,
            contract_id    => 'corpus-frontier-census',
            owner_leaf     => 'FIXTURE.1',
            authority      => 'fixture',
            declared_on    => '2026-08-11',
            cohort_rule    => {
                corpus_root              => 'generated/source_ir',
                artifact_name            => 'source_ir.json',
                source_field             => 'requested_path',
                excluded_source_prefixes => ['corpus/'],
                retired_source_prefixes  => ['/Users/'],
                retention_contract       => 'doctrine/chain_currency/retained_bundles.json',
            },
            expected   => $override{expected}  // { cohort => 4, refreshed => 2 },
            remaining  => $override{remaining} // ['left_one', 'left_two'],
            root_claim => {
                path               => 'docs/tasks/FIXTURE.md',
                completed_sentence => '2 of 4 real chip-spec refreshes are complete.',
                remaining_phrase   => 'two real documents remaining',
            },
        };
        $write->("$dir/tree/doctrine/corpus_frontier/census.json", $json->encode($contract));
        $write->(
            "$dir/tree/docs/tasks/FIXTURE.md",
            $override{root_text}
                // "# fixture\n\n2 of 4 real chip-spec refreshes are complete.\n\nWith two real documents remaining.\n",
        );
    };

    my $case = sub {
        my ($name, $expect_ok, $pattern, %override) = @_;
        $build->(%override);
        my @errors;
        my $result = run_census("$dir/tree", 'doctrine/corpus_frontier/census.json', \@errors);
        my $ok;
        if ($expect_ok) {
            $ok = (!@errors && defined($result) && $result->{cohort} == 4 && $result->{refreshed} == 2);
        }
        else {
            $ok = (grep { $_ =~ $pattern } @errors) ? 1 : 0;
        }
        if ($ok) { $passed++ }
        else {
            $failed++;
            print STDERR "corpus-frontier-census self-test FAILED: $name\n";
            print STDERR "  errors: " . (join('; ', @errors) || '(none)') . "\n";
        }
    };

    $case->('clean fixture passes', 1, undef);
    $case->(
        'a dropped remaining document fails closed', 0,
        qr/absent from the declared remaining set/,
        remaining => ['left_one'], expected => { cohort => 4, refreshed => 3 },
    );
    $case->(
        'an inflated denominator fails closed', 0,
        qr/derived cohort 4 disagrees/,
        expected => { cohort => 5, refreshed => 3 },
    );
    $case->(
        'a refreshed count that does not fit the identity fails closed', 0,
        qr/derived refreshed 2 disagrees/,
        expected => { cohort => 4, refreshed => 1 },
    );
    $case->(
        'a declared-remaining document that kept its bundle fails closed', 0,
        qr/retains a normalized bundle/,
        retained => ['kept_bundle', 'left_one'],
    );
    $case->(
        'a declared-remaining key outside the cohort fails closed', 0,
        qr/is not a cohort document/,
        remaining => ['left_one', 'gold_a'],
    );
    $case->(
        'a root frontier stating another count fails closed', 0,
        qr/does not state/,
        root_text => "# fixture\n\n3 of 4 real chip-spec refreshes are complete.\n\nWith two real documents remaining.\n",
    );
    $case->(
        'a duplicate remaining key fails closed', 0,
        qr/duplicate key/,
        remaining => ['left_one', 'left_one', 'left_two'],
    );

    # The absent-corpus skip, proven end to end rather than assumed.
    $build->();
    remove_tree("$dir/tree/generated/source_ir");
    my @skip_errors;
    my $skipped = run_census("$dir/tree", 'doctrine/corpus_frontier/census.json', \@skip_errors);
    if (!defined($skipped) && !@skip_errors) { $passed++ }
    else {
        $failed++;
        print STDERR "corpus-frontier-census self-test FAILED: absent corpus root must skip, not fail\n";
    }

    # The brace scanner must not be fooled by a brace inside a path string.
    my $tricky = slice_json_object('{"source": {"requested_path": "/Users/a{b}/x.pdf"}, "next": 1}', '"source"');
    if (defined($tricky) && $tricky eq '{"requested_path": "/Users/a{b}/x.pdf"}') { $passed++ }
    else {
        $failed++;
        print STDERR "corpus-frontier-census self-test FAILED: brace scan mishandled a braced string\n";
    }

    my $total = $passed + $failed;
    if ($failed) {
        print STDERR "corpus-frontier-census: self-test $passed/$total passed, $failed FAILED.\n";
        return 0;
    }
    print "corpus-frontier-census: self-test $passed/$total passed.\n";
    return 1;
}
