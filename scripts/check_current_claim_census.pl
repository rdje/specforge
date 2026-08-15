#!/usr/bin/env perl
use strict;
use warnings;

use Cwd qw(abs_path getcwd);
use Digest::SHA qw(sha256_hex);
use Encode qw(encode_utf8);
use File::Basename qw(dirname);
use File::Path qw(make_path remove_tree);
use File::Spec;
use IO::Select;
use IPC::Open3;
use JSON::PP;
use Symbol qw(gensym);

my $script_root = abs_path(File::Spec->catdir(dirname(abs_path($0)), '..'));
my $root;
my $contract_rel = 'doctrine/claim_verification/current_claim_census.jsonl';
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
$root = abs_path($root) // die "current-claim-census: repository root does not exist\n";
run_self_test() if $mode eq 'self-test';

my $result = validate_census(
    root => $root,
    contract_rel => $contract_rel,
    execute_commands => 1,
);
if (@{$result->{errors}}) {
    print STDERR "current-claim-census: $_\n" for @{$result->{errors}};
    print STDERR "current-claim-census: FAILED with " . scalar(@{$result->{errors}}) . " violation(s).\n";
    exit 1;
}

my $candidates = produce_candidates($result);
if ($mode eq 'produce') {
    my $json = JSON::PP->new->canonical(1);
    print $json->encode($_), "\n" for @$candidates;
} elsif ($mode eq 'report') {
    my $summary = {
        phase => $result->{phase},
        current_surfaces => scalar(keys %{$result->{current_surfaces}}),
        included_surfaces => $result->{included_surfaces},
        excluded_surfaces => $result->{excluded_surfaces},
        views => scalar(keys %{$result->{views}}),
        evidence_units => scalar(@{$result->{evidence}}),
        authority_outcomes => $result->{outcome_counts},
        producer_candidates => scalar(@$candidates),
    };
    print JSON::PP->new->canonical(1)->encode($summary), "\n";
} else {
    print "current-claim-census: ", scalar(keys %{$result->{current_surfaces}}),
        " current surfaces (", $result->{included_surfaces}, " included, ",
        $result->{excluded_surfaces}, " excluded) cover ", scalar(keys %{$result->{views}}),
        " views in '$result->{phase}' phase with ", scalar(@{$result->{evidence}}),
        " frozen evidence unit(s).\n";
}
exit 0;

sub usage {
    die "Usage: $0 [--root DIR] [--contract PATH] [--check|--report|--produce|--self-test]\n";
}

