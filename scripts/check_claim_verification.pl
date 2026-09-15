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
my $root = $script_root;
my $registry_rel = 'doctrine/claim_verification/claims.jsonl';
my $mode = 'check';
my $probe_id;

while (@ARGV) {
    my $arg = shift @ARGV;
    if ($arg eq '--root') {
        my $candidate = shift @ARGV // usage();
        $root = abs_path($candidate) // die "claim-verification: root does not exist\n";
    } elsif ($arg eq '--registry') {
        $registry_rel = shift @ARGV // usage();
    } elsif ($arg eq '--check') {
        $mode = 'check';
    } elsif ($arg eq '--report') {
        $mode = 'report';
    } elsif ($arg eq '--self-test') {
        $mode = 'self-test';
    } elsif ($arg eq '--probe') {
        $mode = 'probe';
        $probe_id = shift @ARGV // usage();
    } else {
        usage();
    }
}

if ($mode eq 'self-test') {
    run_self_test();
    exit 0;
}
if ($mode eq 'probe') {
    run_probe($probe_id);
    exit 0;
}

safe_relative($registry_rel)
    or die "claim-verification: registry path is unsafe: '$registry_rel'\n";
my $result = validate_registry(
    root => $root,
    registry_rel => $registry_rel,
    execute_commands => 1,
    check_publication => 1,
);
if (@{$result->{errors}}) {
    print STDERR "claim-verification: $_\n" for @{$result->{errors}};
    print STDERR "claim-verification: FAILED with " . scalar(@{$result->{errors}}) . " violation(s).\n";
    exit 1;
}

if ($mode eq 'report') {
    print JSON::PP->new->canonical(1)->encode({
        claims => $result->{claim_count},
        commands_executed => $result->{commands_executed},
        publication_source => $result->{publication_source},
        published_claims => $result->{published_claims},
        control_audit => $result->{control_audit},
        statuses => $result->{statuses},
    }), "\n";
} else {
    print "claim-verification: $result->{claim_count} claims, "
        . "$result->{commands_executed} source/control commands, and "
        . "$result->{publication_source} publication resolve with current tracked evidence.\n";
}

sub usage {
    die "Usage: $0 [--check|--report|--self-test|--probe CLAIM_ID] "
        . "[--root DIR] [--registry PATH]\n";
}

