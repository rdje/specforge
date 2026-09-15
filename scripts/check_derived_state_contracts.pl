#!/usr/bin/env perl
use strict;
use warnings;

use Cwd qw(abs_path);
use Encode qw(encode_utf8);
use File::Basename qw(dirname);
use File::Spec;
use JSON::PP;

my $root;
my $registry_rel = 'doctrine/live_document_size/derived_state_contracts.jsonl';
my $surfaces_rel = 'doctrine/live_document_size/surfaces.jsonl';
my $report = 0;

while (@ARGV) {
    my $arg = shift @ARGV;
    if ($arg eq '--root') {
        $root = shift @ARGV // usage();
    } elsif ($arg eq '--registry') {
        $registry_rel = shift @ARGV // usage();
    } elsif ($arg eq '--surfaces') {
        $surfaces_rel = shift @ARGV // usage();
    } elsif ($arg eq '--report') {
        $report = 1;
    } else {
        usage();
    }
}

$root //= File::Spec->catdir(dirname(abs_path($0)), '..');
$root = abs_path($root) // die "derived-state: repository root does not exist\n";
my $root_device = (stat($root))[0];

my @errors;
my $json = JSON::PP->new->canonical(1);
my %valid_class = map { $_ => 1 } qw(
  derive_on_read verified_copy authored_intent immutable_evidence
);
my %historical_lifecycle = map { $_ => 1 } qw(
  archive_terminal external_terminal frozen_legacy
);
my %executed_core;
my %class_count;
my %secondary_ownership_count;
my $secondary_copy_count = 0;

my (undef, $surfaces) = read_jsonl_registry(
    absolute($surfaces_rel),
    'surface registry',
    128,
    131_072,
    8_192,
);
my (undef, $contracts) = read_jsonl_registry(
    absolute($registry_rel),
    'derived-state registry',
    64,
    65_536,
    4_096,
);

my %surface_by_id;
for my $surface (@$surfaces) {
    my $id = scalar_value($surface->{surface_id});
    next if !defined $id;
    if ($surface_by_id{$id}) {
        problem("duplicate surface_id '$id' prevents derived-state ownership");
        next;
    }
    $surface_by_id{$id} = $surface;
}

my %contract_seen;
my %field_seen;
my %marker_seen;
for my $contract (@$contracts) {
    validate_contract(
        $contract,
        \%surface_by_id,
        \%contract_seen,
        \%field_seen,
        \%marker_seen,
    );
}

if (@errors) {
    print STDERR "derived-state: $_\n" for @errors;
    print STDERR "derived-state: FAILED with ", scalar(@errors), " violation(s).\n";
    exit 1;
}

if ($report) {
    print $json->encode({
        contracts => scalar(@$contracts),
        classifications => \%class_count,
        executed_core_verifiers => scalar(keys %executed_core),
        secondary_copies => $secondary_copy_count,
        secondary_ownership => \%secondary_ownership_count,
    }), "\n";
} else {
    print "derived-state: ", scalar(@$contracts),
        " explicit field contracts satisfy bounded classification and authority rules.\n";
}
exit 0;

sub usage {
    die "Usage: $0 [--root DIR] [--registry PATH] [--surfaces PATH] [--report]\n";
}

sub absolute {
    my ($relative) = @_;
    return File::Spec->catfile($root, split m{/}, $relative);
}

sub problem {
    my ($message) = @_;
    push @errors, $message;
}

sub scalar_value {
    my ($value) = @_;
    return undef if !defined($value) || ref($value) || $value eq '';
    return $value;
}