sub validate_census {
    my (%args) = @_;
    my $base = $args{root};
    my $relative = $args{contract_rel};
    my $execute_commands = $args{execute_commands};
    my @errors;
    my ($meta, $records) = read_bounded_jsonl(
        absolute($base, $relative),
        'census contract',
        256,
        262_144,
        16_384,
        \@errors,
    );
    my $tracked = tracked_paths($base, \@errors);

    reject_unknown(
        $meta,
        'census registry record',
        \@errors,
        qw(record_type schema_version phase expected_current_surfaces max_records max_bytes max_record_bytes max_array_items max_scalar_bytes required_views),
    );
    exact_scalar($meta->{record_type}, 'registry', 'census registry record_type', \@errors);
    exact_integer($meta->{schema_version}, 1, 'census registry schema_version', \@errors);
    my $phase = required_scalar($meta->{phase}, 'census registry phase', \@errors) // '';
    push @errors, "census registry phase '$phase' is unknown"
        if $phase ne 'inventory' && $phase ne 'frozen';
    my $expected_current = positive_integer(
        $meta->{expected_current_surfaces},
        'census registry expected_current_surfaces',
        \@errors,
    ) // 0;
    my @required_views = scalar_array($meta->{required_views}, 'census registry required_views', \@errors);
    my %required_view_seen;
    for my $view (@required_views) {
        push @errors, "census registry required_views duplicates '$view'" if $required_view_seen{$view}++;
    }
    my @canonical_views = qw(
      current_status roadmap_controller_projections maintained_references
      doctrine_baselines mdbook_quantitative_claims
    );
    push @errors, 'census registry required_views must be the exact five-view domain'
        if JSON::PP->new->canonical(1)->encode([sort @required_views])
            ne JSON::PP->new->canonical(1)->encode([sort @canonical_views]);

    my (@source_records, @view_records, @surface_records, @evidence_records);
    for my $record (@$records) {
        my $type = $record->{record_type} // '';
        if ($type eq 'source') {
            push @source_records, $record;
        } elsif ($type eq 'view') {
            push @view_records, $record;
        } elsif ($type eq 'surface') {
            push @surface_records, $record;
        } elsif ($type eq 'evidence') {
            push @evidence_records, $record;
        } else {
            push @errors, "census contract has unknown record_type '$type'";
        }
    }

    my %sources;
    for my $source (@source_records) {
        reject_unknown($source, 'census source record', \@errors,
            qw(record_type schema_version source_id path sha256 identity));
        exact_integer($source->{schema_version}, 1, 'census source schema_version', \@errors);
        my $id = required_scalar($source->{source_id}, 'census source source_id', \@errors) // next;
        push @errors, "census source duplicates source_id '$id'" if $sources{$id};
        $sources{$id} = $source;
        validate_tracked_file($base, $source->{path}, $tracked, "census source '$id'", \@errors);
        if ($id eq 'claim_registry') {
            exact_scalar($source->{identity}, 'live_validated', "census source '$id' identity", \@errors);
            push @errors, "census source '$id' must not self-bind a registry digest"
                if exists $source->{sha256};
        } else {
            validate_sha($base, $source->{path}, $source->{sha256}, "census source '$id'", \@errors);
            push @errors, "census source '$id' has incompatible identity"
                if exists $source->{identity};
        }
    }
    my @source_ids = qw(surface_registry derived_state_registry claim_registry);
    for my $id (@source_ids) {
        push @errors, "census contract lacks source '$id'" if !$sources{$id};
    }
    my %known_source_id = map { $_ => 1 } @source_ids;
    push @errors, "census contract has unknown source_id '$_'"
        for grep { !$known_source_id{$_} } keys %sources;

    my $surface_source = source_records($base, $sources{surface_registry}, 'surface registry', \@errors);
    my $derived_source = source_records($base, $sources{derived_state_registry}, 'derived-state registry', \@errors);
    my $claim_source = source_records($base, $sources{claim_registry}, 'claim registry', \@errors);

    my %views;
    for my $view (@view_records) {
        reject_unknown($view, 'census view record', \@errors,
            qw(record_type schema_version view_id description));
        exact_integer($view->{schema_version}, 1, 'census view schema_version', \@errors);
        my $id = required_scalar($view->{view_id}, 'census view view_id', \@errors) // next;
        required_scalar($view->{description}, "census view '$id' description", \@errors);
        push @errors, "census view duplicates view_id '$id'" if $views{$id};
        push @errors, "census view '$id' is not required" if !$required_view_seen{$id};
        $views{$id} = $view;
    }
    push @errors, "census contract lacks required view '$_'" for grep { !$views{$_} } @required_views;

    my %current_surfaces;
    for my $surface (@$surface_source) {
        my $id = required_scalar($surface->{surface_id}, 'source surface_id', \@errors) // next;
        my $lifecycle = required_scalar($surface->{lifecycle}, "source surface '$id' lifecycle", \@errors) // '';
        next if $lifecycle eq 'archive_terminal' || $lifecycle eq 'frozen_legacy';
        push @errors, "surface registry duplicates current surface_id '$id'" if $current_surfaces{$id};
        $current_surfaces{$id} = $surface;
    }
    push @errors, 'derived current-surface denominator ' . scalar(keys %current_surfaces)
        . " differs from declared $expected_current"
        if scalar(keys %current_surfaces) != $expected_current;

    my %surface_contract;
    my %view_usage;
    my ($included, $excluded) = (0, 0);
    for my $surface (@surface_records) {
        reject_unknown($surface, 'census surface record', \@errors,
            qw(record_type schema_version surface_id disposition views scope_reason));
        exact_integer($surface->{schema_version}, 1, 'census surface schema_version', \@errors);
        my $id = required_scalar($surface->{surface_id}, 'census surface surface_id', \@errors) // next;
        push @errors, "census surface duplicates surface_id '$id'" if $surface_contract{$id};
        push @errors, "census surface '$id' is outside the derived current denominator"
            if !$current_surfaces{$id};
        my $disposition = required_scalar($surface->{disposition}, "census surface '$id' disposition", \@errors) // '';
        if ($disposition eq 'included') {
            $included++;
            my @surface_views = scalar_array($surface->{views}, "census surface '$id' views", \@errors);
            push @errors, "included census surface '$id' has no view" if !@surface_views;
            my %seen;
            for my $view (@surface_views) {
                push @errors, "census surface '$id' duplicates view '$view'" if $seen{$view}++;
                push @errors, "census surface '$id' references unknown view '$view'" if !$views{$view};
                $view_usage{$view}++;
            }
            push @errors, "included census surface '$id' declares scope_reason"
                if exists $surface->{scope_reason};
        } elsif ($disposition eq 'excluded') {
            $excluded++;
            required_slug($surface->{scope_reason}, "excluded census surface '$id' scope_reason", \@errors);
            push @errors, "excluded census surface '$id' declares views" if exists $surface->{views};
        } else {
            push @errors, "census surface '$id' has unknown disposition '$disposition'";
        }
        $surface_contract{$id} = $surface;
    }
    push @errors, "current surface '$_' lacks one census disposition"
        for grep { !$surface_contract{$_} } sort keys %current_surfaces;
    push @errors, "required view '$_' has no included surface"
        for grep { !$view_usage{$_} } @required_views;

    my %path_owner;
    for my $id (sort keys %current_surfaces) {
        my $targets = $current_surfaces{$id}{targets};
        if (ref($targets) ne 'ARRAY' || !@$targets) {
            push @errors, "source surface '$id' has no targets";
            next;
        }
        my @regexes;
        for my $target (@$targets) {
            if (!safe_relative_pattern($target)) {
                push @errors, "source surface '$id' has unsafe target '$target'";
                next;
            }
            push @regexes, glob_regex($target);
        }
        my @matches = grep {
            my $path = $_;
            scalar grep { $path =~ $_ } @regexes;
        } sort keys %$tracked;
        push @errors, "source surface '$id' matches no tracked path" if !@matches;
        for my $path (@matches) {
            push @errors, "tracked path '$path' belongs to multiple current surfaces: $path_owner{$path}, $id"
                if exists $path_owner{$path} && $path_owner{$path} ne $id;
            $path_owner{$path} = $id;
        }
    }

    my %derived_by_id;
    for my $contract (@$derived_source) {
        my $id = $contract->{contract_id};
        $derived_by_id{$id} = $contract if defined($id) && !ref($id) && $id ne '';
    }
    my %claim_by_id;
    for my $claim (@$claim_source) {
        my $id = $claim->{claim_id};
        $claim_by_id{$id} = $claim if defined($id) && !ref($id) && $id ne '';
    }

    my (%evidence_seen, %claim_key_seen, %surface_evidence, %view_evidence, %outcome_counts);
    for my $evidence (@evidence_records) {
        validate_evidence(
            $base, $evidence, $execute_commands, $tracked, \%views, \%surface_contract,
            \%path_owner, \%derived_by_id, \%claim_by_id, \%evidence_seen,
            \%claim_key_seen, \%surface_evidence, \%view_evidence, \%outcome_counts, \@errors,
        );
    }
    if ($phase eq 'inventory') {
        push @errors, 'inventory phase must not freeze evidence records' if @evidence_records;
    } elsif ($phase eq 'frozen') {
        push @errors, 'frozen phase has no evidence records' if !@evidence_records;
        push @errors, "included census surface '$_' has no frozen evidence unit"
            for grep {
                ($surface_contract{$_}{disposition} // '') eq 'included' && !$surface_evidence{$_}
            } sort keys %surface_contract;
        push @errors, "required view '$_' has no frozen evidence unit"
            for grep { !$view_evidence{$_} } @required_views;
    }

    return {
        errors => \@errors,
        phase => $phase,
        current_surfaces => \%current_surfaces,
        surface_contract => \%surface_contract,
        path_owner => \%path_owner,
        views => \%views,
        evidence => \@evidence_records,
        derived_source => $derived_source,
        claim_source => $claim_source,
        tracked => $tracked,
        root => $base,
        included_surfaces => $included,
        excluded_surfaces => $excluded,
        outcome_counts => \%outcome_counts,
    };
}

sub validate_evidence {
    my ($base, $record, $execute, $tracked, $views, $surfaces, $path_owner,
        $derived, $claims, $evidence_seen, $claim_key_seen, $surface_evidence,
        $view_evidence, $outcome_counts, $errors) = @_;
    reject_unknown($record, 'census evidence record', $errors,
        qw(record_type schema_version evidence_id claim_key surface_id view_id path region outcome verifier claim_id missing_legs scope_reason source_authority_id));
    exact_integer($record->{schema_version}, 1, 'census evidence schema_version', $errors);
    my $id = required_slug($record->{evidence_id}, 'census evidence evidence_id', $errors) // return;
    my $claim_key = required_slug($record->{claim_key}, "census evidence '$id' claim_key", $errors);
    push @$errors, "census evidence duplicates evidence_id '$id'" if $evidence_seen->{$id}++;
    push @$errors, "census evidence duplicates claim_key '$claim_key'" if defined($claim_key) && $claim_key_seen->{$claim_key}++;
    my $surface_id = required_scalar($record->{surface_id}, "census evidence '$id' surface_id", $errors) // '';
    my $surface = $surfaces->{$surface_id};
    push @$errors, "census evidence '$id' references unknown surface '$surface_id'" if !$surface;
    push @$errors, "census evidence '$id' references excluded surface '$surface_id'"
        if $surface && ($surface->{disposition} // '') ne 'included';
    my $view_id = required_scalar($record->{view_id}, "census evidence '$id' view_id", $errors) // '';
    push @$errors, "census evidence '$id' references unknown view '$view_id'" if !$views->{$view_id};
    if ($surface && ref($surface->{views}) eq 'ARRAY') {
        push @$errors, "census evidence '$id' view '$view_id' is not assigned to surface '$surface_id'"
            if !grep { $_ eq $view_id } @{$surface->{views}};
    }
    my $path = $record->{path};
    validate_tracked_file($base, $path, $tracked, "census evidence '$id' path", $errors);
    push @$errors, "census evidence '$id' path '$path' belongs to '" . ($path_owner->{$path} // 'no current surface')
        . "', not '$surface_id'"
        if defined($path) && (!defined($path_owner->{$path}) || $path_owner->{$path} ne $surface_id);
    my $region_text = validate_region($base, $path, $record->{region}, "census evidence '$id'", $errors);

    my $outcome = required_scalar($record->{outcome}, "census evidence '$id' outcome", $errors) // '';
    my %valid_outcome = map { $_ => 1 } qw(derived identity_gated registered incomplete excluded);
    push @$errors, "census evidence '$id' has unknown outcome '$outcome'" if !$valid_outcome{$outcome};
    $outcome_counts->{$outcome}++ if $valid_outcome{$outcome};
    if ($outcome eq 'derived' || $outcome eq 'identity_gated') {
        push @$errors, "census evidence '$id' lacks verifier" if ref($record->{verifier}) ne 'HASH';
        validate_verifier($base, $record->{verifier}, $execute, $tracked, "census evidence '$id'", $errors)
            if ref($record->{verifier}) eq 'HASH';
        my $authority_id = $record->{source_authority_id};
        if (defined $authority_id) {
            my $authority = $derived->{$authority_id};
            push @$errors, "census evidence '$id' references unknown source_authority_id '$authority_id'"
                if !$authority;
            if ($authority) {
                push @$errors, "census evidence '$id' source authority surface differs"
                    if ($authority->{surface_id} // '') ne $surface_id;
                push @$errors, "census evidence '$id' source authority path differs"
                    if ($authority->{path} // '') ne ($path // '');
                my $marker = $authority->{field_marker};
                push @$errors, "census evidence '$id' exact region omits source authority marker"
                    if defined($marker) && index($region_text, $marker) < 0;
            }
        }
        reject_present($record, $id, $errors, qw(claim_id missing_legs scope_reason));
    } elsif ($outcome eq 'registered') {
        my $claim_id = required_slug($record->{claim_id}, "census evidence '$id' claim_id", $errors);
        push @$errors, "census evidence '$id' references unknown claim_id '$claim_id'"
            if defined($claim_id) && !$claims->{$claim_id};
        push @$errors, "census evidence '$id' references superseded claim_id '$claim_id'"
            if defined($claim_id) && $claims->{$claim_id} && ($claims->{$claim_id}{status} // '') eq 'superseded';
        reject_present($record, $id, $errors, qw(verifier missing_legs scope_reason source_authority_id));
    } elsif ($outcome eq 'incomplete') {
        my @legs = scalar_array($record->{missing_legs}, "census evidence '$id' missing_legs", $errors);
        my %valid_leg = map { $_ => 1 } qw(rederive falsification durability);
        my %seen;
        push @$errors, "census evidence '$id' has no missing leg" if !@legs;
        for my $leg (@legs) {
            push @$errors, "census evidence '$id' has unknown missing leg '$leg'" if !$valid_leg{$leg};
            push @$errors, "census evidence '$id' duplicates missing leg '$leg'" if $seen{$leg}++;
        }
        reject_present($record, $id, $errors, qw(verifier claim_id scope_reason source_authority_id));
    } elsif ($outcome eq 'excluded') {
        required_slug($record->{scope_reason}, "census evidence '$id' scope_reason", $errors);
        reject_present($record, $id, $errors, qw(verifier claim_id missing_legs source_authority_id));
    }
    $surface_evidence->{$surface_id}++ if $surface;
    $view_evidence->{$view_id}++ if $views->{$view_id};
}

sub validate_region {
    my ($base, $path, $region, $label, $errors) = @_;
    if (ref($region) ne 'HASH') {
        push @$errors, "$label region must be an object";
        return '';
    }
    reject_unknown($region, "$label region", $errors, qw(kind start_line end_line sha256));
    exact_scalar($region->{kind}, 'line_range_sha256', "$label region kind", $errors);
    my $start = positive_integer($region->{start_line}, "$label region start_line", $errors) // 0;
    my $end = positive_integer($region->{end_line}, "$label region end_line", $errors) // 0;
    push @$errors, "$label region end_line precedes start_line" if $end && $start && $end < $start;
    my $expected = $region->{sha256};
    push @$errors, "$label region sha256 must be 64 lowercase hex characters"
        if !defined($expected) || ref($expected) || $expected !~ /\A[0-9a-f]{64}\z/;
    return '' if !defined($path) || ref($path) || !safe_relative($path);
    my $absolute = absolute($base, $path);
    return '' if !-f $absolute || -l $absolute;
    my $raw = read_raw($absolute, $errors, $path);
    my @lines = ($raw =~ /.*?(?:\n|\z)/g);
    pop @lines if @lines && $lines[-1] eq '';
    if (!$start || !$end || $end > @lines) {
        push @$errors, "$label region $start-$end exceeds $path line count " . scalar(@lines);
        return '';
    }
    my $text = join('', @lines[$start - 1 .. $end - 1]);
    my $actual = sha256_hex($text);
    push @$errors, "$label exact region is stale: SHA-256 $actual != $expected"
        if defined($expected) && $expected =~ /\A[0-9a-f]{64}\z/ && $actual ne $expected;
    return $text;
}

sub validate_verifier {
    my ($base, $verifier, $execute, $tracked, $label, $errors) = @_;
    reject_unknown($verifier, "$label verifier", $errors,
        qw(argv producer inputs expected_exit stdout_contains));
    my @argv = scalar_array($verifier->{argv}, "$label verifier argv", $errors);
    push @$errors, "$label verifier argv is empty" if !@argv;
    for my $arg (@argv) {
        push @$errors, "$label verifier argv contains an absolute or parent-relative argument '$arg'"
            if $arg =~ m{\A/} || $arg =~ m{(?:\A|/)\.\.(?:/|\z)};
    }
    my $producer = $verifier->{producer};
    validate_tracked_file($base, $producer, $tracked, "$label verifier producer", $errors);
    if (@argv && defined($producer) && !ref($producer)) {
        my $invokes = $argv[0] eq $producer
            || (($argv[0] eq 'perl' || $argv[0] eq 'bash') && @argv > 1 && $argv[1] eq $producer);
        push @$errors, "$label verifier argv does not invoke producer '$producer'" if !$invokes;
    }
    my @inputs = scalar_array($verifier->{inputs}, "$label verifier inputs", $errors);
    push @$errors, "$label verifier has no inputs" if !@inputs;
    my %seen;
    for my $input (@inputs) {
        push @$errors, "$label verifier duplicates input '$input'" if $seen{$input}++;
        validate_tracked_file($base, $input, $tracked, "$label verifier input", $errors);
    }
    my $expected_exit = nonnegative_integer($verifier->{expected_exit}, "$label verifier expected_exit", $errors);
    my $contains = required_scalar($verifier->{stdout_contains}, "$label verifier stdout_contains", $errors);
    if ($execute && @argv && defined($expected_exit) && defined($contains)) {
        my ($stdout, $stderr, $exit) = capture_command($base, \@argv);
        push @$errors, "$label verifier exited $exit, expected $expected_exit: $stderr"
            if $exit != $expected_exit;
        push @$errors, "$label verifier stdout omits '$contains'"
            if index($stdout, $contains) < 0;
    }
}

sub produce_candidates {
    my ($result) = @_;
    my $base = $result->{root};
    my %candidate;
    for my $surface_id (sort keys %{$result->{surface_contract}}) {
        my $surface = $result->{surface_contract}{$surface_id};
        next if ($surface->{disposition} // '') ne 'included';
        my ($path) = sort grep { ($result->{path_owner}{$_} // '') eq $surface_id }
            keys %{$result->{path_owner}};
        next if !defined $path;
        my ($line, $text) = first_nonblank_line($base, $path);
        for my $view (@{$surface->{views}}) {
            add_candidate(\%candidate, $surface_id, $view, $path, $line, $text,
                'surface_review', 'review_required');
        }
    }
    for my $authority (@{$result->{derived_source}}) {
        my $surface_id = $authority->{surface_id} // next;
        my $surface = $result->{surface_contract}{$surface_id} // next;
        next if ($surface->{disposition} // '') ne 'included';
        my $path = $authority->{path} // next;
        my $marker = $authority->{field_marker} // next;
        my ($line, $text) = unique_marker_line($base, $path, $marker);
        next if !$line;
        my $class = $authority->{classification} // '';
        my $suggestion = $class eq 'authored_intent' || $class eq 'immutable_evidence'
            ? 'excluded' : 'derived';
        add_candidate(\%candidate, $surface_id, $surface->{views}[0], $path, $line, $text,
            'derived_state:' . ($authority->{contract_id} // 'unknown'), $suggestion);
    }
    for my $path (sort keys %{$result->{path_owner}}) {
        my $surface_id = $result->{path_owner}{$path};
        my $surface = $result->{surface_contract}{$surface_id} // next;
        next if ($surface->{disposition} // '') ne 'included';
        my $raw = read_raw(absolute($base, $path), [], $path);
        my @lines = ($raw =~ /.*?(?:\n|\z)/g);
        pop @lines if @lines && $lines[-1] eq '';
        for my $index (0 .. $#lines) {
            while ($lines[$index] =~ /\[claim:\s*([a-z0-9]+(?:-[a-z0-9]+)*)\]/g) {
                add_candidate(\%candidate, $surface_id, $surface->{views}[0], $path,
                    $index + 1, $lines[$index], "registered:$1", 'registered');
            }
        }
    }
    return [map { $candidate{$_} } sort keys %candidate];
}

sub add_candidate {
    my ($set, $surface, $view, $path, $line, $text, $basis, $suggestion) = @_;
    return if !$line || !defined($text) || $text eq '';
    my $key = join(':', $surface, $view, $path, $line);
    if ($set->{$key}) {
        if ($suggestion ne 'review_required') {
            $set->{$key}{basis} = $basis;
            $set->{$key}{suggested_outcome} = $suggestion;
        }
        return;
    }
    $set->{$key} = {
        record_type => 'candidate', schema_version => 1, candidate_id => $key,
        surface_id => $surface, view_id => $view, path => $path,
        region => {kind => 'line_range_sha256', start_line => $line, end_line => $line,
            sha256 => sha256_hex($text)},
        basis => $basis, suggested_outcome => $suggestion,
    };
}

sub first_nonblank_line {
    my ($base, $path) = @_;
    my $raw = read_raw(absolute($base, $path), [], $path);
    my @lines = ($raw =~ /.*?(?:\n|\z)/g);
    pop @lines if @lines && $lines[-1] eq '';
    for my $index (0 .. $#lines) {
        return ($index + 1, $lines[$index]) if $lines[$index] =~ /\S/;
    }
    return;
}

sub unique_marker_line {
    my ($base, $path, $marker) = @_;
    return if !safe_relative($path) || !-f absolute($base, $path);
    my $raw = read_raw(absolute($base, $path), [], $path);
    my @lines = ($raw =~ /.*?(?:\n|\z)/g);
    pop @lines if @lines && $lines[-1] eq '';
    my @matches = grep { index($lines[$_], $marker) >= 0 } 0 .. $#lines;
    return if @matches != 1;
    return ($matches[0] + 1, $lines[$matches[0]]);
}

sub source_records {
    my ($base, $source, $label, $errors) = @_;
    return [] if !$source || !safe_relative($source->{path});
    my $raw = read_raw(absolute($base, $source->{path}), $errors, $label);
    my @records;
    my $line_number = 0;
    for my $line (split /\n/, $raw, -1) {
        $line_number++;
        next if $line eq '';
        my $record = eval { JSON::PP->new->decode($line) };
        if ($@ || ref($record) ne 'HASH') {
            push @$errors, "$label line $line_number is not one JSON object";
            next;
        }
        next if ($record->{record_type} // '') eq 'registry';
        push @records, $record;
    }
    return \@records;
}

sub read_bounded_jsonl {
    my ($path, $label, $hard_records, $hard_bytes, $hard_record_bytes, $errors) = @_;
    if (!-f $path || -l $path) {
        push @$errors, "$label is missing or not a regular file";
        return ({}, []);
    }
    my $file_bytes = -s $path;
    push @$errors, "$label exceeds portable hard byte cap $hard_bytes" if $file_bytes > $hard_bytes;
    my $raw = read_raw($path, $errors, $label);
    my (@records, @lengths);
    my $line_number = 0;
    for my $line (split /\n/, $raw, -1) {
        $line_number++;
        next if $line_number > 1 && $line eq '' && $line_number == 1 + scalar(split /\n/, $raw);
        if ($line eq '') {
            push @$errors, "$label line $line_number is blank";
            next;
        }
        push @$errors, "$label line $line_number exceeds portable raw-record cap $hard_record_bytes"
            if length(encode_utf8($line)) > $hard_record_bytes;
        my $record = eval { JSON::PP->new->decode($line) };
        if ($@ || ref($record) ne 'HASH') {
            push @$errors, "$label line $line_number is not one JSON object";
            next;
        }
        push @records, $record;
        push @lengths, length(encode_utf8($line));
    }
    return ({}, []) if !@records;
    my $meta = shift @records;
    my $max_records = positive_integer($meta->{max_records}, "$label max_records", $errors);
    my $max_bytes = positive_integer($meta->{max_bytes}, "$label max_bytes", $errors);
    my $max_record_bytes = positive_integer($meta->{max_record_bytes}, "$label max_record_bytes", $errors);
    my $max_array_items = positive_integer($meta->{max_array_items}, "$label max_array_items", $errors);
    my $max_scalar_bytes = positive_integer($meta->{max_scalar_bytes}, "$label max_scalar_bytes", $errors);
    push @$errors, "$label max_records exceeds portable hard cap $hard_records"
        if defined($max_records) && $max_records > $hard_records;
    push @$errors, "$label max_bytes exceeds portable hard cap $hard_bytes"
        if defined($max_bytes) && $max_bytes > $hard_bytes;
    push @$errors, "$label max_record_bytes exceeds portable hard cap $hard_record_bytes"
        if defined($max_record_bytes) && $max_record_bytes > $hard_record_bytes;
    push @$errors, "$label max_array_items exceeds portable hard cap 64"
        if defined($max_array_items) && $max_array_items > 64;
    push @$errors, "$label max_scalar_bytes exceeds portable hard cap 2048"
        if defined($max_scalar_bytes) && $max_scalar_bytes > 2_048;
    push @$errors, "$label has more data records than max_records"
        if defined($max_records) && @records > $max_records;
    push @$errors, "$label exceeds declared max_bytes"
        if defined($max_bytes) && $file_bytes > $max_bytes;
    push @$errors, "$label has a record above max_record_bytes"
        if defined($max_record_bytes) && grep { $_ > $max_record_bytes } @lengths;
    if (defined($max_array_items) && defined($max_scalar_bytes)) {
        validate_value_bounds($meta, "$label registry record", $max_array_items, $max_scalar_bytes, $errors);
        validate_value_bounds($_, $label, $max_array_items, $max_scalar_bytes, $errors) for @records;
    }
    return ($meta, \@records);
}

sub validate_value_bounds {
    my ($value, $label, $max_array, $max_scalar, $errors) = @_;
    if (ref($value) eq 'HASH') {
        validate_value_bounds($value->{$_}, "$label.$_", $max_array, $max_scalar, $errors)
            for sort keys %$value;
    } elsif (ref($value) eq 'ARRAY') {
        push @$errors, "$label array exceeds $max_array items" if @$value > $max_array;
        validate_value_bounds($_, $label, $max_array, $max_scalar, $errors) for @$value;
    } elsif (ref($value)) {
        push @$errors, "$label contains an unsupported value type";
    } elsif (defined($value) && length(encode_utf8("$value")) > $max_scalar) {
        push @$errors, "$label scalar exceeds $max_scalar bytes";
    }
}

sub tracked_paths {
    my ($base, $errors) = @_;
    my ($stdout, $stderr, $exit) = capture_command($base, ['git', 'ls-files', '-z']);
    if ($exit != 0) {
        push @$errors, "cannot enumerate tracked paths: $stderr";
        return {};
    }
    my %tracked = map { $_ => 1 } grep { $_ ne '' } split /\0/, $stdout;
    return \%tracked;
}

sub validate_tracked_file {
    my ($base, $path, $tracked, $label, $errors) = @_;
    if (!safe_relative($path)) {
        push @$errors, "$label is unsafe";
        return;
    }
    push @$errors, "$label '$path' is untracked" if !$tracked->{$path};
    my $absolute = absolute($base, $path);
    push @$errors, "$label '$path' is missing, a symlink, or not a regular file"
        if !-f $absolute || -l $absolute;
}

sub validate_sha {
    my ($base, $path, $expected, $label, $errors) = @_;
    push @$errors, "$label sha256 must be 64 lowercase hex characters"
        if !defined($expected) || ref($expected) || $expected !~ /\A[0-9a-f]{64}\z/;
    return if !safe_relative($path) || !-f absolute($base, $path);
    my $actual = sha256_hex(read_raw(absolute($base, $path), $errors, $path));
    push @$errors, "$label is stale: SHA-256 $actual != $expected"
        if defined($expected) && $expected =~ /\A[0-9a-f]{64}\z/ && $actual ne $expected;
}

sub scalar_array {
    my ($value, $label, $errors) = @_;
    if (ref($value) ne 'ARRAY') {
        push @$errors, "$label must be an array";
        return;
    }
    my @result;
    for my $item (@$value) {
        my $scalar = required_scalar($item, "$label item", $errors);
        push @result, $scalar if defined $scalar;
    }
    return @result;
}

sub reject_unknown {
    my ($object, $label, $errors, @allowed) = @_;
    return if ref($object) ne 'HASH';
    my %allowed = map { $_ => 1 } @allowed;
    push @$errors, "$label has unknown field '$_'" for grep { !$allowed{$_} } sort keys %$object;
}

sub reject_present {
    my ($record, $id, $errors, @fields) = @_;
    push @$errors, "census evidence '$id' has incompatible field '$_'"
        for grep { exists $record->{$_} } @fields;
}

sub required_scalar {
    my ($value, $label, $errors) = @_;
    if (!defined($value) || ref($value) || $value eq '' || $value =~ /[\x00-\x1f]/) {
        push @$errors, "$label must be a nonempty control-free scalar";
        return;
    }
    return "$value";
}

sub required_slug {
    my ($value, $label, $errors) = @_;
    my $scalar = required_scalar($value, $label, $errors);
    return if !defined $scalar;
    push @$errors, "$label has invalid identifier shape"
        if $scalar !~ /\A[a-z0-9]+(?:[._:-][a-z0-9]+)*\z/;
    return $scalar;
}

sub exact_scalar {
    my ($value, $expected, $label, $errors) = @_;
    push @$errors, "$label must equal '$expected'"
        if !defined($value) || ref($value) || "$value" ne $expected;
}

sub exact_integer {
    my ($value, $expected, $label, $errors) = @_;
    push @$errors, "$label must equal $expected"
        if !defined($value) || ref($value) || $value !~ /\A[0-9]+\z/ || 0 + $value != $expected;
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

sub safe_relative {
    my ($path) = @_;
    return defined($path) && !ref($path) && $path ne '' && $path !~ m{\A/}
        && $path !~ /\\/ && $path !~ m{(?:\A|/)\.\.(?:/|\z)}
        && $path !~ m{(?:\A|/)\.(?:/|\z)} && $path !~ /[\x00-\x1f]/;
}

sub safe_relative_pattern {
    my ($path) = @_;
    return defined($path) && !ref($path) && $path ne '' && $path !~ m{\A/}
        && $path !~ /\\/ && $path !~ m{(?:\A|/)\.\.(?:/|\z)} && $path !~ /[\x00-\x1f]/;
}

sub glob_regex {
    my ($pattern) = @_;
    my $regex = '';
    my @chars = split //, $pattern;
    for (my $i = 0; $i < @chars; $i++) {
        if ($chars[$i] eq '*' && $i + 1 < @chars && $chars[$i + 1] eq '*') {
            $regex .= '.*';
            $i++;
        } elsif ($chars[$i] eq '*') {
            $regex .= '[^/]*';
        } elsif ($chars[$i] eq '?') {
            $regex .= '[^/]';
        } else {
            $regex .= quotemeta($chars[$i]);
        }
    }
    return qr/\A$regex\z/;
}

sub absolute {
    my ($base, $relative) = @_;
    return File::Spec->catfile($base, split m{/}, $relative);
}

sub read_raw {
    my ($path, $errors, $label) = @_;
    open my $fh, '<:raw', $path or do {
        push @$errors, "cannot read $label: $!";
        return '';
    };
    local $/;
    my $raw = <$fh> // '';
    close $fh or push @$errors, "cannot close $label: $!";
    return $raw;
}

sub capture_command {
    my ($base, $argv) = @_;
    my $old = getcwd();
    chdir $base or return ('', "cannot chdir to $base: $!", 127);
    my ($in, $out);
    my $err = gensym;
    my $pid = eval { open3($in, $out, $err, @$argv) };
    if ($@) {
        chdir $old or die "current-claim-census: cannot restore cwd: $!\n";
        return ('', "cannot execute '$argv->[0]': $@", 127);
    }
    close $in;
    my $select = IO::Select->new($out, $err);
    my ($stdout, $stderr) = ('', '');
    my %stdout_fd = (fileno($out) => 1);
    while (my @ready = $select->can_read) {
        for my $fh (@ready) {
            my $fd = fileno($fh);
            my $buffer = '';
            my $read = sysread($fh, $buffer, 8192);
            if (!defined($read) || $read == 0) {
                $select->remove($fh);
                close $fh;
            } elsif ($stdout_fd{$fd}) {
                $stdout .= $buffer;
            } else {
                $stderr .= $buffer;
            }
        }
    }
    waitpid($pid, 0);
    my $exit = $? == -1 ? 127 : ($? >> 8);
    chdir $old or die "current-claim-census: cannot restore cwd: $!\n";
    return ($stdout, $stderr, $exit);
}

sub run_self_test {
    my $generated = absolute($script_root, 'generated');
    make_path($generated) if !-d $generated;
    my $fixture = File::Spec->catdir($generated, ".current-claim-census-self-test.$$");
    die "current-claim-census self-test: unsafe fixture path\n"
        if index($fixture, $generated . File::Spec->catfile('')) != 0;
    remove_tree($fixture) if -e $fixture;
    make_path(absolute($fixture, 'doctrine/claim_verification'));
    make_path(absolute($fixture, 'doctrine/live_document_size'));
    make_path(absolute($fixture, 'scripts'));
    write_raw(absolute($fixture, 'status.md'), "Status 2 current\n");
    write_raw(absolute($fixture, 'fixture.md'), "Fixture literal 7\n");
    write_raw(absolute($fixture, 'archive.md'), "Historical 1\n");
    write_raw(absolute($fixture, 'frozen.md'), "Frozen 1\n");
    write_raw(absolute($fixture, 'scripts/probe.pl'), "#!/usr/bin/env perl\nprint qq{census source PASS\\n};\n");
    my $source_meta = {record_type => 'registry', schema_version => 1, max_records => 16,
        max_bytes => 16_384, max_record_bytes => 4_096, max_array_items => 8, max_scalar_bytes => 512};
    my @surface_records = (
        $source_meta,
        {surface_id => 'status', lifecycle => 'bounded_snapshot', targets => ['status.md']},
        {surface_id => 'fixture', lifecycle => 'bounded_snapshot', targets => ['fixture.md']},
        {surface_id => 'archive', lifecycle => 'archive_terminal', targets => ['archive.md']},
        {surface_id => 'frozen', lifecycle => 'frozen_legacy', targets => ['frozen.md']},
    );
    my @derived_records = (
        $source_meta,
        {record_type => 'contract', contract_id => 'status-authority', surface_id => 'status',
            path => 'status.md', field_marker => 'Status 2 current', classification => 'verified_copy'},
    );
    my @claim_records = (
        $source_meta,
        {record_type => 'claim', claim_id => 'fixture-claim', status => 'verified'},
    );
    write_jsonl(absolute($fixture, 'doctrine/live_document_size/surfaces.jsonl'), \@surface_records);
    write_jsonl(absolute($fixture, 'doctrine/live_document_size/derived_state_contracts.jsonl'), \@derived_records);
    write_jsonl(absolute($fixture, 'doctrine/claim_verification/claims.jsonl'), \@claim_records);
    command_ok($fixture, ['git', 'init', '-q']);
    command_ok($fixture, ['git', 'config', 'user.email', 'census-self-test@example.invalid']);
    command_ok($fixture, ['git', 'config', 'user.name', 'Census Self Test']);

    my $meta = {record_type => 'registry', schema_version => 1, phase => 'frozen',
        expected_current_surfaces => 2, max_records => 32, max_bytes => 32_768,
        max_record_bytes => 4_096, max_array_items => 8, max_scalar_bytes => 512,
        required_views => [qw(current_status roadmap_controller_projections maintained_references doctrine_baselines mdbook_quantitative_claims)]};
    my @sources = map {
        my ($id, $path) = @$_;
        $id eq 'claim_registry'
            ? {record_type => 'source', schema_version => 1, source_id => $id, path => $path,
                identity => 'live_validated'}
            : {record_type => 'source', schema_version => 1, source_id => $id, path => $path,
                sha256 => sha256_hex(read_raw(absolute($fixture, $path), [], $path))}
    } (
        ['surface_registry', 'doctrine/live_document_size/surfaces.jsonl'],
        ['derived_state_registry', 'doctrine/live_document_size/derived_state_contracts.jsonl'],
        ['claim_registry', 'doctrine/claim_verification/claims.jsonl'],
    );
    my @views = map {{record_type => 'view', schema_version => 1, view_id => $_,
        description => "fixture $_ view"}} @{$meta->{required_views}};
    my @surfaces = (
        {record_type => 'surface', schema_version => 1, surface_id => 'status', disposition => 'included',
            views => [@{$meta->{required_views}}]},
        {record_type => 'surface', schema_version => 1, surface_id => 'fixture', disposition => 'excluded',
            scope_reason => 'test_fixture_literals'},
    );
    my $status_text = "Status 2 current\n";
    my @evidence = map {
        my $view = $_;
        {record_type => 'evidence', schema_version => 1, evidence_id => "status-$view",
            claim_key => "status-$view", surface_id => 'status', view_id => $view, path => 'status.md',
            region => {kind => 'line_range_sha256', start_line => 1, end_line => 1,
                sha256 => sha256_hex($status_text)}, outcome => 'derived', source_authority_id => 'status-authority',
            verifier => {argv => ['perl', 'scripts/probe.pl'], producer => 'scripts/probe.pl',
                inputs => ['scripts/probe.pl', 'status.md'], expected_exit => 0,
                stdout_contains => 'census source PASS'}}
    } @{$meta->{required_views}};
    my @base_records = ($meta, @sources, @views, @surfaces, @evidence);
    write_jsonl(absolute($fixture, $contract_rel), \@base_records);
    command_ok($fixture, ['git', 'add', '.']);
    command_ok($fixture, ['git', 'commit', '-qm', 'fixture']);

    my $executed = validate_census(root => $fixture, contract_rel => $contract_rel, execute_commands => 1);
    die "current-claim-census self-test clean execution failed: " . join('; ', @{$executed->{errors}}) . "\n"
        if @{$executed->{errors}};

    my @cases = (
        ['clean frozen census', 1, qr//, sub {}],
        ['clean inventory census', 1, qr//, sub {
            $_[0][0]{phase} = 'inventory';
            @{$_[0]} = grep { ($_->{record_type} // '') ne 'evidence' } @{$_[0]};
        }],
        ['missing current surface', 0, qr/lacks one census disposition/, sub {
            @{$_[0]} = grep { !(($_->{record_type} // '') eq 'surface' && ($_->{surface_id} // '') eq 'fixture') } @{$_[0]};
        }],
        ['unknown view join', 0, qr/references unknown view/, sub { $_[0][-1]{view_id} = 'unknown_view' }],
        ['missing required view evidence', 0, qr/required view 'mdbook_quantitative_claims' has no frozen evidence unit/, sub {
            @{$_[0]} = grep {
                !(($_->{record_type} // '') eq 'evidence'
                    && ($_->{view_id} // '') eq 'mdbook_quantitative_claims')
            } @{$_[0]};
        }],
        ['duplicate surface', 0, qr/duplicates surface_id/, sub {
            my ($surface) = grep { ($_->{record_type} // '') eq 'surface' } @{$_[0]};
            push @{$_[0]}, clone($surface);
        }],
        ['stale source digest', 0, qr/is stale: SHA-256/, sub { $_[0][1]{sha256} = '0' x 64 }],
        ['untracked evidence path', 0, qr/is untracked/, sub {
            write_raw(absolute($fixture, 'untracked.md'), "Untracked\n");
            $_[0][-1]{path} = 'untracked.md';
            $_[0][-1]{region}{sha256} = sha256_hex("Untracked\n");
        }],
        ['stale exact region', 0, qr/exact region is stale/, sub { $_[0][-1]{region}{sha256} = '0' x 64 }],
        ['unknown registered claim', 0, qr/references unknown claim_id/, sub {
            $_[0][-1]{outcome} = 'registered';
            $_[0][-1]{claim_id} = 'missing-claim';
            delete $_[0][-1]{verifier};
            delete $_[0][-1]{source_authority_id};
        }],
        ['missing executable verifier', 0, qr/lacks verifier/, sub { delete $_[0][-1]{verifier} }],
        ['duplicate evidence identity', 0, qr/duplicates evidence_id/, sub { push @{$_[0]}, clone($_[0][-1]) }],
        ['unknown evidence field', 0, qr/unknown field 'surprise'/, sub { $_[0][-1]{surprise} = 1 }],
        ['excluded surface missing reason', 0, qr/scope_reason must be/, sub {
            my ($surface) = grep { ($_->{surface_id} // '') eq 'fixture' } @{$_[0]};
            delete $surface->{scope_reason};
        }],
        ['portable record bound', 0, qr/max_records exceeds portable hard cap/, sub { $_[0][0]{max_records} = 257 }],
    );
    my ($passed, $total) = (0, 0);
    for my $case (@cases) {
        $total++;
        unlink absolute($fixture, 'untracked.md') if -e absolute($fixture, 'untracked.md');
        my ($name, $expected_ok, $diagnostic, $mutate) = @$case;
        my $records = clone(\@base_records);
        $mutate->($records);
        write_jsonl(absolute($fixture, $contract_rel), $records);
        my $result = validate_census(root => $fixture, contract_rel => $contract_rel, execute_commands => 0);
        my $ok = @{$result->{errors}} ? 0 : 1;
        my $joined = join("\n", @{$result->{errors}});
        die "current-claim-census self-test '$name' expected " . ($expected_ok ? 'PASS' : 'RED')
            . ", got " . ($ok ? 'PASS' : "RED: $joined") . "\n"
            if $ok != $expected_ok;
        die "current-claim-census self-test '$name' missed diagnostic $diagnostic: $joined\n"
            if !$expected_ok && $joined !~ $diagnostic;
        $passed++;
    }
    remove_tree($fixture);
    die "current-claim-census self-test: residue remains at $fixture\n" if -e $fixture;
    print "current-claim-census: self-test $passed/$total positive, missing, unknown, duplicate, untracked, stale, and bound cases pass.\n";
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
    open my $fh, '>:raw', $path or die "current-claim-census self-test: cannot write $path: $!\n";
    print {$fh} $raw or die "current-claim-census self-test: cannot write bytes to $path: $!\n";
    close $fh or die "current-claim-census self-test: cannot close $path: $!\n";
}

sub clone {
    my ($value) = @_;
    my $json = JSON::PP->new->canonical(1);
    return $json->decode($json->encode($value));
}

sub command_ok {
    my ($base, $argv) = @_;
    my ($stdout, $stderr, $exit) = capture_command($base, $argv);
    die "current-claim-census self-test: '@$argv' failed ($exit): $stdout$stderr\n" if $exit != 0;
}
