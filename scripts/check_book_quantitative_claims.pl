#!/usr/bin/env perl
use strict;
use warnings;

use Cwd qw(abs_path getcwd);
use Digest::SHA qw(sha256_hex);
use File::Basename qw(dirname);
use File::Path qw(make_path remove_tree);
use File::Spec;
use IO::Select;
use IPC::Open3;
use JSON::PP;
use Symbol qw(gensym);

my $script_root = abs_path(File::Spec->catdir(dirname(abs_path($0)), '..'));
my $root;
my $contract_rel = 'doctrine/claim_verification/book_quantitative_claims.jsonl';
my $mode = 'check';

while (@ARGV) {
    my $arg = shift @ARGV;
    if ($arg eq '--root') {
        $root = shift @ARGV // usage();
    } elsif ($arg eq '--contract') {
        $contract_rel = shift @ARGV // usage();
    } elsif ($arg eq '--check') {
        $mode = 'check';
    } elsif ($arg eq '--report') {
        $mode = 'report';
    } elsif ($arg eq '--produce') {
        $mode = 'produce';
    } elsif ($arg eq '--self-test') {
        $mode = 'self-test';
    } else {
        usage();
    }
}

$root //= $script_root;
$root = abs_path($root) // die "book-quantitative-claims: repository root does not exist\n";
run_self_test() if $mode eq 'self-test';

my $result = validate_contract(
    root => $root,
    contract_rel => $contract_rel,
    execute_commands => 1,
);
if (@{$result->{errors}}) {
    print STDERR "book-quantitative-claims: $_\n" for @{$result->{errors}};
    print STDERR "book-quantitative-claims: FAILED with " . scalar(@{$result->{errors}}) . " violation(s).\n";
    exit 1;
}

my $json = JSON::PP->new->canonical(1);
if ($mode eq 'produce') {
    print $json->encode($_), "\n" for @{$result->{candidates}};
} elsif ($mode eq 'report') {
    print $json->encode({
        phase => $result->{phase},
        book_files => scalar(@{$result->{members}}),
        candidate_lines => scalar(@{$result->{candidates}}),
        candidate_files => $result->{candidate_files},
        regions => scalar(@{$result->{regions}}),
        authority_outcomes => $result->{outcome_counts},
    }), "\n";
} else {
    print "book-quantitative-claims: ", scalar(@{$result->{members}}),
        " book files contain ", scalar(@{$result->{candidates}}), " prose candidate lines across ",
        $result->{candidate_files}, " files in '$result->{phase}' phase with ",
        scalar(@{$result->{regions}}), " adjudicated region(s).\n";
}
exit 0;

sub usage {
    die "Usage: $0 [--root DIR] [--contract PATH] [--check|--report|--produce|--self-test]\n";
}