sub read_jsonl_registry {
    my ($path, $label, $hard_records, $hard_bytes, $hard_record_bytes) = @_;
    if (!-f $path || -l $path) {
        problem("$label is missing or not a regular file: " . relative_to_root($path));
        return ({}, []);
    }
    my $file_bytes = -s $path;
    problem("$label exceeds portable hard byte cap $hard_bytes") if $file_bytes > $hard_bytes;
    open my $fh, '<:raw', $path or do {
        problem("cannot read $label: $!");
        return ({}, []);
    };
    my @records;
    my @record_lengths;
    my $line_number = 0;
    while (my $line = <$fh>) {
        $line_number++;
        chomp $line;
        $line =~ s/\r\z//;
        if ($line eq '') {
            problem("$label line $line_number is blank");
            next;
        }
        problem("$label line $line_number exceeds portable raw-record cap $hard_record_bytes")
            if length($line) > $hard_record_bytes;
        my $record = eval { decode_json($line) };
        if ($@ || ref($record) ne 'HASH') {
            problem("$label line $line_number is not one JSON object");
            next;
        }
        push @records, $record;
        push @record_lengths, length($line);
    }
    close $fh or problem("cannot close $label: $!");
    if (!@records) {
        problem("$label is empty");
        return ({}, []);
    }

    my $meta = shift @records;
    reject_unknown_fields(
        $meta,
        "$label registry record",
        # LIVE-DOCUMENT-PRESSURE-HEADROOM.22 — the surface registry header also declares the
        # milestone band its own bounds report pressure against. This loader reads the same header,
        # so an allowed field must be allowed in both places or the second reader refuses it, which
        # is the second-enforcer defect `.2a` caught in the task-plane cap.
        qw(record_type schema_version max_records max_bytes max_record_bytes max_array_items max_scalar_bytes milestones),
    );
    problem("$label first record must have record_type=registry")
        if ($meta->{record_type} // '') ne 'registry';
    for my $field (qw(schema_version max_records max_bytes max_record_bytes max_array_items max_scalar_bytes)) {
        problem("$label registry record lacks numeric '$field'")
            if !defined($meta->{$field}) || ref($meta->{$field}) || $meta->{$field} !~ /^\d+$/;
    }
    problem("$label schema_version must be 1")
        if defined($meta->{schema_version}) && $meta->{schema_version} != 1;
    for my $field (qw(max_records max_bytes max_record_bytes max_array_items max_scalar_bytes)) {
        problem("$label registry '$field' must be positive")
            if defined($meta->{$field}) && $meta->{$field} == 0;
    }
    if (defined $meta->{max_records}) {
        problem("$label declares max_records above portable hard cap")
            if $meta->{max_records} > $hard_records;
        problem("$label has more records than its declared max_records")
            if @records > $meta->{max_records};
    }
    if (defined $meta->{max_bytes}) {
        problem("$label declares max_bytes above portable hard cap")
            if $meta->{max_bytes} > $hard_bytes;
        problem("$label exceeds its declared max_bytes") if $file_bytes > $meta->{max_bytes};
    }
    if (defined $meta->{max_record_bytes}) {
        problem("$label declares max_record_bytes above portable hard cap")
            if $meta->{max_record_bytes} > $hard_record_bytes;
        problem("$label contains a record above its declared max_record_bytes")
            if grep { $_ > $meta->{max_record_bytes} } @record_lengths;
    }
    problem("$label declares max_array_items above portable hard cap")
        if defined($meta->{max_array_items}) && $meta->{max_array_items} > 64;
    problem("$label declares max_scalar_bytes above portable hard cap")
        if defined($meta->{max_scalar_bytes}) && $meta->{max_scalar_bytes} > 1_024;
    if (defined($meta->{max_array_items}) && defined($meta->{max_scalar_bytes})) {
        validate_value_bounds(
            $_,
            "$label data record",
            $meta->{max_array_items},
            $meta->{max_scalar_bytes},
        ) for @records;
    }
    return ($meta, \@records);
}

sub validate_value_bounds {
    my ($value, $label, $max_array_items, $max_scalar_bytes) = @_;
    if (ref($value) eq 'HASH') {
        validate_value_bounds($value->{$_}, "$label.$_", $max_array_items, $max_scalar_bytes)
            for sort keys %$value;
    } elsif (ref($value) eq 'ARRAY') {
        problem("$label has more than $max_array_items array items") if @$value > $max_array_items;
        for my $index (0 .. $#$value) {
            validate_value_bounds($value->[$index], "$label\[$index\]", $max_array_items, $max_scalar_bytes);
        }
    } elsif (ref($value)) {
        problem("$label has an unsupported value type");
    } elsif (defined($value) && length(encode_utf8("$value")) > $max_scalar_bytes) {
        problem("$label exceeds the declared scalar byte limit $max_scalar_bytes");
    }
}

sub reject_unknown_fields {
    my ($object, $label, @allowed) = @_;
    return if ref($object) ne 'HASH';
    my %allowed = map { $_ => 1 } @allowed;
    problem("$label has unknown field '$_'") for grep { !$allowed{$_} } sort keys %$object;
}

sub validate_contract {
    my ($contract, $surfaces, $contract_seen, $field_seen, $marker_seen) = @_;
    my @common = qw(
      record_type schema_version contract_id surface_id path field_id
      classification field_marker authority
    );
    my $classification = scalar_value($contract->{classification}) // '<invalid>';
    my @class_fields = $classification eq 'derive_on_read'
        ? qw(accessor forbidden_storage_marker verifier)
        : $classification eq 'verified_copy'
        ? qw(accessor verifier secondary_copies)
        : $classification eq 'immutable_evidence'
        ? qw(capture_boundary)
        : ();
    reject_unknown_fields($contract, "derived-state contract", @common, @class_fields);

    problem("derived-state record must have record_type=contract")
        if ($contract->{record_type} // '') ne 'contract';
    problem("derived-state contract has unsupported schema_version")
        if !defined($contract->{schema_version}) || ref($contract->{schema_version})
        || $contract->{schema_version} !~ /^\d+$/ || $contract->{schema_version} != 1;

    my %value;
    for my $field (qw(contract_id surface_id path field_id classification field_marker authority)) {
        $value{$field} = scalar_value($contract->{$field});
        problem("derived-state contract lacks non-empty scalar '$field'") if !defined $value{$field};
    }
    return if !defined $value{contract_id};
    my $id = $value{contract_id};
    problem("derived-state contract_id '$id' has an invalid identifier shape")
        if $id !~ /\A[a-z][a-z0-9._-]*\z/;
    problem("derived-state contract '$id' is declared more than once") if $contract_seen->{$id}++;

    if (!$valid_class{$classification}) {
        problem("derived-state contract '$id' has invalid classification '$classification'");
        return;
    }
    $class_count{$classification}++;

    my $surface_id = $value{surface_id};
    my $surface = defined($surface_id) ? $surfaces->{$surface_id} : undef;
    if (!defined $surface) {
        problem("derived-state contract '$id' names unknown surface '$surface_id'");
        return;
    }
    my $lifecycle = scalar_value($surface->{lifecycle}) // '';
    if ($historical_lifecycle{$lifecycle}) {
        problem("derived-state contract '$id' must govern a current maintained surface");
        return;
    }

    my $path = $value{path};
    if (!defined($path) || !safe_relative_path($path)) {
        problem("derived-state contract '$id' path is absolute, escaping, or malformed");
        return;
    }
    my $targets = $surface->{targets};
    if (ref($targets) ne 'ARRAY' || !grep { safe_relative_pattern($_) && $path =~ glob_regex($_) } @$targets) {
        problem("derived-state contract '$id' path is outside surface '$surface_id': $path");
        return;
    }
    my $absolute_path = absolute($path);
    if (!-f $absolute_path || -l $absolute_path) {
        problem("derived-state contract '$id' path is missing or not a regular file: $path");
        return;
    }
    my $device = (stat($absolute_path))[0];
    if (!defined($device) || $device != $root_device) {
        problem("derived-state contract '$id' path is off the repository volume: $path");
        return;
    }

    my $field_id = $value{field_id};
    problem("derived-state contract '$id' has invalid field_id '$field_id'")
        if !defined($field_id) || $field_id !~ /\A[a-z][a-z0-9._-]*\z/;
    my $field_key = join("\0", $surface_id, $path, $field_id // '');
    problem("derived-state field is declared more than once: $surface_id $path $field_id")
        if $field_seen->{$field_key}++;
    my $marker_key = join("\0", $path, $value{field_marker} // '');
    problem("derived-state marker is declared more than once: $path $value{field_marker}")
        if $marker_seen->{$marker_key}++;

    open my $fh, '<:raw', $absolute_path or do {
        problem("derived-state contract '$id' cannot read $path: $!");
        return;
    };
    local $/;
    my $content = <$fh> // '';
    close $fh or problem("derived-state contract '$id' cannot close $path: $!");

    my $marker_count = literal_occurrences($content, $value{field_marker});
    problem("derived-state contract '$id' field marker must occur exactly once in $path (found $marker_count)")
        if $marker_count != 1;

    if ($classification eq 'derive_on_read') {
        my $accessor = required_class_scalar($contract, 'accessor', $id);
        my $forbidden = required_class_scalar($contract, 'forbidden_storage_marker', $id);
        my $verifier = required_class_scalar($contract, 'verifier', $id);
        problem("derived-state contract '$id' accessor is absent from $path")
            if defined($accessor) && !literal_occurrences($content, $accessor);
        problem("derived-state contract '$id' still stores forbidden current-state marker in $path")
            if defined($forbidden) && literal_occurrences($content, $forbidden);
        problem("derived-state contract '$id' derive_on_read must use builtin:derive_on_read")
            if defined($verifier) && $verifier ne 'builtin:derive_on_read';
    } elsif ($classification eq 'verified_copy') {
        required_class_scalar($contract, 'accessor', $id);
        validate_secondary_copies(
            $contract->{secondary_copies},
            $id,
            $surfaces,
            $marker_seen,
        ) if exists $contract->{secondary_copies};
        my $verifier = required_class_scalar($contract, 'verifier', $id);
        execute_verifier($id, $verifier) if defined $verifier;
    } elsif ($classification eq 'immutable_evidence') {
        my $boundary = required_class_scalar($contract, 'capture_boundary', $id);
        my $count = defined($boundary) ? literal_occurrences($content, $boundary) : 0;
        problem("derived-state contract '$id' capture boundary must occur exactly once in $path (found $count)")
            if defined($boundary) && $count != 1;
    }
}

sub validate_secondary_copies {
    my ($copies, $contract_id, $surfaces, $marker_seen) = @_;
    if (ref($copies) ne 'ARRAY' || !@$copies) {
        problem("derived-state contract '$contract_id' secondary_copies must be a non-empty array");
        return;
    }

    my %role_seen;
    for my $index (0 .. $#$copies) {
        my $copy = $copies->[$index];
        my $label = "derived-state contract '$contract_id' secondary_copies[$index]";
        if (ref($copy) ne 'HASH') {
            problem("$label must be an object");
            next;
        }
        reject_unknown_fields(
            $copy,
            $label,
            qw(role ownership surface_id path field_marker),
        );

        my %value;
        for my $field (qw(role ownership path field_marker)) {
            $value{$field} = scalar_value($copy->{$field});
            problem("$label lacks non-empty scalar '$field'") if !defined $value{$field};
        }
        my $role = $value{role};
        if (defined $role) {
            problem("$label role '$role' has an invalid identifier shape")
                if $role !~ /\A[a-z][a-z0-9._-]*\z/;
            problem("derived-state contract '$contract_id' secondary role '$role' is declared more than once")
                if $role_seen{$role}++;
        }

        my $ownership = $value{ownership};
        if (defined($ownership) && $ownership ne 'surface' && $ownership ne 'control') {
            problem("$label has invalid ownership '$ownership'");
        }

        my $path = $value{path};
        if (!defined($path) || !safe_relative_path($path)) {
            problem("$label path is absolute, escaping, or malformed");
            next;
        }
        my $absolute_path = absolute($path);
        if (!-f $absolute_path || -l $absolute_path) {
            problem("$label path is missing or not a regular file: $path");
            next;
        }
        my $device = (stat($absolute_path))[0];
        if (!defined($device) || $device != $root_device) {
            problem("$label path is off the repository volume: $path");
            next;
        }

        my $surface_id = scalar_value($copy->{surface_id});
        if (defined($ownership) && $ownership eq 'surface') {
            if (!defined $surface_id) {
                problem("$label surface ownership requires surface_id");
            } else {
                my $surface = $surfaces->{$surface_id};
                if (!defined $surface) {
                    problem("$label names unknown surface '$surface_id'");
                } else {
                    my $lifecycle = scalar_value($surface->{lifecycle}) // '';
                    problem("$label must govern a current maintained surface")
                        if $historical_lifecycle{$lifecycle};
                    my $targets = $surface->{targets};
                    if (ref($targets) ne 'ARRAY'
                        || !grep { safe_relative_pattern($_) && $path =~ glob_regex($_) } @$targets) {
                        problem("$label path is outside surface '$surface_id': $path");
                    }
                }
            }
        } elsif (defined($ownership) && $ownership eq 'control') {
            problem("$label control ownership must not declare surface_id") if defined $surface_id;
            problem("$label Markdown path must use surface ownership: $path") if $path =~ /\.md\z/i;
        }

        open my $fh, '<:raw', $absolute_path or do {
            problem("$label cannot read $path: $!");
            next;
        };
        local $/;
        my $content = <$fh> // '';
        close $fh or problem("$label cannot close $path: $!");

        my $marker = $value{field_marker};
        if (defined $marker) {
            my $marker_key = join("\0", $path, $marker);
            problem("derived-state marker is declared more than once: $path $marker")
                if $marker_seen->{$marker_key}++;
            my $count = literal_occurrences($content, $marker);
            problem("$label field marker must occur exactly once in $path (found $count)")
                if $count != 1;
        }
        $secondary_copy_count++;
        $secondary_ownership_count{$ownership}++ if defined $ownership;
    }
}

sub required_class_scalar {
    my ($contract, $field, $id) = @_;
    my $value = scalar_value($contract->{$field});
    problem("derived-state contract '$id' lacks non-empty scalar '$field'") if !defined $value;
    return $value;
}

sub execute_verifier {
    my ($contract_id, $verifier) = @_;
    if ($verifier !~ /\A(core|adapter):(.+)\z/) {
        problem("derived-state contract '$contract_id' verified_copy must use core: or adapter: execution");
        return;
    }
    my ($kind, $path) = ($1, $2);
    if (!safe_relative_path($path) || $path =~ /[*?]/) {
        problem("derived-state contract '$contract_id' has unsafe verifier path '$path'");
        return;
    }
    return if $kind eq 'core' && $executed_core{$path}++;
    my $absolute_path = absolute($path);
    if (!-f $absolute_path || -l $absolute_path) {
        problem("derived-state contract '$contract_id' verifier is missing: $path");
        return;
    }
    my @command = $path =~ /\.pl\z/ ? ($^X, $absolute_path) : ($absolute_path);
    if ($kind eq 'adapter') {
        push @command,
            '--root', $root,
            '--registry', $registry_rel,
            '--contract', $contract_id;
    } elsif (!-x $absolute_path && $path !~ /\.pl\z/) {
        problem("derived-state contract '$contract_id' core verifier is not executable: $path");
        return;
    }

    my $pid = fork();
    if (!defined $pid) {
        problem("derived-state contract '$contract_id' cannot fork verifier '$path': $!");
        return;
    }
    if ($pid == 0) {
        chdir $root or exit 126;
        open STDIN, '<', File::Spec->devnull() or exit 126;
        exec { $command[0] } @command;
        exit 126;
    }
    waitpid($pid, 0);
    if ($? != 0) {
        my $status = $? & 127 ? 'signal ' . ($? & 127) : 'exit ' . ($? >> 8);
        problem("derived-state contract '$contract_id' verifier failed ($status): $path");
    }
}

sub safe_relative_path {
    my ($path) = @_;
    return 0 if !defined($path) || ref($path) || $path eq '';
    return 0 if $path =~ m{\A/} || $path =~ /\\/ || $path =~ /[\x00-\x1f\x7f]/;
    return 0 if $path =~ m{(?:\A|/)\.\.?(/|\z)} || $path =~ m{//};
    return 1;
}

sub safe_relative_pattern {
    my ($path) = @_;
    return 0 if !defined($path) || ref($path) || $path eq '';
    return 0 if $path =~ m{\A/} || $path =~ /\\/ || $path =~ /[\x00-\x1f\x7f]/;
    return 0 if $path =~ m{(?:\A|/)\.\.?(/|\z)} || $path =~ m{//};
    return 1;
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

sub literal_occurrences {
    my ($content, $literal) = @_;
    return 0 if !defined($literal) || $literal eq '';
    my $count = 0;
    my $offset = 0;
    while (1) {
        my $position = index($content, $literal, $offset);
        last if $position < 0;
        $count++;
        $offset = $position + length($literal);
    }
    return $count;
}

sub relative_to_root {
    my ($path) = @_;
    my $relative = File::Spec->abs2rel($path, $root);
    $relative =~ s{\\}{/}g;
    return $relative;
}