sub validate_registry {
    my (%args) = @_;
    my $base = $args{root};
    my $relative = $args{registry_rel};
    my $execute = $args{execute_commands} // 0;
    my $check_publication = $args{check_publication} // 0;
    my @errors;

    my $path = absolute($base, $relative);
    if (!-f $path || -l $path) {
        return empty_result("registry '$relative' must be a regular non-symlink file");
    }
    my (undef, $registry_stderr, $registry_exit) =
        capture_command($base, ['git', 'ls-files', '--error-unmatch', '--', $relative]);
    push @errors, "registry '$relative' is untracked" if $registry_exit != 0;
    my $raw = read_raw($path, \@errors, "registry '$relative'");
    return empty_result(@errors) if @errors;
    push @errors, "registry must end with one newline" if $raw !~ /\n\z/;
    my @lines = split /\n/, $raw, -1;
    pop @lines if @lines && $lines[-1] eq '';
    push @errors, "registry must not contain blank records"
        if grep { $_ eq '' } @lines;
    return empty_result(@errors, 'registry is empty') if !@lines;

    my $json = JSON::PP->new->utf8(1);
    my @records;
    for my $index (0 .. $#lines) {
        my $record = eval { $json->decode($lines[$index]) };
        if ($@ || ref($record) ne 'HASH') {
            push @errors, "record " . ($index + 1) . " is not a JSON object";
            next;
        }
        push @records, { value => $record, bytes => length(encode_utf8($lines[$index])) + 1 };
    }
    return empty_result(@errors) if @errors;

    my $meta = $records[0]{value};
    validate_meta($meta, \@errors);
    my $limits = limits_from($meta);
    if ($limits) {
        push @errors, "registry bytes " . length($raw) . " exceed max_bytes $limits->{max_bytes}"
            if length($raw) > $limits->{max_bytes};
        push @errors, "registry has " . (@records - 1) . " claims, exceeding max_records $limits->{max_records}"
            if @records - 1 > $limits->{max_records};
        for my $index (0 .. $#records) {
            push @errors, "record " . ($index + 1) . " bytes $records[$index]{bytes} exceed max_record_bytes $limits->{max_record_bytes}"
                if $records[$index]{bytes} > $limits->{max_record_bytes};
            validate_value_bounds($records[$index]{value}, "record " . ($index + 1), $limits, \@errors);
        }
    }

    my %claims;
    my %statuses;
    my $commands_executed = 0;
    for my $index (1 .. $#records) {
        my $claim = $records[$index]{value};
        my $label = "record " . ($index + 1);
        my $id = validate_claim_shape($claim, $label, \@errors);
        next if !defined $id;
        if (exists $claims{$id}) {
            push @errors, "$label duplicates claim_id '$id'";
            next;
        }
        $claims{$id} = $claim;
        $statuses{$claim->{status} // 'invalid'}++;
    }

    for my $id (sort keys %claims) {
        my $claim = $claims{$id};
        validate_claim_semantics($base, $claim, \%claims, \@errors);
    }
    my $control_audit = audit_claim_controls_and_producers($base, \%claims, \@errors);

    if ($execute && !@errors) {
        for my $id (sort keys %claims) {
            my $claim = $claims{$id};
            next if ($claim->{status} // '') ne 'verified';
            for my $command (@{$claim->{rederive}{commands}}, @{$claim->{falsification}{controls}}) {
                execute_declared_command($base, $id, $command, \@errors);
                $commands_executed++;
            }
        }
    }

    my ($publication_source, $published_claims) = ('not-checked', []);
    if ($check_publication && !@errors) {
        my $message_rel = 'git_message_brief.txt';
        my $message_path = absolute($base, $message_rel);
        my $message = '';
        if (-f $message_path && -s $message_path) {
            $message = read_raw($message_path, \@errors, $message_rel);
            $publication_source = $message_rel;
        } else {
            my ($stdout, $stderr, $exit) = capture_command($base, ['git', 'log', '-1', '--format=%B']);
            push @errors, "cannot read HEAD publication declaration: $stderr" if $exit != 0;
            $message = $stdout;
            $publication_source = 'HEAD';
        }
        if (!@errors) {
            $published_claims = validate_publication_message($message, \%claims, \@errors);
        }
    }

    return {
        errors => \@errors,
        claim_count => scalar(keys %claims),
        commands_executed => $commands_executed,
        publication_source => $publication_source,
        published_claims => $published_claims,
        control_audit => $control_audit,
        statuses => \%statuses,
    };
}

sub validate_meta {
    my ($meta, $errors) = @_;
    # LIVE-DOCUMENT-PRESSURE-HEADROOM.22b — the header also declares the pressure band its own bounds
    # are reported against; the band itself is computed centrally in check_live_document_size.pl.
    reject_unknown($meta, 'registry control', $errors, qw(
        record_type schema_version max_records max_bytes max_record_bytes max_array_items
        max_scalar_bytes allowed_statuses milestones
    ));
    exact_scalar($meta->{record_type}, 'registry', 'registry control.record_type', $errors);
    exact_integer($meta->{schema_version}, 1, 'registry control.schema_version', $errors);
    my %hard = (
        max_records => 128,
        max_bytes => 131_072,
        max_record_bytes => 32_768,
        max_array_items => 64,
        max_scalar_bytes => 4_096,
    );
    for my $field (sort keys %hard) {
        my $value = positive_integer($meta->{$field}, "registry control.$field", $errors);
        push @$errors, "registry control.$field $value exceeds portable hard cap $hard{$field}"
            if defined($value) && $value > $hard{$field};
    }
    if (ref($meta->{allowed_statuses}) ne 'ARRAY') {
        push @$errors, 'registry control.allowed_statuses must be an array';
    } elsif (join("\0", @{$meta->{allowed_statuses}}) ne join("\0", qw(verified incomplete superseded))) {
        push @$errors, 'registry control.allowed_statuses must be exactly verified, incomplete, superseded';
    }
}

sub limits_from {
    my ($meta) = @_;
    for my $field (qw(max_records max_bytes max_record_bytes max_array_items max_scalar_bytes)) {
        return if !defined($meta->{$field}) || ref($meta->{$field}) || $meta->{$field} !~ /\A[1-9][0-9]*\z/;
    }
    return { map { $_ => 0 + $meta->{$_} } qw(max_records max_bytes max_record_bytes max_array_items max_scalar_bytes) };
}

sub validate_value_bounds {
    my ($value, $label, $limits, $errors) = @_;
    if (ref($value) eq 'ARRAY') {
        push @$errors, "$label has " . scalar(@$value) . " array items, exceeding max_array_items $limits->{max_array_items}"
            if @$value > $limits->{max_array_items};
        validate_value_bounds($value->[$_], "$label\[$_\]", $limits, $errors) for 0 .. $#$value;
    } elsif (ref($value) eq 'HASH') {
        validate_value_bounds($value->{$_}, "$label.$_", $limits, $errors) for sort keys %$value;
    } elsif (!ref($value) && defined($value)) {
        my $bytes = length(encode_utf8("$value"));
        push @$errors, "$label scalar bytes $bytes exceed max_scalar_bytes $limits->{max_scalar_bytes}"
            if $bytes > $limits->{max_scalar_bytes};
    }
}

sub validate_claim_shape {
    my ($claim, $label, $errors) = @_;
    reject_unknown($claim, $label, $errors, qw(
        record_type schema_version claim_id status assertion owner rederive falsification durability
        missing_legs superseded_by
    ));
    exact_scalar($claim->{record_type}, 'claim', "$label.record_type", $errors);
    exact_integer($claim->{schema_version}, 1, "$label.schema_version", $errors);
    my $id = required_scalar($claim->{claim_id}, "$label.claim_id", $errors);
    if (defined($id) && $id !~ /\A[a-z0-9]+(?:-[a-z0-9]+)*\z/) {
        push @$errors, "$label.claim_id '$id' is not a stable lowercase hyphenated id";
    }
    my $status = required_scalar($claim->{status}, "$label.status", $errors);
    push @$errors, "$label.status '$status' is unknown"
        if defined($status) && $status !~ /\A(?:verified|incomplete|superseded)\z/;
    required_scalar($claim->{assertion}, "$label.assertion", $errors);
    required_scalar($claim->{owner}, "$label.owner", $errors);
    return $id;
}

sub validate_claim_semantics {
    my ($base, $claim, $claims, $errors) = @_;
    my $id = $claim->{claim_id} // '<unknown>';
    my $label = "claim '$id'";
    my $status = $claim->{status} // '';
    if ($status eq 'verified') {
        push @$errors, "$label verified claim must not declare missing_legs"
            if exists $claim->{missing_legs};
        push @$errors, "$label verified claim must not declare superseded_by"
            if exists $claim->{superseded_by};
        validate_rederive($base, $claim->{rederive}, "$label.rederive", $errors);
        validate_falsification($base, $claim->{falsification}, "$label.falsification", $errors);
        validate_durability($base, $claim->{durability}, "$label.durability", $errors);
        validate_command_artifact_join($claim, $label, $errors);
    } elsif ($status eq 'incomplete') {
        reject_unknown_optional_legs($claim, $label, $errors);
        if (ref($claim->{missing_legs}) ne 'ARRAY' || !@{$claim->{missing_legs}}) {
            push @$errors, "$label incomplete claim needs nonempty missing_legs";
        } else {
            my %seen;
            for my $leg (@{$claim->{missing_legs}}) {
                push @$errors, "$label missing_legs contains unknown '$leg'"
                    if !defined($leg) || ref($leg) || $leg !~ /\A(?:rederive|falsification|durability)\z/;
                push @$errors, "$label missing_legs duplicates '$leg'" if defined($leg) && $seen{$leg}++;
            }
        }
        push @$errors, "$label incomplete claim must not declare superseded_by" if exists $claim->{superseded_by};
    } elsif ($status eq 'superseded') {
        reject_unknown_optional_legs($claim, $label, $errors);
        my $target = required_scalar($claim->{superseded_by}, "$label.superseded_by", $errors);
        push @$errors, "$label superseded_by '$target' is unknown"
            if defined($target) && !exists $claims->{$target};
        push @$errors, "$label cannot supersede itself" if defined($target) && $target eq $id;
        push @$errors, "$label superseded claim must not declare missing_legs" if exists $claim->{missing_legs};
    }
}

sub reject_unknown_optional_legs {
    my ($claim, $label, $errors) = @_;
    for my $leg (qw(rederive falsification durability)) {
        push @$errors, "$label status '$claim->{status}' must omit $leg" if exists $claim->{$leg};
    }
}

sub validate_rederive {
    my ($base, $leg, $label, $errors) = @_;
    if (ref($leg) ne 'HASH') {
        push @$errors, "$label must be an object";
        return;
    }
    reject_unknown($leg, $label, $errors, qw(boundary result commands));
    required_scalar($leg->{boundary}, "$label.boundary", $errors);
    required_scalar($leg->{result}, "$label.result", $errors);
    validate_commands($base, $leg->{commands}, "$label.commands", $errors);
}

sub validate_falsification {
    my ($base, $leg, $label, $errors) = @_;
    if (ref($leg) ne 'HASH') {
        push @$errors, "$label must be an object";
        return;
    }
    reject_unknown($leg, $label, $errors, qw(competing_hypothesis observed_red result controls));
    required_scalar($leg->{competing_hypothesis}, "$label.competing_hypothesis", $errors);
    required_scalar($leg->{result}, "$label.result", $errors);
    push @$errors, "$label.observed_red must be JSON true"
        if !defined($leg->{observed_red}) || !JSON::PP::is_bool($leg->{observed_red}) || !$leg->{observed_red};
    validate_commands($base, $leg->{controls}, "$label.controls", $errors, 1);
}

sub validate_durability {
    my ($base, $leg, $label, $errors) = @_;
    if (ref($leg) ne 'HASH') {
        push @$errors, "$label must be an object";
        return;
    }
    reject_unknown($leg, $label, $errors, qw(refresh_owner refresh_rule artifacts stale_check retained_evidence));
    required_scalar($leg->{refresh_owner}, "$label.refresh_owner", $errors);
    required_scalar($leg->{refresh_rule}, "$label.refresh_rule", $errors);
    if (ref($leg->{artifacts}) ne 'ARRAY' || !@{$leg->{artifacts}}) {
        push @$errors, "$label.artifacts must be a nonempty array";
    } else {
        my %seen;
        for my $index (0 .. $#{$leg->{artifacts}}) {
            my $artifact = $leg->{artifacts}[$index];
            my $item = "$label.artifacts[$index]";
            if (ref($artifact) ne 'HASH') {
                push @$errors, "$item must be an object";
                next;
            }
            reject_unknown($artifact, $item, $errors, qw(path role sha256));
            my $path = required_scalar($artifact->{path}, "$item.path", $errors);
            my $role = required_scalar($artifact->{role}, "$item.role", $errors);
            my $sha = required_scalar($artifact->{sha256}, "$item.sha256", $errors);
            push @$errors, "$item.role '$role' is unknown"
                if defined($role) && $role !~ /\A(?:canonical_input|producer|control|producer_control|retained_evidence)\z/;
            push @$errors, "$item.sha256 is not lowercase SHA-256"
                if defined($sha) && $sha !~ /\A[0-9a-f]{64}\z/;
            if (defined($path)) {
                push @$errors, "$item.path '$path' is unsafe" if !safe_relative($path);
                push @$errors, "$item.path duplicates '$path'" if $seen{$path}++;
                validate_tracked_artifact($base, $path, $sha, $item, $errors) if safe_relative($path);
            }
        }
    }
    validate_command($base, $leg->{stale_check}, "$label.stale_check", $errors);
    if (ref($leg->{retained_evidence}) ne 'ARRAY' || !@{$leg->{retained_evidence}}) {
        push @$errors, "$label.retained_evidence must be a nonempty array";
    } else {
        my %seen;
        for my $path (@{$leg->{retained_evidence}}) {
            push @$errors, "$label.retained_evidence contains an unsafe path"
                if !defined($path) || ref($path) || !safe_relative($path);
            push @$errors, "$label.retained_evidence duplicates '$path'" if defined($path) && $seen{$path}++;
        }
    }
}

sub validate_commands {
    my ($base, $commands, $label, $errors, $require_red_evidence) = @_;
    if (ref($commands) ne 'ARRAY' || !@$commands) {
        push @$errors, "$label must be a nonempty array";
        return;
    }
    my %seen;
    for my $index (0 .. $#$commands) {
        my $command = $commands->[$index];
        validate_command($base, $command, "$label\[$index\]", $errors, $require_red_evidence);
        if (ref($command) eq 'HASH' && defined($command->{id}) && !ref($command->{id})) {
            push @$errors, "$label duplicates command id '$command->{id}'" if $seen{$command->{id}}++;
        }
    }
}

sub validate_command {
    my ($base, $command, $label, $errors, $require_red_evidence) = @_;
    if (ref($command) ne 'HASH') {
        push @$errors, "$label must be an object";
        return;
    }
    my @fields = qw(id argv producer inputs expected_exit stdout_contains);
    push @fields, 'red_evidence' if $require_red_evidence;
    reject_unknown($command, $label, $errors, @fields);
    required_scalar($command->{id}, "$label.id", $errors);
    my $producer = required_scalar($command->{producer}, "$label.producer", $errors);
    push @$errors, "$label.producer '$producer' is unsafe"
        if defined($producer) && !safe_relative($producer);
    if (ref($command->{argv}) ne 'ARRAY' || !@{$command->{argv}}) {
        push @$errors, "$label.argv must be a nonempty array";
    } else {
        for my $index (0 .. $#{$command->{argv}}) {
            my $arg = $command->{argv}[$index];
            push @$errors, "$label.argv[$index] must be a nonempty scalar"
                if !defined($arg) || ref($arg) || $arg eq '' || $arg =~ /[\x00-\x1f]/;
        }
        if (defined($producer)) {
            my @argv = @{$command->{argv}};
            my $matches = $argv[0] eq $producer
                || (($argv[0] eq 'perl' || $argv[0] eq 'bash') && @argv > 1 && $argv[1] eq $producer);
            push @$errors, "$label.argv does not invoke its declared producer '$producer'" if !$matches;
        }
    }
    if (ref($command->{inputs}) ne 'ARRAY' || !@{$command->{inputs}}) {
        push @$errors, "$label.inputs must be a nonempty array";
    } else {
        my %seen;
        for my $path (@{$command->{inputs}}) {
            push @$errors, "$label.inputs contains unsafe path"
                if !defined($path) || ref($path) || !safe_relative($path);
            push @$errors, "$label.inputs duplicates '$path'" if defined($path) && $seen{$path}++;
        }
    }
    exact_integer($command->{expected_exit}, 0, "$label.expected_exit", $errors);
    required_scalar($command->{stdout_contains}, "$label.stdout_contains", $errors);
    validate_red_evidence($base, $producer, $command->{red_evidence}, "$label.red_evidence", $errors)
        if $require_red_evidence;
}

sub validate_red_evidence {
    my ($base, $producer, $evidence, $label, $errors) = @_;
    if (ref($evidence) ne 'HASH') {
        push @$errors, "$label must be an object";
        return;
    }
    reject_unknown($evidence, $label, $errors, qw(case_id perturbation expected_red source_region));
    my $case_id = required_scalar($evidence->{case_id}, "$label.case_id", $errors);
    push @$errors, "$label.case_id '$case_id' is not a stable lowercase hyphenated id"
        if defined($case_id) && $case_id !~ /\A[a-z0-9]+(?:-[a-z0-9]+)*\z/;
    required_scalar($evidence->{perturbation}, "$label.perturbation", $errors);
    my $expected = required_scalar($evidence->{expected_red}, "$label.expected_red", $errors);
    my $region = $evidence->{source_region};
    if (ref($region) ne 'HASH') {
        push @$errors, "$label.source_region must be an object";
        return;
    }
    reject_unknown($region, "$label.source_region", $errors, qw(kind start_line end_line sha256));
    exact_scalar($region->{kind}, 'line_range_sha256', "$label.source_region.kind", $errors);
    my $start = positive_integer($region->{start_line}, "$label.source_region.start_line", $errors);
    my $end = positive_integer($region->{end_line}, "$label.source_region.end_line", $errors);
    my $sha = required_scalar($region->{sha256}, "$label.source_region.sha256", $errors);
    push @$errors, "$label.source_region.sha256 is not lowercase SHA-256"
        if defined($sha) && $sha !~ /\A[0-9a-f]{64}\z/;
    return if !defined($start) || !defined($end) || !defined($producer) || !safe_relative($producer);
    if ($start > $end) {
        push @$errors, "$label.source_region start_line exceeds end_line";
        return;
    }
    if ($end - $start + 1 > 64) {
        push @$errors, "$label.source_region exceeds 64-line evidence bound";
        return;
    }
    my $path = absolute($base, $producer);
    if (!-f $path || -l $path) {
        push @$errors, "$label producer '$producer' must be a regular non-symlink file";
        return;
    }
    my $raw = read_raw($path, $errors, $producer);
    my @lines = split /(?<=\n)/, $raw;
    if ($end > @lines) {
        push @$errors, "$label.source_region ends at line $end beyond producer line count " . scalar(@lines);
        return;
    }
    my $selected = join '', @lines[$start - 1 .. $end - 1];
    my $actual = sha256_hex($selected);
    push @$errors, "$label.source_region is stale: SHA-256 $actual != $sha"
        if defined($sha) && $sha =~ /\A[0-9a-f]{64}\z/ && $actual ne $sha;
    push @$errors, "$label.source_region omits expected RED diagnostic '$expected'"
        if defined($expected) && index($selected, $expected) < 0;
}

sub audit_claim_controls_and_producers {
    my ($base, $claims, $errors) = @_;
    my %producer;
    my ($controls, $red_evidence) = (0, 0);
    for my $id (sort keys %$claims) {
        my $claim = $claims->{$id};
        next if ($claim->{status} // '') ne 'verified';
        my @commands = (
            ref($claim->{rederive}{commands}) eq 'ARRAY' ? @{$claim->{rederive}{commands}} : (),
            ref($claim->{falsification}{controls}) eq 'ARRAY' ? @{$claim->{falsification}{controls}} : (),
            ref($claim->{durability}{stale_check}) eq 'HASH' ? ($claim->{durability}{stale_check}) : (),
        );
        $producer{$_->{producer}} = 1
            for grep { ref($_) eq 'HASH' && defined($_->{producer}) && !ref($_->{producer}) } @commands;
        for my $control (ref($claim->{falsification}{controls}) eq 'ARRAY'
                ? @{$claim->{falsification}{controls}} : ()) {
            next if ref($control) ne 'HASH';
            $controls++;
            $red_evidence++ if ref($control->{red_evidence}) eq 'HASH';
        }
    }
    my ($untracked, $ignored) = (0, 0);
    for my $scan (
        ['untracked', ['git', 'ls-files', '-z', '--others', '--exclude-standard', '--', qw(scripts doctrine docs .github)]],
        ['ignored', ['git', 'ls-files', '-z', '--others', '--ignored', '--exclude-standard', '--', qw(scripts doctrine docs .github)]],
    ) {
        my ($kind, $argv) = @$scan;
        my ($stdout, $stderr, $exit) = capture_command($base, $argv);
        if ($exit != 0) {
            push @$errors, "cannot census $kind governed producer candidates: $stderr";
            next;
        }
        for my $path (grep { $_ ne '' } split /\0/, $stdout) {
            next if !producer_shaped($path);
            $kind eq 'ignored' ? $ignored++ : $untracked++;
            push @$errors, "$kind producer-shaped path '$path' exists under governed source roots";
        }
    }
    return {
        cited_controls => $controls,
        exact_red_evidence => $red_evidence,
        governed_producers => scalar(keys %producer),
        ignored_candidates => $ignored,
        untracked_candidates => $untracked,
    };
}

sub producer_shaped {
    my ($path) = @_;
    return $path =~ m{\A(?:scripts|doctrine|docs|[.]github)/}
        && $path =~ /(?:[.](?:pl|pm|sh|py|rb|js|ts|rs)|(?:\A|\/)Makefile)\z/;
}

sub validate_command_artifact_join {
    my ($claim, $label, $errors) = @_;
    return if ref($claim->{durability}) ne 'HASH' || ref($claim->{durability}{artifacts}) ne 'ARRAY';
    my %artifact = map {
        ref($_) eq 'HASH' && defined($_->{path}) && !ref($_->{path}) ? ($_->{path} => $_) : ()
    } @{$claim->{durability}{artifacts}};
    my @commands = (
        ref($claim->{rederive}{commands}) eq 'ARRAY' ? @{$claim->{rederive}{commands}} : (),
        ref($claim->{falsification}{controls}) eq 'ARRAY' ? @{$claim->{falsification}{controls}} : (),
    );
    for my $command (@commands) {
        next if ref($command) ne 'HASH';
        my $producer = $command->{producer};
        push @$errors, "$label command '$command->{id}' producer '$producer' is absent from durability.artifacts"
            if defined($producer) && !exists $artifact{$producer};
        for my $input (ref($command->{inputs}) eq 'ARRAY' ? @{$command->{inputs}} : ()) {
            push @$errors, "$label command '$command->{id}' input '$input' is absent from durability.artifacts"
                if defined($input) && !exists $artifact{$input};
        }
    }
    my $stale = $claim->{durability}{stale_check};
    if (ref($stale) eq 'HASH') {
        my %watched = map { $_ => 1 } grep { defined && !ref } (
            $stale->{producer},
            ref($stale->{inputs}) eq 'ARRAY' ? @{$stale->{inputs}} : (),
        );
        for my $path (sort keys %artifact) {
            push @$errors, "$label stale_check omits artifact '$path'" if !$watched{$path};
        }
        for my $path (sort keys %watched) {
            push @$errors, "$label stale_check names unknown artifact '$path'" if !exists $artifact{$path};
        }
    }
    my %retained = map { $_ => 1 } grep { defined && !ref }
        (ref($claim->{durability}{retained_evidence}) eq 'ARRAY'
            ? @{$claim->{durability}{retained_evidence}} : ());
    for my $path (sort keys %retained) {
        push @$errors, "$label retained evidence '$path' is absent from durability.artifacts"
            if !exists $artifact{$path};
        push @$errors, "$label retained evidence '$path' lacks retained_evidence role"
            if exists($artifact{$path}) && ($artifact{$path}{role} // '') ne 'retained_evidence';
    }
}

sub validate_tracked_artifact {
    my ($base, $path, $sha, $label, $errors) = @_;
    my $absolute = absolute($base, $path);
    if (!-f $absolute || -l $absolute) {
        push @$errors, "$label '$path' must be a regular non-symlink file";
        return;
    }
    my (undef, $stderr, $exit) = capture_command($base, ['git', 'ls-files', '--error-unmatch', '--', $path]);
    if ($exit != 0) {
        push @$errors, "$label '$path' is untracked";
        return;
    }
    return if !defined($sha) || $sha !~ /\A[0-9a-f]{64}\z/;
    my $actual = sha256_hex(read_raw($absolute, $errors, $path));
    push @$errors, "$label '$path' is stale: SHA-256 $actual != $sha" if $actual ne $sha;
}

sub execute_declared_command {
    my ($base, $claim_id, $command, $errors) = @_;
    return if ref($command) ne 'HASH' || ref($command->{argv}) ne 'ARRAY';
    my ($stdout, $stderr, $exit) = capture_command($base, $command->{argv});
    if ($exit != ($command->{expected_exit} // -1)) {
        push @$errors, "claim '$claim_id' command '$command->{id}' exited $exit: $stderr";
        return;
    }
    my $needle = $command->{stdout_contains} // '';
    push @$errors, "claim '$claim_id' command '$command->{id}' output omitted expected marker '$needle'"
        if $needle ne '' && index($stdout, $needle) < 0;
}

sub validate_publication_message {
    my ($message, $claims, $errors) = @_;
    my @lines = ($message =~ /^Published-claims:\s*(.*?)\s*$/mg);
    if (@lines != 1) {
        push @$errors, 'publication message must contain exactly one Published-claims: declaration';
        return [];
    }
    my $value = $lines[0];
    if ($value eq 'none') {
        return [];
    }
    my @ids = split /\s*,\s*/, $value, -1;
    my %seen;
    for my $id (@ids) {
        push @$errors, "publication declaration contains invalid claim id '$id'"
            if $id !~ /\A[a-z0-9]+(?:-[a-z0-9]+)*\z/;
        push @$errors, "publication declaration duplicates claim id '$id'" if $seen{$id}++;
        push @$errors, "publication declaration references unknown claim id '$id'"
            if !exists $claims->{$id};
        push @$errors, "publication declaration references superseded claim id '$id'"
            if exists($claims->{$id}) && ($claims->{$id}{status} // '') eq 'superseded';
    }
    return \@ids;
}

sub run_probe {
    my ($id) = @_;
    my @errors;
    if ($id eq 'claim-verification-contract-published') {
        require_literal('CLAIM_VERIFICATION.md', 'The bounded registry and gate are active', \@errors);
        require_literal('AGENTS.md', 'Published-claims:', \@errors);
        require_literal('README.md', '[Claim verification](CLAIM_VERIFICATION.md)', \@errors);
        require_literal('COMMIT.md', 'Published-claims:', \@errors);
        require_literal('TOOLBOX.md', 'Published-claims:', \@errors);
        require_literal('.github/PULL_REQUEST_TEMPLATE.md', 'Published-claims:', \@errors);
        require_literal('DOCTRINE_ENFORCEMENT.md', '`CLAIM-VERIFICATION`', \@errors);
        require_literal('scripts/check_doctrines.sh', '"CLAIM-VERIFICATION|gate|', \@errors);
    } elsif ($id eq 'workflow-standard-capacity-profile') {
        my $surfaces_raw = read_raw(absolute($root, 'doctrine/live_document_size/surfaces.jsonl'), \@errors,
            'doctrine/live_document_size/surfaces.jsonl');
        my @surface_records = map { JSON::PP->new->decode($_) } grep { $_ ne '' } split /\n/, $surfaces_raw;
        my ($meta) = grep { ($_->{record_type} // '') eq 'registry' } @surface_records;
        my ($workflow) = grep { ($_->{surface_id} // '') eq 'workflow_standards' } @surface_records;
        my ($catalog_surface) = grep { ($_->{surface_id} // '') eq 'canonical_collection_indexes' } @surface_records;
        if (!$meta || !$workflow || !$catalog_surface) {
            push @errors, 'live-size registry omits workflow/catalog feasibility inputs';
        } else {
            my $catalog = read_raw(absolute($root, 'docs/catalogs/workflow-standards.md'), \@errors,
                'docs/catalogs/workflow-standards.md');
            my @lines = ($catalog =~ /.*\n/g);
            my @rows = grep { /^\| \[/ } @lines;
            my $targets = ref($workflow->{targets}) eq 'ARRAY' ? scalar(@{$workflow->{targets}}) : 0;
            push @errors, "workflow catalog rows " . scalar(@rows) . " differ from targets $targets"
                if @rows != $targets;
            my $capacity = $workflow->{enforcement_ceilings}{files} // 0;
            my $scaffold_lines = @lines - @rows;
            my $scaffold_bytes = length($catalog) - length(join('', @rows));
            my $max_row_bytes = 512;
            require_literal('scripts/check_canonical_collection_catalogs.pl', 'my $MAX_ROW_BYTES = 512;', \@errors);
            my $full_lines = $scaffold_lines + $capacity;
            my $full_bytes = $scaffold_bytes + $capacity * ($max_row_bytes + 1);
            my $line_ceiling = $catalog_surface->{enforcement_ceilings}{lines_each} // 0;
            my $byte_ceiling = $catalog_surface->{enforcement_ceilings}{bytes_each} // 0;
            my $target_cap = $meta->{max_array_items} // 0;
            push @errors, workflow_feasibility_errors(
                $capacity, $full_lines, $full_bytes, $line_ceiling, $byte_ceiling, $target_cap,
            );
            my @controlled_red = workflow_feasibility_errors(
                $capacity, $full_lines, $full_bytes, $full_lines - 1, $byte_ceiling, $target_cap,
            );
            push @errors, 'workflow feasibility controlled sub-ceiling mutation did not go RED'
                if !grep { /exceeds controlled catalog ceiling/ } @controlled_red;
            if (!@errors) {
                print "workflow-catalog-feasibility: $capacity members => $full_lines lines / $full_bytes bytes "
                    . "below $line_ceiling / $byte_ceiling and target cap $target_cap; "
                    . "controlled sub-ceiling RED PASS\n";
                return;
            }
        }
    } elsif ($id eq 'claim-provenance-gate-active') {
        require_literal('CLAIM_VERIFICATION.md', 'scripts/check_claim_verification.pl', \@errors);
        require_literal('DOCTRINE_ENFORCEMENT.md', '`CLAIM-VERIFICATION`', \@errors);
        require_literal('scripts/check_doctrines.sh', '"CLAIM-VERIFICATION|gate|', \@errors);
        require_regular('scripts/check_claim_verification.pl', \@errors);
        my $audit_result = validate_registry(
            root => $root, registry_rel => $registry_rel, execute_commands => 0, check_publication => 0,
        );
        push @errors, @{$audit_result->{errors}};
        my $audit = $audit_result->{control_audit};
        if (!@errors) {
            push @errors, 'not every cited falsification control has exact RED evidence'
                if $audit->{cited_controls} != $audit->{exact_red_evidence};
            push @errors, 'governed producer census contains ignored or untracked candidates'
                if $audit->{ignored_candidates} || $audit->{untracked_candidates};
        }
        if (!@errors) {
            print "claim-control-audit: $audit->{cited_controls} controls / "
                . "$audit->{exact_red_evidence} exact RED regions / $audit->{governed_producers} producers / "
                . "$audit->{ignored_candidates} ignored / $audit->{untracked_candidates} untracked PASS\n";
        }
    } else {
        die "claim-verification: unknown probe '$id'\n";
    }
    if (@errors) {
        print STDERR "claim-verification-probe: $_\n" for @errors;
        exit 1;
    }
    print "claim-verification-probe: $id PASS\n";
}

sub workflow_feasibility_errors {
    my ($capacity, $full_lines, $full_bytes, $line_ceiling, $byte_ceiling, $target_cap) = @_;
    my @errors;
    push @errors, "full workflow catalog $full_lines/$full_bytes exceeds controlled catalog ceiling "
        . "$line_ceiling/$byte_ceiling"
        if $full_lines > $line_ceiling || $full_bytes > $byte_ceiling;
    push @errors, "workflow capacity $capacity exceeds target-array cap $target_cap"
        if $capacity > $target_cap;
    return @errors;
}

sub require_literal {
    my ($relative, $literal, $errors) = @_;
    my $path = absolute($root, $relative);
    if (!-f $path || -l $path) {
        push @$errors, "$relative is missing or not a regular file";
        return;
    }
    my $raw = read_raw($path, $errors, $relative);
    push @$errors, "$relative omits required literal '$literal'" if index($raw, $literal) < 0;
}

sub require_regular {
    my ($relative, $errors) = @_;
    my $path = absolute($root, $relative);
    push @$errors, "$relative is missing or not a regular file" if !-f $path || -l $path;
}

sub run_self_test {
    my $generated = absolute($script_root, 'generated');
    make_path($generated) if !-d $generated;
    my $fixture = File::Spec->catdir($generated, ".claim-verification-self-test.$$" );
    die "claim-verification self-test: unsafe fixture path\n"
        if index($fixture, $generated . File::Spec->catfile('')) != 0;
    remove_tree($fixture) if -e $fixture;
    make_path(File::Spec->catdir($fixture, 'doctrine', 'claim_verification'));
    make_path(File::Spec->catdir($fixture, 'scripts'));
    write_raw(File::Spec->catfile($fixture, 'scripts', 'producer.pl'), "#!/usr/bin/env perl\nprint qq{source PASS\\n};\n");
    write_raw(File::Spec->catfile($fixture, 'scripts', 'control.pl'), "#!/usr/bin/env perl\nprint qq{control RED observed\\n};\n");
    write_raw(File::Spec->catfile($fixture, 'input.txt'), "canonical\n");
    write_raw(File::Spec->catfile($fixture, 'evidence.txt'), "retained\n");
    write_raw(File::Spec->catfile($fixture, '.gitignore'), "scripts/ignored-*.pl\n");
    command_ok($fixture, ['git', 'init', '-q']);
    command_ok($fixture, ['git', 'config', 'user.email', 'claim-self-test@example.invalid']);
    command_ok($fixture, ['git', 'config', 'user.name', 'Claim Self Test']);

    my $meta = {
        record_type => 'registry', schema_version => 1, max_records => 8, max_bytes => 32_768,
        max_record_bytes => 16_384, max_array_items => 16, max_scalar_bytes => 1_024,
        allowed_statuses => [qw(verified incomplete superseded)],
    };
    my $claim = fixture_claim($fixture);
    my @base_records = ($meta, $claim);
    write_registry($fixture, \@base_records);
    command_ok($fixture, ['git', 'add', '.']);
    command_ok($fixture, ['git', 'commit', '-qm', 'fixture']);

    my $passed = 0;
    my $total = 0;
    my @cases = (
        ['clean verified registry', 1, qr//, sub {}],
        ['valid incomplete registry', 1, qr//, sub {
            $_[0][1] = {record_type => 'claim', schema_version => 1, claim_id => 'fixture-claim',
                status => 'incomplete', assertion => 'fixture assertion', owner => 'fixture-owner',
                missing_legs => ['falsification']};
        }],
        ['valid superseded registry', 1, qr//, sub {
            push @{$_[0]}, {record_type => 'claim', schema_version => 1, claim_id => 'old-fixture-claim',
                status => 'superseded', assertion => 'old fixture assertion', owner => 'fixture-owner',
                superseded_by => 'fixture-claim'};
        }],
        ['missing leg', 0, qr/rederive must be an object/, sub { delete $_[0][1]{rederive} }],
        ['unknown claim field', 0, qr/unknown field 'surprise'/, sub { $_[0][1]{surprise} = 1 }],
        ['duplicate claim id', 0, qr/duplicates claim_id/, sub { push @{$_[0]}, clone($_[0][1]) }],
        ['stale artifact digest', 0, qr/is stale: SHA-256/, sub { $_[0][1]{durability}{artifacts}[0]{sha256} = '0' x 64 }],
        ['missing artifact identity', 0, qr/is absent from durability.artifacts/, sub { shift @{$_[0][1]{durability}{artifacts}} }],
        ['duplicate artifact', 0, qr/path duplicates/, sub { push @{$_[0][1]{durability}{artifacts}}, clone($_[0][1]{durability}{artifacts}[0]) }],
        ['untracked artifact', 0, qr/is untracked/, sub {
            write_raw(File::Spec->catfile($fixture, 'untracked.txt'), "untracked\n");
            push @{$_[0][1]{durability}{artifacts}}, {path => 'untracked.txt', role => 'canonical_input', sha256 => sha256_hex("untracked\n")};
            push @{$_[0][1]{durability}{stale_check}{inputs}}, 'untracked.txt';
        }],
        ['unsafe path', 0, qr/is unsafe/, sub { $_[0][1]{durability}{artifacts}[0]{path} = '../escape' }],
        ['false RED assertion', 0, qr/observed_red must be JSON true/, sub { $_[0][1]{falsification}{observed_red} = JSON::PP::false }],
        ['missing exact RED evidence', 0, qr/red_evidence must be an object/, sub {
            delete $_[0][1]{falsification}{controls}[0]{red_evidence};
        }],
        ['stale exact RED evidence', 0, qr/source_region is stale/, sub {
            $_[0][1]{falsification}{controls}[0]{red_evidence}{source_region}{sha256} = '0' x 64;
        }],
        ['RED evidence omits diagnostic', 0, qr/omits expected RED diagnostic/, sub {
            $_[0][1]{falsification}{controls}[0]{red_evidence}{expected_red} = 'missing RED diagnostic';
        }],
        ['untracked producer candidate', 0, qr/untracked producer-shaped path/, sub {
            write_raw(File::Spec->catfile($fixture, 'scripts', 'scratch.pl'), "print qq{scratch\\n};\n");
        }],
        ['ignored producer candidate', 0, qr/ignored producer-shaped path/, sub {
            write_raw(File::Spec->catfile($fixture, 'scripts', 'ignored-scratch.pl'), "print qq{scratch\\n};\n");
        }],
        ['unknown status', 0, qr/status 'almost' is unknown/, sub { $_[0][1]{status} = 'almost' }],
        ['stale-check omission', 0, qr/stale_check omits artifact/, sub { pop @{$_[0][1]{durability}{stale_check}{inputs}} }],
        ['unknown supersession target', 0, qr/superseded_by 'missing-claim' is unknown/, sub {
            $_[0][1] = {record_type => 'claim', schema_version => 1, claim_id => 'old-claim', status => 'superseded', assertion => 'old', owner => 'owner', superseded_by => 'missing-claim'};
        }],
        ['portable hard-cap refusal', 0, qr/exceeds portable hard cap/, sub { $_[0][0]{max_records} = 129 }],
    );
    for my $case (@cases) {
        $total++;
        my ($name, $expected_ok, $diagnostic, $mutate) = @$case;
        unlink File::Spec->catfile($fixture, 'untracked.txt') if -e File::Spec->catfile($fixture, 'untracked.txt');
        unlink File::Spec->catfile($fixture, 'scripts', 'scratch.pl')
            if -e File::Spec->catfile($fixture, 'scripts', 'scratch.pl');
        unlink File::Spec->catfile($fixture, 'scripts', 'ignored-scratch.pl')
            if -e File::Spec->catfile($fixture, 'scripts', 'ignored-scratch.pl');
        my $records = clone(\@base_records);
        $mutate->($records);
        write_registry($fixture, $records);
        my $result = validate_registry(root => $fixture, registry_rel => $registry_rel, execute_commands => 0, check_publication => 0);
        my $ok = @{$result->{errors}} ? 0 : 1;
        my $joined = join("\n", @{$result->{errors}});
        die "claim-verification self-test '$name' expected " . ($expected_ok ? 'PASS' : 'RED') . ", got " . ($ok ? 'PASS' : "RED: $joined") . "\n"
            if $ok != $expected_ok;
        die "claim-verification self-test '$name' missed diagnostic $diagnostic: $joined\n"
            if !$expected_ok && $joined !~ $diagnostic;
        $passed++;
    }

    my %fixture_claims = ('fixture-claim' => $claim);
    my @message_cases = (
        ['known publication', "Subject\n\nPublished-claims: fixture-claim\n", 1, qr//],
        ['none publication', "Subject\n\nPublished-claims: none\n", 1, qr//],
        ['missing declaration', "Subject\n", 0, qr/exactly one/],
        ['unknown publication', "Published-claims: unknown-claim\n", 0, qr/references unknown claim id/],
        ['duplicate publication', "Published-claims: fixture-claim, fixture-claim\n", 0, qr/duplicates claim id/],
        ['duplicate declaration', "Published-claims: none\nPublished-claims: none\n", 0, qr/exactly one/],
    );
    for my $case (@message_cases) {
        $total++;
        my ($name, $message, $expected_ok, $diagnostic) = @$case;
        my @errors;
        validate_publication_message($message, \%fixture_claims, \@errors);
        my $ok = @errors ? 0 : 1;
        my $joined = join("\n", @errors);
        die "claim-verification self-test '$name' expected " . ($expected_ok ? 'PASS' : 'RED') . ", got " . ($ok ? 'PASS' : "RED: $joined") . "\n"
            if $ok != $expected_ok;
        die "claim-verification self-test '$name' missed diagnostic $diagnostic: $joined\n"
            if !$expected_ok && $joined !~ $diagnostic;
        $passed++;
    }

    remove_tree($fixture);
    die "claim-verification self-test: residue remains at $fixture\n" if -e $fixture;
    # PRODUCTION-GRAPH-CENSUS-PIN.3 — `$passed/$total` detects a FAILING case but not a
    # DELETED one: `$total` is incremented in the same case loop, so removing a case drops both
    # and the ratio stays N/N (measured: this suite went 19/19 -> 18/18 and exited 0). The
    # expected case count is therefore declared here, independently of the loop.
    my $expected_cases = 27;
    die "claim-verification: self-test ran $total cases, declaration expects $expected_cases — "
        . "re-derive the declaration beside the suite\n"
        if $total != $expected_cases;
    print "claim-verification: self-test $passed/$total positive, missing, unknown, duplicate, stale, untracked, and bound cases pass.\n";
}

sub fixture_claim {
    my ($fixture) = @_;
    my @paths = (
        ['scripts/producer.pl', 'producer_control'],
        ['scripts/control.pl', 'control'],
        ['input.txt', 'canonical_input'],
        ['evidence.txt', 'retained_evidence'],
    );
    my @artifacts = map {
        my ($path, $role) = @$_;
        {path => $path, role => $role, sha256 => sha256_hex(read_raw(absolute($fixture, $path), [], $path))}
    } @paths;
    return {
        record_type => 'claim', schema_version => 1, claim_id => 'fixture-claim', status => 'verified',
        assertion => 'fixture assertion', owner => 'fixture-owner',
        rederive => {boundary => 'fixture boundary', result => 'source passes', commands => [{
            id => 'source', argv => ['perl', 'scripts/producer.pl'], producer => 'scripts/producer.pl',
            inputs => ['input.txt'], expected_exit => 0, stdout_contains => 'source PASS',
        }]},
        falsification => {competing_hypothesis => 'fixture competitor', observed_red => JSON::PP::true,
            result => 'control distinguishes competitor', controls => [{
                id => 'control', argv => ['perl', 'scripts/control.pl'], producer => 'scripts/control.pl',
                inputs => ['input.txt'], expected_exit => 0, stdout_contains => 'control RED observed',
                red_evidence => {case_id => 'fixture-known-bad', perturbation => 'fixture control input',
                    expected_red => 'control RED observed', source_region => {
                        kind => 'line_range_sha256', start_line => 2, end_line => 2,
                        sha256 => sha256_hex("print qq{control RED observed\\n};\n"),
                    }},
            }]},
        durability => {refresh_owner => 'fixture-owner', refresh_rule => 'refresh on digest drift',
            artifacts => \@artifacts,
            stale_check => {id => 'stale', argv => ['perl', 'scripts/producer.pl'], producer => 'scripts/producer.pl',
                inputs => [qw(scripts/control.pl input.txt evidence.txt)], expected_exit => 0,
                stdout_contains => 'source PASS'},
            retained_evidence => ['evidence.txt']},
    };
}

sub write_registry {
    my ($fixture, $records) = @_;
    my $json = JSON::PP->new->canonical(1);
    write_raw(absolute($fixture, $registry_rel), join('', map { $json->encode($_) . "\n" } @$records));
}

sub clone {
    my ($value) = @_;
    my $json = JSON::PP->new->canonical(1);
    return $json->decode($json->encode($value));
}

sub reject_unknown {
    my ($object, $label, $errors, @allowed) = @_;
    return if ref($object) ne 'HASH';
    my %allowed = map { $_ => 1 } @allowed;
    push @$errors, "$label has unknown field '$_'" for grep { !$allowed{$_} } sort keys %$object;
}

sub required_scalar {
    my ($value, $label, $errors) = @_;
    if (!defined($value) || ref($value) || $value eq '' || $value =~ /[\x00-\x1f]/) {
        push @$errors, "$label must be a nonempty control-free scalar";
        return;
    }
    return "$value";
}

sub exact_scalar {
    my ($value, $expected, $label, $errors) = @_;
    push @$errors, "$label must equal '$expected'"
        if !defined($value) || ref($value) || "$value" ne "$expected";
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

sub safe_relative {
    my ($path) = @_;
    return defined($path) && !ref($path) && $path ne '' && $path !~ m{\A/}
        && $path !~ /\\/ && $path !~ m{(?:\A|/)\.\.(?:/|\z)}
        && $path !~ m{(?:\A|/)\.(?:/|\z)} && $path !~ /[\x00-\x1f]/;
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

sub write_raw {
    my ($path, $raw) = @_;
    make_path(dirname($path)) if !-d dirname($path);
    open my $fh, '>:raw', $path or die "claim-verification self-test: cannot write $path: $!\n";
    print {$fh} $raw or die "claim-verification self-test: cannot write bytes to $path: $!\n";
    close $fh or die "claim-verification self-test: cannot close $path: $!\n";
}

sub capture_command {
    my ($base, $argv) = @_;
    my $old = getcwd();
    chdir $base or return ('', "cannot chdir to $base: $!", 127);
    my ($in, $out);
    my $err = gensym;
    my $pid = eval { open3($in, $out, $err, @$argv) };
    if ($@) {
        chdir $old or die "claim-verification: cannot restore cwd: $!\n";
        return ('', "cannot execute '$argv->[0]': $@", 127);
    }
    close $in;
    my $select = IO::Select->new($out, $err);
    my ($stdout, $stderr) = ('', '');
    my %is_stdout = (fileno($out) => 1);
    while (my @ready = $select->can_read) {
        for my $fh (@ready) {
            my $buffer = '';
            my $read = sysread($fh, $buffer, 8192);
            if (!defined($read) || $read == 0) {
                $select->remove($fh);
                close $fh;
            } elsif ($is_stdout{fileno($fh)}) {
                $stdout .= $buffer;
            } else {
                $stderr .= $buffer;
            }
        }
    }
    waitpid($pid, 0);
    my $exit = $? == -1 ? 127 : ($? >> 8);
    chdir $old or die "claim-verification: cannot restore cwd: $!\n";
    return ($stdout, $stderr, $exit);
}

sub command_ok {
    my ($base, $argv) = @_;
    my ($stdout, $stderr, $exit) = capture_command($base, $argv);
    die "claim-verification self-test: '@$argv' failed ($exit): $stdout$stderr\n" if $exit != 0;
}

sub empty_result {
    my (@errors) = @_;
    return {errors => \@errors, claim_count => 0, commands_executed => 0,
        publication_source => 'not-checked', published_claims => [], statuses => {},
        control_audit => {cited_controls => 0, exact_red_evidence => 0, governed_producers => 0,
            ignored_candidates => 0, untracked_candidates => 0}};
}