sub validate_contract {
    my (%args) = @_;
    my $base = $args{root};
    my $relative = $args{contract_rel};
    my $execute_commands = $args{execute_commands};
    my @errors;
    my ($meta, $records) = read_bounded_jsonl(
        absolute($base, $relative),
        'book quantitative contract',
        1_024,
        524_288,
        32_768,
        \@errors,
    );
    my $tracked = tracked_paths($base, \@errors);

    reject_unknown($meta, 'registry record', \@errors, qw(
        record_type schema_version phase expected_book_files expected_candidate_lines
        expected_candidate_files max_records max_bytes max_record_bytes max_array_items max_scalar_bytes
    ));
    exact_scalar($meta->{record_type}, 'registry', 'registry record_type', \@errors);
    exact_integer($meta->{schema_version}, 1, 'registry schema_version', \@errors);
    my $phase = required_scalar($meta->{phase}, 'registry phase', \@errors) // '';
    push @errors, "registry phase '$phase' is unknown" if $phase ne 'inventory' && $phase ne 'frozen';
    my $expected_files = positive_integer($meta->{expected_book_files}, 'registry expected_book_files', \@errors) // 0;
    my $expected_candidates = positive_integer(
        $meta->{expected_candidate_lines}, 'registry expected_candidate_lines', \@errors) // 0;
    my $expected_candidate_files = positive_integer(
        $meta->{expected_candidate_files}, 'registry expected_candidate_files', \@errors) // 0;

    my (@source_records, @region_records);
    for my $record (@$records) {
        my $type = $record->{record_type} // '';
        if ($type eq 'source') {
            push @source_records, $record;
        } elsif ($type eq 'region') {
            push @region_records, $record;
        } else {
            push @errors, "contract has unknown record_type '$type'";
        }
    }
    push @errors, 'inventory phase must contain zero region records' if $phase eq 'inventory' && @region_records;
    push @errors, 'frozen phase must contain at least one region record' if $phase eq 'frozen' && !@region_records;

    my %sources;
    for my $source (@source_records) {
        reject_unknown($source, 'source record', \@errors,
            qw(record_type schema_version source_id path sha256 identity));
        exact_integer($source->{schema_version}, 1, 'source schema_version', \@errors);
        my $id = required_scalar($source->{source_id}, 'source source_id', \@errors) // next;
        push @errors, "source duplicates source_id '$id'" if $sources{$id};
        $sources{$id} = $source;
        validate_tracked_file($base, $source->{path}, $tracked, "source '$id'", \@errors);
        if ($id eq 'claim_registry') {
            exact_scalar($source->{identity}, 'live_validated', "source '$id' identity", \@errors);
            push @errors, "source '$id' must not self-bind a digest" if exists $source->{sha256};
        } else {
            validate_sha($base, $source->{path}, $source->{sha256}, "source '$id'", \@errors);
            push @errors, "source '$id' has incompatible identity" if exists $source->{identity};
        }
    }
    my @source_ids = qw(surface_registry book_summary derived_state_registry claim_registry);
    push @errors, "contract lacks source '$_'" for grep { !$sources{$_} } @source_ids;
    my %known_source = map { $_ => 1 } @source_ids;
    push @errors, "contract has unknown source_id '$_'" for grep { !$known_source{$_} } keys %sources;

    my $members = [];
    if ($sources{surface_registry} && $sources{book_summary}) {
        $members = derive_book_members(
            $base,
            $sources{surface_registry}{path},
            $sources{book_summary}{path},
            $tracked,
            \@errors,
        );
    }
    push @errors, 'derived book-file denominator ' . scalar(@$members) . " differs from declared $expected_files"
        if scalar(@$members) != $expected_files;

    my $candidates = discover_candidates($base, $members, \@errors);
    my %candidate_file = map { $_->{path} => 1 } @$candidates;
    push @errors, 'derived candidate-line denominator ' . scalar(@$candidates)
        . " differs from declared $expected_candidates"
        if scalar(@$candidates) != $expected_candidates;
    push @errors, 'derived candidate-file denominator ' . scalar(keys %candidate_file)
        . " differs from declared $expected_candidate_files"
        if scalar(keys %candidate_file) != $expected_candidate_files;

    my %claims = claim_ids($base, $sources{claim_registry}{path}, \@errors);
    my %derived = derived_authorities($base, $sources{derived_state_registry}{path}, \@errors);
    my %member = map { $_ => 1 } @$members;
    my %candidate_by_path;
    push @{$candidate_by_path{$_->{path}}}, $_ for @$candidates;
    my (%region_id, %coverage, %outcome_counts);
    for my $region (@region_records) {
        reject_unknown($region, 'region record', \@errors, qw(
            record_type schema_version region_id path region outcome source_authority_id verifier
            claim_id missing_legs scope_reason
        ));
        exact_integer($region->{schema_version}, 1, 'region schema_version', \@errors);
        my $id = required_slug($region->{region_id}, 'region region_id', \@errors) // next;
        push @errors, "region duplicates region_id '$id'" if $region_id{$id}++;
        my $path = required_scalar($region->{path}, "region '$id' path", \@errors) // next;
        push @errors, "region '$id' path '$path' is outside derived book membership" if !$member{$path};
        validate_tracked_file($base, $path, $tracked, "region '$id'", \@errors);
        my ($start, $end, $region_text) = validate_region(
            $base, $path, $region->{region}, "region '$id'", \@errors);
        my @inside = grep { $_->{line} >= $start && $_->{line} <= $end }
            @{$candidate_by_path{$path} // []};
        push @errors, "region '$id' contains no quantitative candidate" if !@inside;
        for my $candidate (@inside) {
            my $key = "$path:$candidate->{line}";
            push @errors, "candidate '$key' is covered by overlapping regions '$coverage{$key}' and '$id'"
                if $coverage{$key};
            $coverage{$key} = $id;
        }
        my $outcome = required_scalar($region->{outcome}, "region '$id' outcome", \@errors) // '';
        $outcome_counts{$outcome}++ if $outcome ne '';
        validate_outcome(
            base => $base,
            tracked => $tracked,
            claims => \%claims,
            derived => \%derived,
            region => $region,
            region_text => $region_text,
            id => $id,
            outcome => $outcome,
            execute_commands => $execute_commands,
            errors => \@errors,
        );
    }
    if ($phase eq 'frozen') {
        for my $candidate (@$candidates) {
            my $key = "$candidate->{path}:$candidate->{line}";
            push @errors, "candidate '$key' lacks one adjudicated region" if !$coverage{$key};
        }
    }

    return {
        errors => \@errors,
        phase => $phase,
        members => $members,
        candidates => $candidates,
        candidate_files => scalar(keys %candidate_file),
        regions => \@region_records,
        outcome_counts => \%outcome_counts,
    };
}

sub derive_book_members {
    my ($base, $surface_rel, $summary_rel, $tracked, $errors) = @_;
    my @surface_rows = read_jsonl_records(absolute($base, $surface_rel), 'surface registry', $errors);
    my @shipped = grep { ($_->{surface_id} // '') eq 'shipped_behavior' } @surface_rows;
    if (@shipped != 1) {
        push @$errors, "surface registry must contain exactly one shipped_behavior record";
        return [];
    }
    my $targets = $shipped[0]{targets};
    if (ref($targets) ne 'ARRAY' || !@$targets) {
        push @$errors, 'shipped_behavior targets must be a nonempty array';
        return [];
    }
    my @regexes;
    for my $target (@$targets) {
        if (!safe_relative_pattern($target)) {
            push @$errors, "shipped_behavior has unsafe target '$target'";
            next;
        }
        push @regexes, glob_regex($target);
    }
    my @surface_members = sort grep {
        my $path = $_;
        scalar grep { $path =~ $_ } @regexes;
    } keys %$tracked;

    my $summary_raw = read_raw(absolute($base, $summary_rel), $errors, 'book summary');
    my %summary_members = ($summary_rel => 1);
    for my $line (split /\n/, $summary_raw) {
        while ($line =~ /\]\(([^)]+)\)/g) {
            my $target = $1;
            $target =~ s/^<|>$//g;
            $target =~ s/#.*\z//;
            next if $target !~ /\.md\z/;
            if ($target =~ m{\A/|\A[a-zA-Z][a-zA-Z0-9+.-]*:|(?:\A|/)\.\.(?:/|\z)}) {
                push @$errors, "book summary has unsafe Markdown target '$target'";
                next;
            }
            my $absolute = File::Spec->rel2abs($target, dirname(absolute($base, $summary_rel)));
            my $resolved = abs_path($absolute);
            if (!defined $resolved || index($resolved, $base . File::Spec->catfile('')) != 0) {
                push @$errors, "book summary target '$target' does not resolve inside the repository";
                next;
            }
            my $relative = File::Spec->abs2rel($resolved, $base);
            $relative =~ s{\\}{/}g;
            push @$errors, "book summary duplicates member '$relative'" if $summary_members{$relative}++;
        }
    }
    my @summary_members = sort keys %summary_members;
    my %surface = map { $_ => 1 } @surface_members;
    my %summary = map { $_ => 1 } @summary_members;
    push @$errors, "book summary omits shipped_behavior member '$_'"
        for grep { !$summary{$_} } @surface_members;
    push @$errors, "book summary includes non-shipped_behavior member '$_'"
        for grep { !$surface{$_} } @summary_members;
    return \@surface_members;
}

sub discover_candidates {
    my ($base, $members, $errors) = @_;
    my @candidates;
    for my $path (@$members) {
        my $raw = read_raw(absolute($base, $path), $errors, "book member '$path'");
        my @lines = split_lines($raw);
        my ($fence_char, $fence_len);
        for my $index (0 .. $#lines) {
            my $line = $lines[$index];
            my $visible = $line;
            $visible =~ s/\r?\n\z//;
            if (defined $fence_char) {
                if ($visible =~ /^\s{0,3}(\Q$fence_char\E{$fence_len,})\s*\z/) {
                    undef $fence_char;
                    undef $fence_len;
                }
                next;
            }
            if ($visible =~ /^\s{0,3}(`{3,}|~{3,})/) {
                my $run = $1;
                $fence_char = substr($run, 0, 1);
                $fence_len = length($run);
                next;
            }
            next if !is_candidate($visible);
            my $line_no = $index + 1;
            my $line_sha = sha256_hex($line);
            my $id_sha = sha256_hex(join("\0", $path, $line_no, $line_sha));
            push @candidates, {
                record_type => 'candidate',
                schema_version => 1,
                candidate_id => 'book-quantity-' . substr($id_sha, 0, 16),
                path => $path,
                line => $line_no,
                sha256 => $line_sha,
            };
        }
        push @$errors, "book member '$path' has an unterminated fenced code block" if defined $fence_char;
    }
    return \@candidates;
}

sub is_candidate {
    my ($line) = @_;
    # An ordered-list marker is not a quantity. It only became ambiguous when the optional adjective
    # below was added: `5. Dependency-connected questions` reads as "5 <adjective> questions" and would
    # publish a list index as a measured count. Strip the marker before matching, never the digits
    # inside the sentence.
    $line =~ s/^(\s*)[0-9]+\.\s+/$1/;
    my $number = qr/[0-9][0-9,._]*/;
    # CLAIM-VERIFICATION-ADOPTION.9. The unit vocabulary is a closed list and its blind spots are values
    # no record can ever be asked for, so both halves of this clause are measured rather than guessed:
    #   * `items`, `units`, `tables`, `cells` are the four nouns with a RECORDED miss — one per
    #     demonstration in `.9`. Adding them costs 13 new candidate lines.
    #   * up to TWO words may sit between the numeral and the noun. This is the third gap, and the one
    #     the noun list alone cannot close: the demonstrations published "126 NAME cells", "56 EXACT
    #     EVIDENCE units" and "464 CONVERTER TEXT items", where the noun never touches the numeral.
    #     The bound is measured, not chosen: 0 words catches 1 of the 4 demonstrated misses, 1 catches
    #     2, TWO catches 4 of 4, and a third word adds 12 more candidate lines while catching nothing
    #     new. Total cost of both halves: 118 new candidate lines, and the sample is overwhelmingly
    #     real published quantities ("53 of 56 Debug registers", "124 legacy tables", "39 public fields").
    # THREE words between numeral and noun remain invisible; that residual bound is asserted in the
    # self-test's grammar control rather than left for a reader to discover.
    my $units = qr/(?:files?|lines?|bytes?|records?|members?|facts?|questions?|shards?|cases?|tests?|checks?|surfaces?|claims?|fields?|families?|documents?|pages?|fixtures?|diagnostics?|commands?|doctrines?|signals?|registers?|artifacts?|rules?|items?|units?|tables?|cells?)/i;
    my $adjective = qr/(?:[A-Za-z][A-Za-z-]*\s+){0,2}/;
    # The unit alternation had no TRAILING boundary, so `signals?` matched inside "Gbps PHY SIGNALing",
    # `records?` inside "RECORDed once" and `checks?` inside "catalog CHECKer". Measured over the book:
    # the boundary removes exactly 5 lines and every one of the 5 is a false positive of that shape.
    # A precision fix, not a narrowing of what the alarm is for.
    return $line =~ /(?<![A-Za-z0-9_])(?:$number\s*%|$number\s*\/\s*$number|$number\s+$adjective$units(?![A-Za-z]))/ ? 1 : 0;
}

sub validate_outcome {
    my (%args) = @_;
    my $region = $args{region};
    my $outcome = $args{outcome};
    my $id = $args{id};
    my $errors = $args{errors};
    my %known = map { $_ => 1 } qw(derived identity_gated registered incomplete excluded);
    push @$errors, "region '$id' has unknown outcome '$outcome'" if !$known{$outcome};
    if ($outcome eq 'derived' || $outcome eq 'identity_gated') {
        if ($outcome eq 'derived') {
            my $authority_id = required_slug(
                $region->{source_authority_id}, "region '$id' source_authority_id", $errors);
            my $authority = defined($authority_id) ? $args{derived}{$authority_id} : undef;
            push @$errors, "region '$id' references unknown source_authority_id '$authority_id'"
                if defined($authority_id) && !$authority;
            if ($authority) {
                push @$errors, "region '$id' source authority path differs"
                    if ($authority->{path} // '') ne ($region->{path} // '');
                my $marker = $authority->{field_marker};
                push @$errors, "region '$id' exact region omits source authority marker"
                    if defined($marker) && index($args{region_text} // '', $marker) < 0;
            }
        }
        push @$errors, "region '$id' identity_gated outcome declares source_authority_id"
            if $outcome eq 'identity_gated' && exists $region->{source_authority_id};
        validate_verifier(%args, verifier => $region->{verifier});
        push @$errors, "region '$id' outcome '$outcome' has incompatible claim/missing/scope fields"
            if exists($region->{claim_id}) || exists($region->{missing_legs}) || exists($region->{scope_reason});
    } elsif ($outcome eq 'registered') {
        my $claim = required_slug($region->{claim_id}, "region '$id' claim_id", $errors);
        push @$errors, "region '$id' references unknown or noncurrent claim_id '$claim'"
            if defined($claim) && !$args{claims}{$claim};
        push @$errors, "region '$id' registered outcome has incompatible authority/verifier/missing/scope fields"
            if exists($region->{source_authority_id}) || exists($region->{verifier})
                || exists($region->{missing_legs}) || exists($region->{scope_reason});
    } elsif ($outcome eq 'incomplete') {
        my @legs = scalar_array($region->{missing_legs}, "region '$id' missing_legs", $errors);
        my %allowed = map { $_ => 1 } qw(rederive falsification durability);
        my %seen;
        push @$errors, "region '$id' missing_legs must be a nonempty unique subset of rederive/falsification/durability"
            if !@legs || grep { !$allowed{$_} || $seen{$_}++ } @legs;
        push @$errors, "region '$id' incomplete outcome has incompatible authority/verifier/claim/scope fields"
            if exists($region->{source_authority_id}) || exists($region->{verifier})
                || exists($region->{claim_id}) || exists($region->{scope_reason});
    } elsif ($outcome eq 'excluded') {
        my $reason = required_slug($region->{scope_reason}, "region '$id' scope_reason", $errors);
        my %allowed = map { $_ => 1 } qw(
            authored_threshold_or_choice example_or_command_literal
            schema_version_date_path_or_digest_identity dated_boundary_evidence
        );
        push @$errors, "region '$id' has unknown scope_reason '$reason'" if defined($reason) && !$allowed{$reason};
        push @$errors, "region '$id' excluded outcome has incompatible authority/verifier/claim/missing fields"
            if exists($region->{source_authority_id}) || exists($region->{verifier})
                || exists($region->{claim_id}) || exists($region->{missing_legs});
    }
}

sub validate_verifier {
    my (%args) = @_;
    my $verifier = $args{verifier};
    my $id = $args{id};
    my $errors = $args{errors};
    if (ref($verifier) ne 'HASH') {
        push @$errors, "region '$id' lacks verifier";
        return;
    }
    reject_unknown($verifier, "region '$id' verifier", $errors,
        qw(argv producer inputs expected_exit stdout_contains));
    my @argv = scalar_array($verifier->{argv}, "region '$id' verifier argv", $errors);
    my @inputs = scalar_array($verifier->{inputs}, "region '$id' verifier inputs", $errors);
    my $producer = required_scalar($verifier->{producer}, "region '$id' verifier producer", $errors);
    my $expected = nonnegative_integer($verifier->{expected_exit}, "region '$id' verifier expected_exit", $errors);
    my $needle = required_scalar($verifier->{stdout_contains}, "region '$id' verifier stdout_contains", $errors);
    validate_tracked_file($args{base}, $producer, $args{tracked}, "region '$id' verifier producer", $errors)
        if defined $producer;
    push @$errors, "region '$id' verifier argv must be nonempty" if !@argv;
    push @$errors, "region '$id' verifier inputs must be nonempty" if !@inputs;
    for my $arg (@argv) {
        push @$errors, "region '$id' verifier argv contains unsafe argument '$arg'"
            if $arg =~ m{\A/|(?:\A|/)\.\.(?:/|\z)};
    }
    if (@argv && defined($producer)) {
        my $invokes = $argv[0] eq $producer
            || (($argv[0] eq 'perl' || $argv[0] eq 'bash') && @argv > 1 && $argv[1] eq $producer);
        push @$errors, "region '$id' verifier argv does not invoke producer '$producer'" if !$invokes;
    }
    my %input_seen;
    for my $input (@inputs) {
        push @$errors, "region '$id' verifier duplicates input '$input'" if $input_seen{$input}++;
        validate_tracked_file($args{base}, $input, $args{tracked}, "region '$id' verifier input", $errors);
    }
    push @$errors, "region '$id' verifier inputs omit producer '$producer'"
        if defined($producer) && !grep { $_ eq $producer } @inputs;
    return if !$args{execute_commands} || !@argv || !defined($expected) || !defined($needle);
    my ($stdout, $stderr, $exit) = capture_command($args{base}, \@argv);
    push @$errors, "region '$id' verifier exit $exit != $expected: $stderr"
        if $exit != $expected;
    push @$errors, "region '$id' verifier stdout lacks '$needle'"
        if index($stdout, $needle) < 0;
}

sub validate_region {
    my ($base, $path, $region, $label, $errors) = @_;
    if (ref($region) ne 'HASH') {
        push @$errors, "$label region must be an object";
        return (0, 0, '');
    }
    reject_unknown($region, "$label region", $errors, qw(kind start_line end_line sha256));
    exact_scalar($region->{kind}, 'line_range_sha256', "$label region kind", $errors);
    my $start = positive_integer($region->{start_line}, "$label region start_line", $errors) // 0;
    my $end = positive_integer($region->{end_line}, "$label region end_line", $errors) // 0;
    push @$errors, "$label region end_line precedes start_line" if $end < $start;
    my $raw = read_raw(absolute($base, $path), $errors, $label);
    my @lines = split_lines($raw);
    if ($start < 1 || $end > @lines) {
        push @$errors, "$label region $start..$end is outside 1.." . scalar(@lines);
        return ($start, $end, '');
    }
    my $slice = join('', @lines[$start - 1 .. $end - 1]);
    my $actual = sha256_hex($slice);
    my $expected = $region->{sha256};
    push @$errors, "$label region sha256 must be 64 lowercase hex characters"
        if !defined($expected) || ref($expected) || $expected !~ /\A[0-9a-f]{64}\z/;
    push @$errors, "$label exact region is stale: SHA-256 $actual != $expected"
        if defined($expected) && !ref($expected) && $expected =~ /\A[0-9a-f]{64}\z/ && $actual ne $expected;
    return ($start, $end, $slice);
}

sub claim_ids {
    my ($base, $path, $errors) = @_;
    return () if !defined $path;
    my @rows = read_jsonl_records(absolute($base, $path), 'claim registry', $errors);
    my %ids;
    for my $row (@rows) {
        next if ($row->{record_type} // '') ne 'claim';
        next if ($row->{status} // '') eq 'superseded';
        $ids{$row->{claim_id}} = 1 if defined $row->{claim_id};
    }
    return %ids;
}

sub derived_authorities {
    my ($base, $path, $errors) = @_;
    return () if !defined $path;
    my @rows = read_jsonl_records(absolute($base, $path), 'derived-state registry', $errors);
    my %authorities;
    for my $row (@rows) {
        next if ($row->{record_type} // '') ne 'contract';
        my $id = $row->{contract_id};
        next if !defined($id) || ref($id) || $id eq '';
        push @$errors, "derived-state registry duplicates contract_id '$id'" if $authorities{$id};
        $authorities{$id} = $row;
    }
    return %authorities;
}

sub read_bounded_jsonl {
    my ($path, $label, $hard_records, $hard_bytes, $hard_record_bytes, $errors) = @_;
    my $raw = read_raw($path, $errors, $label);
    push @$errors, "$label exceeds portable hard byte cap $hard_bytes" if length($raw) > $hard_bytes;
    my @lines = grep { length($_) } split /\n/, $raw;
    push @$errors, "$label is empty" if !@lines;
    push @$errors, "$label exceeds portable hard record cap $hard_records" if @lines > $hard_records;
    my $json = JSON::PP->new->utf8(1);
    my @decoded;
    for my $index (0 .. $#lines) {
        push @$errors, "$label record " . ($index + 1) . " exceeds hard byte cap $hard_record_bytes"
            if length($lines[$index]) > $hard_record_bytes;
        my $record = eval { $json->decode($lines[$index]) };
        if ($@ || ref($record) ne 'HASH') {
            push @$errors, "$label record " . ($index + 1) . " is not a JSON object";
            next;
        }
        push @decoded, $record;
    }
    my $meta = shift(@decoded) // {};
    my %compiled = (
        max_records => $hard_records,
        max_bytes => $hard_bytes,
        max_record_bytes => $hard_record_bytes,
        max_array_items => 64,
        max_scalar_bytes => 4_096,
    );
    for my $field (qw(max_records max_bytes max_record_bytes max_array_items max_scalar_bytes)) {
        my $value = positive_integer($meta->{$field}, "registry $field", $errors);
        push @$errors, "registry $field exceeds portable hard cap $compiled{$field}"
            if defined($value) && $value > $compiled{$field};
    }
    push @$errors, "contract exceeds declared max_records $meta->{max_records}"
        if defined($meta->{max_records}) && @lines > $meta->{max_records};
    push @$errors, "contract exceeds declared max_bytes $meta->{max_bytes}"
        if defined($meta->{max_bytes}) && length($raw) > $meta->{max_bytes};
    for my $index (0 .. $#lines) {
        push @$errors, "contract record " . ($index + 1) . " exceeds declared max_record_bytes $meta->{max_record_bytes}"
            if defined($meta->{max_record_bytes}) && length($lines[$index]) > $meta->{max_record_bytes};
    }
    validate_shape($_, $meta->{max_array_items}, $meta->{max_scalar_bytes}, 'contract', $errors)
        for @decoded;
    return ($meta, \@decoded);
}

sub validate_shape {
    my ($value, $max_array, $max_scalar, $label, $errors) = @_;
    return if !defined($max_array) || !defined($max_scalar);
    if (ref($value) eq 'ARRAY') {
        push @$errors, "$label array exceeds max_array_items $max_array" if @$value > $max_array;
        validate_shape($_, $max_array, $max_scalar, $label, $errors) for @$value;
    } elsif (ref($value) eq 'HASH') {
        validate_shape($_, $max_array, $max_scalar, $label, $errors) for values %$value;
    } elsif (!ref($value) && defined($value)) {
        push @$errors, "$label scalar exceeds max_scalar_bytes $max_scalar" if length($value) > $max_scalar;
    }
}

sub read_jsonl_records {
    my ($path, $label, $errors) = @_;
    my $raw = read_raw($path, $errors, $label);
    my $json = JSON::PP->new->utf8(1);
    my @rows;
    my $line_no = 0;
    for my $line (split /\n/, $raw) {
        $line_no++;
        next if $line eq '';
        my $row = eval { $json->decode($line) };
        if ($@ || ref($row) ne 'HASH') {
            push @$errors, "$label line $line_no is not a JSON object";
            next;
        }
        push @rows, $row;
    }
    shift @rows if @rows && (($rows[0]{record_type} // '') eq 'registry' || exists $rows[0]{max_records});
    return @rows;
}

sub tracked_paths {
    my ($base, $errors) = @_;
    my ($stdout, $stderr, $exit) = capture_command($base, ['git', 'ls-files', '-z']);
    if ($exit != 0) {
        push @$errors, "git ls-files failed: $stderr";
        return {};
    }
    my %tracked = map { $_ => 1 } grep { length($_) } split /\0/, $stdout;
    return \%tracked;
}

sub validate_tracked_file {
    my ($base, $path, $tracked, $label, $errors) = @_;
    if (!safe_relative_path($path)) {
        push @$errors, "$label path is not safe repository-relative";
        return;
    }
    push @$errors, "$label path '$path' is untracked" if !$tracked->{$path};
    push @$errors, "$label path '$path' is missing, a symlink, or not a regular file"
        if !-f absolute($base, $path) || -l absolute($base, $path);
}

sub validate_sha {
    my ($base, $path, $expected, $label, $errors) = @_;
    if (!defined($expected) || ref($expected) || $expected !~ /\A[0-9a-f]{64}\z/) {
        push @$errors, "$label sha256 must be 64 lowercase hex characters";
        return;
    }
    my $actual = sha256_hex(read_raw(absolute($base, $path), $errors, $label));
    push @$errors, "$label is stale: SHA-256 $actual != $expected" if $actual ne $expected;
}

sub reject_unknown {
    my ($record, $label, $errors, @allowed) = @_;
    my %allowed = map { $_ => 1 } @allowed;
    push @$errors, "$label has unknown field '$_'" for grep { !$allowed{$_} } keys %$record;
}

sub exact_scalar {
    my ($value, $expected, $label, $errors) = @_;
    push @$errors, "$label must be '$expected'" if !defined($value) || ref($value) || $value ne $expected;
}

sub exact_integer {
    my ($value, $expected, $label, $errors) = @_;
    push @$errors, "$label must be integer $expected"
        if !defined($value) || ref($value) || $value !~ /\A[0-9]+\z/ || $value != $expected;
}

sub positive_integer {
    my ($value, $label, $errors) = @_;
    if (!defined($value) || ref($value) || $value !~ /\A[1-9][0-9]*\z/) {
        push @$errors, "$label must be a positive integer";
        return;
    }
    return 0 + $value;
}

sub nonnegative_integer {
    my ($value, $label, $errors) = @_;
    if (!defined($value) || ref($value) || $value !~ /\A[0-9]+\z/) {
        push @$errors, "$label must be a nonnegative integer";
        return;
    }
    return 0 + $value;
}

sub required_scalar {
    my ($value, $label, $errors) = @_;
    if (!defined($value) || ref($value) || $value eq '') {
        push @$errors, "$label must be a nonempty scalar";
        return;
    }
    return $value;
}

sub required_slug {
    my ($value, $label, $errors) = @_;
    my $scalar = required_scalar($value, $label, $errors);
    return if !defined $scalar;
    push @$errors, "$label must be a lowercase slug" if $scalar !~ /\A[a-z0-9]+(?:[._-][a-z0-9]+)*\z/;
    return $scalar;
}

sub scalar_array {
    my ($value, $label, $errors) = @_;
    if (ref($value) ne 'ARRAY') {
        push @$errors, "$label must be an array";
        return ();
    }
    my @values;
    for my $item (@$value) {
        if (!defined($item) || ref($item) || $item eq '') {
            push @$errors, "$label items must be nonempty scalars";
            next;
        }
        push @values, $item;
    }
    return @values;
}

sub safe_relative_path {
    my ($path) = @_;
    return 0 if !defined($path) || ref($path) || $path eq '' || $path =~ /\0/;
    return 0 if $path =~ m{\A/|\A[a-zA-Z][a-zA-Z0-9+.-]*:|(?:\A|/)\.\.(?:/|\z)};
    return 1;
}

sub safe_relative_pattern {
    my ($path) = @_;
    return safe_relative_path($path) && $path !~ /[{}\[\]]/;
}

sub glob_regex {
    my ($glob) = @_;
    my $quoted = '';
    for (my $i = 0; $i < length($glob); $i++) {
        my $char = substr($glob, $i, 1);
        if ($char eq '*' && substr($glob, $i, 3) eq '**/') {
            $quoted .= '(?:.*/)?';
            $i += 2;
        } elsif ($char eq '*' && substr($glob, $i, 2) eq '**') {
            $quoted .= '.*';
            $i++;
        } elsif ($char eq '*') {
            $quoted .= '[^/]*';
        } elsif ($char eq '?') {
            $quoted .= '[^/]';
        } else {
            $quoted .= quotemeta($char);
        }
    }
    return qr/\A$quoted\z/;
}

sub absolute {
    my ($base, $relative) = @_;
    return File::Spec->catfile($base, split m{/}, $relative // '');
}

sub read_raw {
    my ($path, $errors, $label) = @_;
    if (!open my $fh, '<:raw', $path) {
        push @$errors, "$label cannot be read at $path: $!";
        return '';
    } else {
        local $/;
        my $raw = <$fh> // '';
        close $fh;
        return $raw;
    }
}

sub split_lines {
    my ($raw) = @_;
    my @lines = $raw =~ /(.*?(?:\n|\z))/gs;
    pop @lines if @lines && $lines[-1] eq '';
    return @lines;
}

sub capture_command {
    my ($base, $argv) = @_;
    my $old = getcwd();
    chdir $base or die "book-quantitative-claims: cannot chdir to $base: $!\n";
    my $stderr_fh = gensym;
    my ($stdin_fh, $stdout_fh);
    my $pid = eval { open3($stdin_fh, $stdout_fh, $stderr_fh, @$argv) };
    if ($@) {
        chdir $old or die "book-quantitative-claims: cannot restore cwd: $!\n";
        return ('', $@, 127);
    }
    close $stdin_fh;
    my $select = IO::Select->new($stdout_fh, $stderr_fh);
    my ($stdout, $stderr) = ('', '');
    my %stdout_fd = (fileno($stdout_fh) => 1);
    while (my @ready = $select->can_read) {
        for my $fh (@ready) {
            my $buffer = '';
            my $read = sysread($fh, $buffer, 8192);
            if (!defined($read) || $read == 0) {
                $select->remove($fh);
                close $fh;
            } elsif ($stdout_fd{fileno($fh)}) {
                $stdout .= $buffer;
            } else {
                $stderr .= $buffer;
            }
        }
    }
    waitpid($pid, 0);
    my $exit = $? == -1 ? 127 : ($? >> 8);
    chdir $old or die "book-quantitative-claims: cannot restore cwd: $!\n";
    return ($stdout, $stderr, $exit);
}

sub run_self_test {
    my $generated = absolute($script_root, 'generated');
    make_path($generated) if !-d $generated;
    my $fixture = File::Spec->catdir($generated, ".book-quantitative-claims-self-test.$$");
    die "book-quantitative-claims self-test: unsafe fixture path\n"
        if index($fixture, $generated . File::Spec->catfile('')) != 0;
    remove_tree($fixture) if -e $fixture;
    make_path(absolute($fixture, 'doctrine/claim_verification'));
    make_path(absolute($fixture, 'doctrine/live_document_size'));
    make_path(absolute($fixture, 'docs/book/src'));
    make_path(absolute($fixture, 'scripts'));
    write_raw(absolute($fixture, 'docs/book/src/SUMMARY.md'), "# Summary\n\n- [Chapter](chapter.md)\n");
    write_raw(absolute($fixture, 'docs/book/src/chapter.md'), join('',
        "# Chapter\n",
        "Current 2/3 records.\n",
        "\n",
        "```text\n",
        "Ignored 99/100 tests.\n",
        "```\n",
        "Historical 4 files.\n",
    ));
    write_raw(absolute($fixture, 'scripts/probe.pl'), "#!/usr/bin/env perl\nprint qq{book quantity PASS\\n};\n");
    my $registry_meta = {record_type => 'registry', schema_version => 1, max_records => 16,
        max_bytes => 16_384, max_record_bytes => 4_096, max_array_items => 8, max_scalar_bytes => 512};
    write_jsonl(absolute($fixture, 'doctrine/live_document_size/surfaces.jsonl'), [
        $registry_meta,
        {surface_id => 'shipped_behavior', targets => ['docs/book/src/*.md']},
    ]);
    write_jsonl(absolute($fixture, 'doctrine/live_document_size/derived_state_contracts.jsonl'), [
        $registry_meta,
        {record_type => 'contract', contract_id => 'fixture-source', surface_id => 'shipped_behavior',
            path => 'docs/book/src/chapter.md', field_marker => 'Current 2/3 records.'},
    ]);
    write_jsonl(absolute($fixture, 'doctrine/claim_verification/claims.jsonl'), [
        $registry_meta,
        {record_type => 'claim', claim_id => 'fixture-claim', status => 'verified'},
    ]);
    command_ok($fixture, ['git', 'init', '-q']);
    command_ok($fixture, ['git', 'config', 'user.email', 'book-quantity-self-test@example.invalid']);
    command_ok($fixture, ['git', 'config', 'user.name', 'Book Quantity Self Test']);

    my $meta = {record_type => 'registry', schema_version => 1, phase => 'frozen',
        expected_book_files => 2, expected_candidate_lines => 2, expected_candidate_files => 1,
        max_records => 32, max_bytes => 32_768, max_record_bytes => 4_096,
        max_array_items => 8, max_scalar_bytes => 512};
    my @sources = (
        {record_type => 'source', schema_version => 1, source_id => 'surface_registry',
            path => 'doctrine/live_document_size/surfaces.jsonl',
            sha256 => sha256_hex(read_file(absolute($fixture, 'doctrine/live_document_size/surfaces.jsonl')))},
        {record_type => 'source', schema_version => 1, source_id => 'book_summary',
            path => 'docs/book/src/SUMMARY.md',
            sha256 => sha256_hex(read_file(absolute($fixture, 'docs/book/src/SUMMARY.md')))},
        {record_type => 'source', schema_version => 1, source_id => 'derived_state_registry',
            path => 'doctrine/live_document_size/derived_state_contracts.jsonl',
            sha256 => sha256_hex(read_file(
                absolute($fixture, 'doctrine/live_document_size/derived_state_contracts.jsonl')))},
        {record_type => 'source', schema_version => 1, source_id => 'claim_registry',
            path => 'doctrine/claim_verification/claims.jsonl', identity => 'live_validated'},
    );
    my @chapter_lines = split_lines(read_file(absolute($fixture, 'docs/book/src/chapter.md')));
    my @regions = (
        {record_type => 'region', schema_version => 1, region_id => 'current-record-count',
            path => 'docs/book/src/chapter.md',
            region => {kind => 'line_range_sha256', start_line => 2, end_line => 2,
                sha256 => sha256_hex($chapter_lines[1])},
            outcome => 'derived', source_authority_id => 'fixture-source',
            verifier => {argv => ['perl', 'scripts/probe.pl'], producer => 'scripts/probe.pl',
                inputs => ['scripts/probe.pl', 'docs/book/src/chapter.md'], expected_exit => 0,
                stdout_contains => 'book quantity PASS'}},
        {record_type => 'region', schema_version => 1, region_id => 'historical-file-count',
            path => 'docs/book/src/chapter.md',
            region => {kind => 'line_range_sha256', start_line => 7, end_line => 7,
                sha256 => sha256_hex($chapter_lines[6])},
            outcome => 'excluded', scope_reason => 'dated_boundary_evidence'},
    );
    my @base_records = ($meta, @sources, @regions);
    write_jsonl(absolute($fixture, $contract_rel), \@base_records);
    command_ok($fixture, ['git', 'add', '.']);
    command_ok($fixture, ['git', 'commit', '-qm', 'fixture']);

    my $executed = validate_contract(root => $fixture, contract_rel => $contract_rel, execute_commands => 1);
    die "book-quantitative-claims self-test clean execution failed: " . join('; ', @{$executed->{errors}}) . "\n"
        if @{$executed->{errors}};

    my @cases = (
        ['clean frozen contract', 1, qr//, sub {}],
        ['clean inventory contract', 1, qr//, sub {
            $_[0][0]{phase} = 'inventory';
            @{$_[0]} = grep { ($_->{record_type} // '') ne 'region' } @{$_[0]};
        }],
        ['record count is independent of field-array bound', 1, qr//, sub {
            $_[0][0]{max_array_items} = 4;
        }],
        ['candidate coverage gap', 0, qr/lacks one adjudicated region/, sub { pop @{$_[0]} }],
        ['overlapping regions', 0, qr/covered by overlapping regions/, sub {
            my $copy = clone($_[0][-2]);
            $copy->{region_id} = 'overlap-record-count';
            push @{$_[0]}, $copy;
        }],
        ['stale exact region', 0, qr/exact region is stale/, sub { $_[0][-1]{region}{sha256} = '0' x 64 }],
        ['region without candidate', 0, qr/contains no quantitative candidate/, sub {
            $_[0][-1]{region}{start_line} = 1;
            $_[0][-1]{region}{end_line} = 1;
            $_[0][-1]{region}{sha256} = sha256_hex($chapter_lines[0]);
        }],
        ['unknown outcome', 0, qr/unknown outcome/, sub { $_[0][-1]{outcome} = 'trusted' }],
        ['unknown exclusion reason', 0, qr/unknown scope_reason/, sub { $_[0][-1]{scope_reason} = 'looks_old' }],
        ['missing executable verifier', 0, qr/lacks verifier/, sub { delete $_[0][-2]{verifier} }],
        ['unknown registered claim', 0, qr/unknown or noncurrent claim_id/, sub {
            $_[0][-2]{outcome} = 'registered';
            $_[0][-2]{claim_id} = 'missing-claim';
            delete $_[0][-2]{source_authority_id};
            delete $_[0][-2]{verifier};
        }],
        ['unknown derived authority', 0, qr/unknown source_authority_id/, sub {
            $_[0][-2]{source_authority_id} = 'missing-source';
        }],
        ['verifier does not invoke producer', 0, qr/does not invoke producer/, sub {
            $_[0][-2]{verifier}{argv} = ['perl', '-e', 'print qq{book quantity PASS\\n}'];
        }],
        ['untracked region path', 0, qr/outside derived book membership|is untracked/, sub {
            $_[0][-1]{path} = 'untracked.md';
        }],
        ['stale source digest', 0, qr/is stale: SHA-256/, sub { $_[0][1]{sha256} = '0' x 64 }],
        ['wrong candidate denominator', 0, qr/candidate-line denominator/, sub {
            $_[0][0]{expected_candidate_lines} = 3;
        }],
        ['duplicate region identity', 0, qr/duplicates region_id/, sub {
            push @{$_[0]}, clone($_[0][-1]);
        }],
        ['unknown region field', 0, qr/unknown field 'surprise'/, sub { $_[0][-1]{surprise} = 1 }],
        ['portable record bound', 0, qr/max_records exceeds portable hard cap/, sub {
            $_[0][0]{max_records} = 1_025;
        }],
    );
    my ($passed, $total) = (0, 0);

    # THE GRAMMAR CONTROL (CLAIM-VERIFICATION-ADOPTION.9). Every other case below mutates a RECORD;
    # this one reads the candidate grammar itself, because the failure `.9` exists to stop is not a
    # malformed record — it is a published quantity the scanner cannot see, which no record is ever
    # asked for and which therefore passes silently. Each `must_see` line is a quantity a demonstration
    # actually published while the census reported full coverage and exited 0.
    {
        $total++;
        my @must_see = (
            '126 name cells, and 81 of them are TileLink',        # .9 fourth demonstration
            'contained 56 exact evidence units: 11 derived',      # .9 second demonstration
            'the rule accepts 285 tables in nine documents',      # .9 third demonstration
            '464 converter text items, 115 reaching a record',    # .9 first demonstration
        );
        # The other half of a grammar control: what it must NOT mint. A date, an identifier fragment and
        # an ordered-list marker are not quantities, and the list marker only became ambiguous when the
        # optional adjective was added.
        my @must_not = (
            'measured on 2026-08-28 and repaired later',
            'the segment-0013 capsule is sealed',
            '5. Dependency-connected questions stay in one class',
        );
        my @wrong = ((grep { !is_candidate($_) } @must_see), (grep { is_candidate($_) } @must_not));
        die "book-quantitative-claims self-test 'candidate grammar' mis-classified: "
            . join(' | ', @wrong) . "\n" if @wrong;
        $passed++;
    }

    for my $case (@cases) {
        $total++;
        my ($name, $expected_ok, $diagnostic, $mutate) = @$case;
        my $records = clone(\@base_records);
        $mutate->($records);
        write_jsonl(absolute($fixture, $contract_rel), $records);
        my $result = validate_contract(root => $fixture, contract_rel => $contract_rel, execute_commands => 0);
        my $ok = @{$result->{errors}} ? 0 : 1;
        my $joined = join("\n", @{$result->{errors}});
        die "book-quantitative-claims self-test '$name' expected " . ($expected_ok ? 'PASS' : 'RED')
            . ", got " . ($ok ? 'PASS' : "RED: $joined") . "\n"
            if $ok != $expected_ok;
        die "book-quantitative-claims self-test '$name' missed diagnostic $diagnostic: $joined\n"
            if !$expected_ok && $joined !~ $diagnostic;
        $passed++;
    }
    remove_tree($fixture);
    die "book-quantitative-claims self-test: residue remains at $fixture\n" if -e $fixture;
    # PRODUCTION-GRAPH-CENSUS-PIN.3 — `$passed/$total` detects a FAILING case but not a
    # DELETED one: `$total` is incremented in the same case loop, so removing a case drops both
    # and the ratio stays N/N (measured: this suite went 19/19 -> 18/18 and exited 0). The
    # expected case count is therefore declared here, independently of the loop.
    my $expected_cases = 20;
    die "book-quantitative-claims: self-test ran $total cases, declaration expects $expected_cases — "
        . "re-derive the declaration beside the suite\n"
        if $total != $expected_cases;
    print "book-quantitative-claims: self-test $passed/$total inventory, fence, coverage, region, authority, and bound cases pass.\n";
    exit 0;
}

sub write_jsonl {
    my ($path, $records) = @_;
    my $json = JSON::PP->new->canonical(1);
    write_raw($path, join('', map { $json->encode($_) . "\n" } @$records));
}

sub write_raw {
    my ($path, $raw) = @_;
    make_path(dirname($path)) if !-d dirname($path);
    open my $fh, '>:raw', $path or die "book-quantitative-claims self-test: cannot write $path: $!\n";
    print {$fh} $raw or die "book-quantitative-claims self-test: cannot write bytes to $path: $!\n";
    close $fh or die "book-quantitative-claims self-test: cannot close $path: $!\n";
}

sub read_file {
    my ($path) = @_;
    open my $fh, '<:raw', $path or die "book-quantitative-claims self-test: cannot read $path: $!\n";
    local $/;
    my $raw = <$fh> // '';
    close $fh;
    return $raw;
}

sub clone {
    my ($value) = @_;
    my $json = JSON::PP->new->canonical(1);
    return $json->decode($json->encode($value));
}

sub command_ok {
    my ($base, $argv) = @_;
    my ($stdout, $stderr, $exit) = capture_command($base, $argv);
    die "book-quantitative-claims self-test: '@$argv' failed ($exit): $stdout$stderr\n" if $exit != 0;
}
