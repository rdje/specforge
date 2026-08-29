#!/usr/bin/env perl
use strict;
use warnings;

use Cwd qw(abs_path);
use Digest::SHA qw(sha256_hex);
use Encode qw(encode_utf8);
use File::Basename qw(dirname);
use File::Find qw(find);
use File::Spec;
use JSON::PP;

my $root;
my $registry_rel = 'doctrine/live_document_size/surfaces.jsonl';
my $routes_rel = 'doctrine/readme_entrypoint/routed_destinations.tsv';
my $authorities_rel = 'doctrine/live_document_size/ceiling_increase_authorities.jsonl';
my $report = 0;
my $no_history = 0;

while (@ARGV) {
    my $arg = shift @ARGV;
    if ($arg eq '--root') {
        $root = shift @ARGV // usage();
    } elsif ($arg eq '--registry') {
        $registry_rel = shift @ARGV // usage();
    } elsif ($arg eq '--routes') {
        $routes_rel = shift @ARGV // usage();
    } elsif ($arg eq '--authorities') {
        $authorities_rel = shift @ARGV // usage();
    } elsif ($arg eq '--report') {
        $report = 1;
    } elsif ($arg eq '--no-history') {
        $no_history = 1;
    } else {
        usage();
    }
}

$root //= File::Spec->catdir(dirname(abs_path($0)), '..');
$root = abs_path($root) // die "live-document-size: repository root does not exist\n";

my @errors;
my @warnings;
my $json = JSON::PP->new->canonical(1);
my @dimensions = qw(files lines_each bytes_each lines_total bytes_total line_bytes_each);
my %valid_lifecycle = map { $_ => 1 } qw(
  bounded_snapshot rolling_ledger partitioned_canonical generated_projection
  archive_terminal frozen_legacy maintained_reference
);
my %valid_state = map { $_ => 1 } qw(normal warning_debt rollover_debt transition_debt frozen terminal);
my %valid_locator = map { $_ => 1 } qw(file collection);

my ($registry_meta, $surfaces) = read_jsonl_registry(
    absolute($registry_rel),
    'surface registry',
    128,
    131_072,
    8_192,
);
my ($authority_meta, $authorities) = read_jsonl_registry(
    absolute($authorities_rel),
    'ceiling-authority registry',
    128,
    65_536,
    8_192,
);

validate_surface_schema($_) for @$surfaces;
validate_authority_schema($_) for @$authorities;
my %authority_surface_seen;
for my $authority (@$authorities) {
    my $id = $authority->{surface_id} // next;
    problem("duplicate ceiling authority for surface '$id'") if $authority_surface_seen{$id}++;
}

my @paths = markdown_paths();
my %path_seen = map { $_ => 1 } @paths;
my %surface_by_id;
my %matches_by_surface;

for my $surface (@$surfaces) {
    my $id = required_scalar($surface, 'surface_id', 'surface record');
    next if !defined $id;
    problem("surface_id '$id' has an invalid identifier shape")
        if $id !~ /\A[a-z0-9][a-z0-9._-]*\z/;
    if ($surface_by_id{$id}) {
        problem("duplicate surface_id '$id'");
        next;
    }
    $surface_by_id{$id} = $surface;
    my $lifecycle = required_scalar($surface, 'lifecycle', "surface '$id'");
    problem("surface '$id' has unknown lifecycle '$lifecycle'")
        if defined($lifecycle) && !$valid_lifecycle{$lifecycle};
    my $state = required_scalar($surface, 'state', "surface '$id'");
    problem("surface '$id' has unknown state '$state'")
        if defined($state) && !$valid_state{$state};
    required_scalar($surface, 'owner', "surface '$id'");
    my $locator = required_scalar($surface, 'locator', "surface '$id'");
    problem("surface '$id' has unknown locator '$locator'")
        if defined($locator) && !$valid_locator{$locator};

    my $targets = $surface->{targets};
    if (ref($targets) ne 'ARRAY' || !@$targets) {
        problem("surface '$id' must declare a non-empty targets array");
        next;
    }
    my @regexes;
    for my $target (@$targets) {
        if (!defined($target) || ref($target) || $target eq '') {
            problem("surface '$id' has an empty or non-scalar target");
            next;
        }
        if (!safe_relative_pattern($target)) {
            problem("surface '$id' target '$target' is absolute, parent-relative, or off-root");
            next;
        }
        push @regexes, glob_regex($target);
    }
    my @matched = grep {
        my $path = $_;
        scalar grep { $path =~ $_ } @regexes;
    } @paths;
    if (!@matched) {
        problem("surface '$id' matches no Markdown path");
    }
    problem("surface '$id' declares a file locator but matches " . scalar(@matched)
        . " paths; use the collection locator")
        if ($locator // '') eq 'file' && @matched > 1;
    $matches_by_surface{$id} = \@matched;
}

for my $path (@paths) {
    my @owners;
    for my $id (sort keys %matches_by_surface) {
        push @owners, $id if grep { $_ eq $path } @{ $matches_by_surface{$id} };
    }
    problem("tracked Markdown '$path' is not classified") if !@owners;
    problem("tracked Markdown '$path' is classified more than once: " . join(', ', @owners))
        if @owners > 1;
}

my %metrics_by_surface;
my %executed_verifier;
for my $id (sort keys %surface_by_id) {
    my $surface = $surface_by_id{$id};
    my $paths = $matches_by_surface{$id} // [];
    my $metrics = measure_paths($paths);
    $metrics_by_surface{$id} = $metrics;
    validate_limits($surface, $metrics, $id);
    validate_cardinality_exemption($surface, $id, \%surface_by_id, \%matches_by_surface);
    validate_lifecycle($surface, $paths, $metrics, $id, \%executed_verifier);
    if ($report) {
        print $json->encode({ surface_id => $id, metrics => $metrics }), "\n";
    }
}

validate_routes(absolute($routes_rel), \%path_seen, \%surface_by_id, \%matches_by_surface);
validate_ceiling_history($surfaces, $authorities) if !$no_history;

if (@errors) {
    print STDERR "live-document-size: $_\n" for @errors;
    print STDERR "live-document-size: FAILED with ", scalar(@errors), " violation(s).\n";
    exit 1;
}

print STDERR "live-document-size: warning: $_\n" for @warnings;
print "live-document-size: ", scalar(@paths), " Markdown files satisfy ",
    scalar(keys %surface_by_id), " governed surfaces.\n" if !$report;
exit 0;

sub usage {
    die "Usage: $0 [--root DIR] [--registry PATH] [--routes PATH] [--authorities PATH] [--report] [--no-history]\n";
}

sub absolute {
    my ($relative) = @_;
    return File::Spec->catfile($root, split m{/}, $relative);
}

sub problem {
    my ($message) = @_;
    push @errors, $message;
}

sub read_jsonl_registry {
    my ($path, $label, $hard_records, $hard_bytes, $hard_record_bytes) = @_;
    if (!-f $path) {
        problem("$label is missing: " . relative_to_root($path));
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
        next if $line eq '';
        problem("$label line $line_number exceeds portable raw-record cap $hard_record_bytes")
            if length($line) > $hard_record_bytes;
        my $record = eval { decode_json($line) };
        if (!$record || ref($record) ne 'HASH') {
            problem("$label line $line_number is not one JSON object: $@");
            next;
        }
        push @records, $record;
        push @record_lengths, length($line);
    }
    close $fh;
    if (!@records) {
        problem("$label is empty");
        return ({}, []);
    }
    my $meta = shift @records;
    problem("$label first record must have record_type=registry")
        if ($meta->{record_type} // '') ne 'registry';
    reject_unknown_fields(
        $meta,
        "$label registry record",
        qw(record_type schema_version max_records max_bytes max_record_bytes max_array_items max_scalar_bytes),
    );
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
        problem("$label declares max_records above portable hard cap") if $meta->{max_records} > $hard_records;
        problem("$label has more records than its declared max_records") if @records > $meta->{max_records};
    }
    if (defined $meta->{max_bytes}) {
        problem("$label declares max_bytes above portable hard cap") if $meta->{max_bytes} > $hard_bytes;
        problem("$label exceeds its declared max_bytes") if $file_bytes > $meta->{max_bytes};
    }
    if (defined $meta->{max_record_bytes}) {
        problem("$label declares max_record_bytes above portable hard cap")
            if $meta->{max_record_bytes} > $hard_record_bytes;
        for my $length (@record_lengths) {
            problem("$label contains a record above its declared max_record_bytes")
                if $length > $meta->{max_record_bytes};
        }
    }
    if (defined $meta->{max_array_items}) {
        problem("$label declares max_array_items above portable hard cap")
            if $meta->{max_array_items} > 64;
    }
    if (defined $meta->{max_scalar_bytes}) {
        problem("$label declares max_scalar_bytes above portable hard cap")
            if $meta->{max_scalar_bytes} > 1_024;
    }
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

sub validate_surface_schema {
    my ($surface) = @_;
    my $id = defined($surface->{surface_id}) && !ref($surface->{surface_id})
        ? $surface->{surface_id}
        : '<unknown>';
    reject_unknown_fields(
        $surface,
        "surface '$id'",
        qw(surface_id targets locator lifecycle state owner health_targets enforcement_ceilings milestones verifier baseline transition currency index index_contract freshness_verifier canonical_inputs sha256 reference_contract archive_manifest aggregate_composition cardinality_exemption),
    );
    validate_aggregate_reachability($surface, $id);
    reject_unknown_fields($surface->{health_targets}, "surface '$id' health_targets", @dimensions);
    reject_unknown_fields($surface->{enforcement_ceilings}, "surface '$id' enforcement_ceilings", @dimensions);
    reject_unknown_fields($surface->{baseline}, "surface '$id' baseline", @dimensions)
        if exists $surface->{baseline};
    reject_unknown_fields($surface->{milestones}, "surface '$id' milestones", qw(warning_pct rollover_pct));
    if (exists $surface->{transition}) {
        reject_unknown_fields($surface->{transition}, "surface '$id' transition", qw(owner max_growth));
        reject_unknown_fields($surface->{transition}{max_growth}, "surface '$id' transition.max_growth", @dimensions)
            if ref($surface->{transition}) eq 'HASH';
    }
    reject_unknown_fields($surface->{currency}, "surface '$id' currency", qw(status owner verifier))
        if exists $surface->{currency};
    reject_unknown_fields($surface->{index_contract}, "surface '$id' index_contract", qw(kind verifier route_surface))
        if exists $surface->{index_contract};
    reject_unknown_fields($surface->{cardinality_exemption}, "surface '$id' cardinality_exemption",
        qw(authority work_unit route_surface_id rationale))
        if exists $surface->{cardinality_exemption};
    if (exists $surface->{reference_contract}) {
        my $contract = $surface->{reference_contract};
        reject_unknown_fields($contract, "surface '$id' reference_contract", qw(mandatory_read max_navigation_depth aggregate_change));
        if (ref($contract) eq 'HASH') {
            reject_unknown_fields($contract->{mandatory_read}, "surface '$id' mandatory_read", qw(path lines_ceiling bytes_ceiling));
            my $aggregate = $contract->{aggregate_change};
            reject_unknown_fields($aggregate, "surface '$id' aggregate_change", qw(authority_id owner baseline delta rationale));
            if (ref($aggregate) eq 'HASH') {
                reject_unknown_fields($aggregate->{baseline}, "surface '$id' aggregate baseline", qw(files lines_total bytes_total));
                reject_unknown_fields($aggregate->{delta}, "surface '$id' aggregate delta", qw(files lines_total bytes_total));
            }
        }
    }
}

# A collection's aggregate must be at least what its own file and per-file bounds permit, or a corpus whose
# every file is legal is refused by a total no single file can see — a state ordinary compliant writing
# reaches and no compliant action leaves (ADR 0029, ADR 0032). The single exemption is a heterogeneous
# collection that declares its exact member partition, which must sum to the declared bounds rather than
# merely assert that it does. A role may use one scalar count for equal file bands or exact per-band counts
# when health and ceiling intentionally admit different collection cardinalities.
sub validate_aggregate_reachability {
    my ($surface, $id) = @_;
    return if ($surface->{locator} // '') ne 'collection';
    my $composition = $surface->{aggregate_composition};
    my $members = validate_aggregate_composition_schema($composition, $id);
    for my $band (qw(health_targets enforcement_ceilings)) {
        my $limits = $surface->{$band};
        next if ref($limits) ne 'HASH';
        my ($files, $per_line, $per_byte, $total_line, $total_byte) =
            @{$limits}{qw(files lines_each bytes_each lines_total bytes_total)};
        next if grep { !defined($_) || ref($_) || $_ !~ /^\d+$/ }
            ($files, $per_line, $per_byte, $total_line, $total_byte);
        if (!$members) {
            problem("surface '$id' $band lines_total $total_line is below its own legal maximum "
                . "$files x $per_line; raise the total or declare an aggregate_composition")
                if $total_line < $files * $per_line;
            problem("surface '$id' $band bytes_total $total_byte is below its own legal maximum "
                . "$files x $per_byte; raise the total or declare an aggregate_composition")
                if $total_byte < $files * $per_byte;
            next;
        }
        my $band_key = $band eq 'health_targets' ? 'health' : 'ceiling';
        my ($count, $lines, $bytes, $max_line, $max_byte) = (0, 0, 0, 0, 0);
        for my $member (@$members) {
            my $bounds = $member->{$band_key};
            next if ref($bounds) ne 'HASH';
            my $member_count = ref($member->{count}) eq 'HASH'
                ? $member->{count}{$band_key}
                : $member->{count};
            next if !defined $member_count;
            $count += $member_count;
            $lines += $member_count * $bounds->{lines};
            $bytes += $member_count * $bounds->{bytes};
            $max_line = $bounds->{lines} if $bounds->{lines} > $max_line;
            $max_byte = $bounds->{bytes} if $bounds->{bytes} > $max_byte;
        }
        problem("surface '$id' aggregate_composition counts sum to $count, not the $band files bound $files")
            if $count != $files;
        problem("surface '$id' aggregate_composition $band_key lines sum to $lines, not lines_total $total_line")
            if $lines != $total_line;
        problem("surface '$id' aggregate_composition $band_key bytes sum to $bytes, not bytes_total $total_byte")
            if $bytes != $total_byte;
        problem("surface '$id' aggregate_composition largest $band_key member is $max_line lines, not lines_each $per_line")
            if $max_line != $per_line;
        problem("surface '$id' aggregate_composition largest $band_key member is $max_byte bytes, not bytes_each $per_byte")
            if $max_byte != $per_byte;
    }
}

sub validate_aggregate_composition_schema {
    my ($composition, $id) = @_;
    return undef if !defined $composition;
    if (ref($composition) ne 'HASH') {
        problem("surface '$id' aggregate_composition must be an object");
        return undef;
    }
    reject_unknown_fields($composition, "surface '$id' aggregate_composition", qw(rationale members));
    problem("surface '$id' aggregate_composition lacks a nonempty rationale")
        if !defined($composition->{rationale}) || ref($composition->{rationale}) || $composition->{rationale} eq '';
    my $members = $composition->{members};
    if (ref($members) ne 'ARRAY' || @$members < 2) {
        problem("surface '$id' aggregate_composition members must list at least two member roles");
        return undef;
    }
    my $valid = 1;
    my %seen_role;
    for my $member (@$members) {
        if (ref($member) ne 'HASH') {
            problem("surface '$id' aggregate_composition member must be an object");
            $valid = 0;
            next;
        }
        reject_unknown_fields($member, "surface '$id' aggregate_composition member", qw(role count health ceiling));
        my $role = $member->{role};
        if (!defined($role) || ref($role) || $role eq '') {
            problem("surface '$id' aggregate_composition member lacks a role");
            $valid = 0;
        } elsif ($seen_role{$role}++) {
            problem("surface '$id' aggregate_composition repeats member role '$role'");
            $valid = 0;
        }
        $role //= '<unknown>';
        my $count = $member->{count};
        if (ref($count) eq 'HASH') {
            reject_unknown_fields(
                $count,
                "surface '$id' aggregate_composition member '$role' count",
                qw(health ceiling),
            );
            for my $band (qw(health ceiling)) {
                next if defined($count->{$band}) && !ref($count->{$band})
                    && $count->{$band} =~ /^\d+$/ && $count->{$band} >= 1;
                problem("surface '$id' aggregate_composition member '$role' count lacks a positive '$band'");
                $valid = 0;
            }
        } elsif (!defined($count) || ref($count) || $count !~ /^\d+$/ || $count < 1) {
            problem("surface '$id' aggregate_composition member '$role' lacks a positive count");
            $valid = 0;
        }
        for my $band (qw(health ceiling)) {
            my $bounds = $member->{$band};
            if (ref($bounds) ne 'HASH') {
                problem("surface '$id' aggregate_composition member '$role' lacks $band bounds");
                $valid = 0;
                next;
            }
            reject_unknown_fields($bounds, "surface '$id' aggregate_composition member '$role' $band", qw(lines bytes));
            for my $axis (qw(lines bytes)) {
                next if defined($bounds->{$axis}) && !ref($bounds->{$axis})
                    && $bounds->{$axis} =~ /^\d+$/ && $bounds->{$axis} >= 1;
                problem("surface '$id' aggregate_composition member '$role' $band lacks a positive '$axis'");
                $valid = 0;
            }
        }
    }
    return $valid ? $members : undef;
}

sub validate_authority_schema {
    my ($authority) = @_;
    my $id = defined($authority->{surface_id}) && !ref($authority->{surface_id})
        ? $authority->{surface_id}
        : '<unknown>';
    reject_unknown_fields($authority, "ceiling authority '$id'", qw(record_type surface_id work_unit owner rationale old new));
    problem("ceiling authority '$id' must have record_type=increase")
        if ($authority->{record_type} // '') ne 'increase';
    required_scalar($authority, $_, "ceiling authority '$id'") for qw(surface_id work_unit owner rationale);
    reject_unknown_fields($authority->{old}, "ceiling authority '$id' old", @dimensions);
    reject_unknown_fields($authority->{new}, "ceiling authority '$id' new", @dimensions);
    numeric_dimensions($authority->{old}, "ceiling authority '$id' old", 0);
    # `new` may carry the null a cardinality exemption introduces; validate_ceiling_history still requires it
    # to equal the surface's new ceilings exactly, so this cannot authorise a null the registry does not hold.
    numeric_dimensions($authority->{new}, "ceiling authority '$id' new", 1);
}

sub markdown_paths {
    my @result;
    my $git_top = git_top();
    if ($git_top eq $root) {
        open my $fh, '-|', 'git', '-C', $root, 'ls-files', '-z', '--cached', '--', '*.md'
            or die "live-document-size: cannot enumerate Git Markdown paths\n";
        local $/ = "\0";
        while (my $path = <$fh>) {
            chomp $path;
            push @result, $path if $path ne '';
        }
        close $fh;
    } else {
        find(
            {
                wanted => sub {
                    return if -d $_;
                    return if $_ !~ /\.md\z/;
                    my $absolute = $File::Find::name;
                    my $relative = File::Spec->abs2rel($absolute, $root);
                    return if $relative =~ m{^(?:\.git|generated)(?:/|\z)};
                    $relative =~ s{\\}{/}g;
                    push @result, $relative;
                },
                no_chdir => 1,
            },
            $root,
        );
    }
    my %unique;
    return sort grep { !$unique{$_}++ } @result;
}

sub safe_relative_pattern {
    my ($path) = @_;
    return 0 if $path =~ m{\A/} || $path =~ m{\\};
    return 0 if $path =~ m{(?:\A|/)\.\.(?:/|\z)};
    return 0 if $path =~ /[\x00-\x1f]/;
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

sub required_scalar {
    my ($record, $field, $label) = @_;
    if (!defined($record->{$field}) || ref($record->{$field}) || $record->{$field} eq '') {
        problem("$label lacks non-empty scalar '$field'");
        return undef;
    }
    return $record->{$field};
}

sub measure_paths {
    my ($paths) = @_;
    my %metrics = map { $_ => 0 } @dimensions;
    $metrics{files} = scalar @$paths;
    for my $path (@$paths) {
        my $absolute = absolute($path);
        if (!-f $absolute) {
            problem("classified Markdown '$path' is missing from the resulting tree");
            next;
        }
        open my $fh, '<:raw', $absolute or do {
            problem("cannot read classified Markdown '$path': $!");
            next;
        };
        local $/;
        my $content = <$fh> // '';
        close $fh;
        my $bytes = length($content);
        my $lines = ($content =~ tr/\n/\n/);
        my $max_line = 0;
        for my $line (split /\n/, $content, -1) {
            $line =~ s/\r\z//;
            $max_line = length($line) if length($line) > $max_line;
        }
        $metrics{bytes_total} += $bytes;
        $metrics{lines_total} += $lines;
        $metrics{bytes_each} = $bytes if $bytes > $metrics{bytes_each};
        $metrics{lines_each} = $lines if $lines > $metrics{lines_each};
        $metrics{line_bytes_each} = $max_line if $max_line > $metrics{line_bytes_each};
    }
    return \%metrics;
}

sub numeric_dimensions {
    my ($object, $label, $allow_null) = @_;
    if (ref($object) ne 'HASH') {
        problem("$label must be an object");
        return;
    }
    for my $dimension (@dimensions) {
        if (!exists $object->{$dimension}) {
            problem("$label lacks '$dimension'");
        } elsif (!defined $object->{$dimension}) {
            problem("$label may not set '$dimension' to null") if !$allow_null;
        } elsif (ref($object->{$dimension}) || $object->{$dimension} !~ /^\d+$/) {
            problem("$label '$dimension' must be a non-negative integer or allowed null");
        }
    }
}

# A count bound a surface can reach with no remedy compliant work can take is the LIVE-DOC-STOP-RISK
# condition. Removing one is legitimate, but a bare `files: null` would let any surface opt out of every
# cardinality control by editing one field. This makes the removal a DECLARED exemption with four conditions a
# checker enforces: the count is null in both bands together, every resource dimension stays numeric, and a
# bounded reader-facing route on a DIFFERENT registered surface covers this surface's declared index
# (LIVE-DOCUMENT-PRESSURE-HEADROOM.2a, ADR 0045).
sub validate_cardinality_exemption {
    my ($surface, $id, $surface_by_id, $matches_by_surface) = @_;
    my $exemption = $surface->{cardinality_exemption};
    my $health = $surface->{health_targets};
    my $ceilings = $surface->{enforcement_ceilings};
    return if ref($health) ne 'HASH' || ref($ceilings) ne 'HASH';
    my $health_null = exists $health->{files} && !defined $health->{files};
    my $ceiling_null = exists $ceilings->{files} && !defined $ceilings->{files};
    my $lifecycle_allows = ($surface->{lifecycle} // '') eq 'maintained_reference';

    if (!defined $exemption) {
        problem("surface '$id' nulls files without a declared cardinality exemption")
            if ($health_null || $ceiling_null) && !$lifecycle_allows;
        return;
    }
    if (ref($exemption) ne 'HASH') {
        problem("surface '$id' cardinality_exemption must be an object");
        return;
    }
    problem("surface '$id' cardinality exemption must null files in both bands together")
        if $health_null != $ceiling_null;
    problem("surface '$id' declares a cardinality exemption while files stays bounded")
        if !$health_null && !$ceiling_null;
    for my $dimension (grep { $_ ne 'files' } @dimensions) {
        problem("surface '$id' cardinality exemption may not unbound resource dimension '$dimension'")
            if !defined $health->{$dimension} || !defined $ceilings->{$dimension};
    }
    for my $field (qw(authority work_unit route_surface_id rationale)) {
        problem("surface '$id' cardinality exemption lacks '$field'")
            if !defined($exemption->{$field}) || ref($exemption->{$field}) || $exemption->{$field} eq '';
    }
    my $authority = $exemption->{authority};
    if (defined($authority) && !ref($authority)) {
        problem("surface '$id' cardinality exemption authority '$authority' is not a tracked repository file")
            if !safe_relative_pattern($authority) || !-f absolute($authority);
    }
    my $route_id = $exemption->{route_surface_id};
    return if !defined($route_id) || ref($route_id) || $route_id eq '';
    if ($route_id eq $id) {
        problem("surface '$id' cardinality exemption route must be a different registered surface");
        return;
    }
    my $route = $surface_by_id->{$route_id};
    if (ref($route) ne 'HASH') {
        problem("surface '$id' cardinality exemption route surface '$route_id' is not registered");
        return;
    }
    my $index = $surface->{index};
    if (!defined($index) || ref($index) || $index eq '') {
        problem("surface '$id' cardinality exemption requires a declared index for its route to cover");
    } else {
        my $covered = grep { $_ eq $index } @{ $matches_by_surface->{$route_id} // [] };
        problem("surface '$id' cardinality exemption route '$route_id' does not cover its declared index")
            if !$covered;
    }
    my $route_ceilings = $route->{enforcement_ceilings};
    if (ref($route_ceilings) ne 'HASH') {
        problem("surface '$id' cardinality exemption route '$route_id' declares no enforcement ceilings");
        return;
    }
    for my $dimension (@dimensions) {
        problem("surface '$id' cardinality exemption route '$route_id' is unbounded in '$dimension'")
            if !defined $route_ceilings->{$dimension};
    }
}

sub validate_limits {
    my ($surface, $metrics, $id) = @_;
    # A maintained reference may null a dimension by lifecycle. Any other surface may null one only behind a
    # declared cardinality exemption, whose conditions validate_cardinality_exemption enforces separately —
    # so a bare null is still refused and cannot be adopted by copying this line (ADR 0045).
    my $allow_null = ($surface->{lifecycle} // '') eq 'maintained_reference'
        || ref($surface->{cardinality_exemption}) eq 'HASH';
    numeric_dimensions($surface->{health_targets}, "surface '$id' health_targets", $allow_null);
    numeric_dimensions($surface->{enforcement_ceilings}, "surface '$id' enforcement_ceilings", $allow_null);
    my $ceilings = $surface->{enforcement_ceilings};
    return if ref($ceilings) ne 'HASH';
    for my $dimension (@dimensions) {
        next if !defined $ceilings->{$dimension};
        problem("surface '$id' exceeds $dimension ceiling: $metrics->{$dimension} > $ceilings->{$dimension}")
            if $metrics->{$dimension} > $ceilings->{$dimension};
    }
    my $targets = $surface->{health_targets};
    my $milestones = $surface->{milestones};
    if (ref($targets) eq 'HASH' && ref($milestones) eq 'HASH') {
        my $warning = $milestones->{warning_pct};
        my $rollover = $milestones->{rollover_pct};
        if (!defined($warning) || !defined($rollover) || $warning !~ /^\d+$/ || $rollover !~ /^\d+$/ || $warning >= $rollover || $rollover >= 100) {
            problem("surface '$id' has invalid warning/rollover milestones");
        } else {
            for my $dimension (@dimensions) {
                next if !defined $targets->{$dimension} || $targets->{$dimension} == 0;
                my $percent = 100 * $metrics->{$dimension} / $targets->{$dimension};
                next if $dimension eq 'files' && $metrics->{$dimension} == $targets->{$dimension};
                # A one-file surface's aggregate merely repeats its per-file measure, so warning on both is
                # noise. Decide that from the measured file count, never from the declared locator: a
                # multi-file surface that called itself a file was silent up to its hard ceiling (ADR 0028).
                next if $metrics->{files} == 1
                    && ($dimension eq 'lines_total' || $dimension eq 'bytes_total');
                next if ($surface->{lifecycle} // '') eq 'frozen_legacy'
                    || ($surface->{lifecycle} // '') eq 'archive_terminal';
                next if ($surface->{state} // '') eq 'transition_debt';
                # Report the distance to the hard boundary next to the percentage. A surface past its health
                # target reports a percentage of a number it already blew, which is the least urgent fact
                # about it: ROADMAP.md read "142.2% of health" while it was 20 lines from a stop that would
                # have landed on an unrelated slice (ADR 0032).
                my $headroom = '';
                my $ceiling = ref($ceilings) eq 'HASH' ? $ceilings->{$dimension} : undef;
                $headroom = sprintf(' — %d below its %d ceiling', $ceiling - $metrics->{$dimension}, $ceiling)
                    if defined($ceiling) && $ceiling >= $metrics->{$dimension};
                if ($percent >= $rollover) {
                    push @warnings, sprintf("surface '%s' %s is at or above rollover (%.1f%%)%s", $id, $dimension, $percent, $headroom);
                } elsif ($percent >= $warning) {
                    push @warnings, sprintf("surface '%s' %s is at or above warning (%.1f%%)%s", $id, $dimension, $percent, $headroom);
                }
            }
        }
    } else {
        problem("surface '$id' must declare milestones");
    }
    if (($surface->{state} // '') eq 'transition_debt') {
        numeric_dimensions($surface->{baseline}, "surface '$id' baseline", 0);
        my $transition = $surface->{transition};
        if (ref($transition) ne 'HASH' || !defined($transition->{owner}) || ref($transition->{owner}) || $transition->{owner} eq '') {
            problem("surface '$id' transition debt lacks an owner");
        } else {
            numeric_dimensions($transition->{max_growth}, "surface '$id' transition.max_growth", 0);
            if (ref($surface->{baseline}) eq 'HASH' && ref($transition->{max_growth}) eq 'HASH') {
                for my $dimension (@dimensions) {
                    next if !defined($surface->{baseline}{$dimension}) || !defined($transition->{max_growth}{$dimension});
                    my $allowed = $surface->{baseline}{$dimension} + $transition->{max_growth}{$dimension};
                    problem("surface '$id' exceeds immutable baseline plus transition allowance for $dimension: $metrics->{$dimension} > $allowed")
                        if $metrics->{$dimension} > $allowed;
                }
            }
        }
    }
}

sub validate_lifecycle {
    my ($surface, $paths, $metrics, $id, $executed) = @_;
    my $lifecycle = $surface->{lifecycle} // '';
    required_scalar($surface, 'verifier', "surface '$id'");
    if ($lifecycle eq 'bounded_snapshot') {
        problem("surface '$id' bounded_snapshot must contain exactly one file") if @$paths != 1;
    } elsif ($lifecycle eq 'partitioned_canonical') {
        validate_index($surface, $paths, $id);
    } elsif ($lifecycle eq 'generated_projection') {
        if (($surface->{locator} // '') eq 'file') {
            problem("surface '$id' generated_projection file must contain exactly one file")
                if @$paths != 1;
        } else {
            validate_index($surface, $paths, $id);
        }
        my $inputs = $surface->{canonical_inputs};
        if (ref($inputs) ne 'ARRAY' || !@$inputs) {
            problem("surface '$id' generated_projection must name canonical_inputs");
        } else {
            for my $input (@$inputs) {
                problem("surface '$id' has an invalid canonical input")
                    if !defined($input) || ref($input) || $input eq '' || !safe_relative_pattern($input);
            }
        }
        my $verifier = required_scalar($surface, 'freshness_verifier', "surface '$id'");
        if (defined $verifier) {
            if (!safe_relative_pattern($verifier) || $verifier =~ /[*?]/) {
                problem("surface '$id' freshness verifier must be one repository-relative executable path");
            } else {
                my $script = absolute($verifier);
                if (!-x $script) {
                    problem("surface '$id' freshness verifier '$verifier' is missing or non-executable");
                } elsif (!$executed->{$verifier}++) {
                    my $status = system { $script } $script;
                    problem("surface '$id' freshness verifier '$verifier' failed") if $status != 0;
                }
            }
        }
    } elsif ($lifecycle eq 'frozen_legacy') {
        problem("surface '$id' frozen_legacy must contain exactly one file") if @$paths != 1;
        my $expected = required_scalar($surface, 'sha256', "surface '$id'");
        if (defined($expected) && @$paths == 1) {
            open my $fh, '<:raw', absolute($paths->[0]) or do {
                problem("cannot hash frozen surface '$id'");
                return;
            };
            local $/;
            my $actual = sha256_hex(<$fh> // '');
            close $fh;
            problem("surface '$id' frozen hash changed") if $actual ne $expected;
        }
    } elsif ($lifecycle eq 'maintained_reference') {
        for my $dimension (qw(lines_each bytes_each line_bytes_each)) {
            problem("surface '$id' maintained_reference requires a numeric $dimension ceiling")
                if !defined $surface->{enforcement_ceilings}{$dimension};
        }
        for my $dimension (qw(files lines_total bytes_total)) {
            problem("surface '$id' maintained_reference must delegate $dimension to exact aggregate authority")
                if defined $surface->{enforcement_ceilings}{$dimension};
        }
        validate_index($surface, $paths, $id);
        my $contract = $surface->{reference_contract};
        if (ref($contract) ne 'HASH') {
            problem("surface '$id' maintained_reference lacks reference_contract");
            return;
        }
        my $mandatory = $contract->{mandatory_read};
        if (ref($mandatory) ne 'HASH') {
            problem("surface '$id' maintained_reference lacks mandatory_read contract");
        } else {
            my $path = $mandatory->{path} // '';
            my $file_metrics = measure_paths([$path]);
            problem("surface '$id' mandatory_read path is outside the surface")
                if !grep { $_ eq $path } @$paths;
            problem("surface '$id' mandatory_read exceeds line ceiling")
                if $file_metrics->{lines_total} > ($mandatory->{lines_ceiling} // -1);
            problem("surface '$id' mandatory_read exceeds byte ceiling")
                if $file_metrics->{bytes_total} > ($mandatory->{bytes_ceiling} // -1);
        }
        problem("surface '$id' max_navigation_depth must be 1")
            if ($contract->{max_navigation_depth} // 0) != 1;
        my $authority = $contract->{aggregate_change};
        if (ref($authority) ne 'HASH') {
            problem("surface '$id' lacks aggregate_change authority");
        } else {
            required_scalar($authority, 'authority_id', "surface '$id' aggregate authority");
            required_scalar($authority, 'owner', "surface '$id' aggregate authority");
            required_scalar($authority, 'rationale', "surface '$id' aggregate authority");
            for my $dimension (qw(files lines_total bytes_total)) {
                my $baseline = $authority->{baseline}{$dimension};
                my $delta = $authority->{delta}{$dimension};
                if (!defined($baseline) || $baseline !~ /^\d+$/ || !defined($delta) || $delta !~ /^-?\d+$/) {
                    problem("surface '$id' aggregate authority has invalid '$dimension' baseline/delta");
                    next;
                }
                my $expected = $baseline + $delta;
                problem("surface '$id' aggregate authority is stale for $dimension: actual $metrics->{$dimension}, expected $expected")
                    if $metrics->{$dimension} != $expected;
            }
        }
    } elsif ($lifecycle eq 'rolling_ledger') {
        problem("surface '$id' rolling_ledger must contain exactly one file") if @$paths != 1;
        problem("surface '$id' rolling ledger must remain transition_debt until it has a shard/archive manifest")
            if ($surface->{state} // '') ne 'transition_debt' && !defined($surface->{archive_manifest});
    } elsif ($lifecycle eq 'archive_terminal') {
        problem("surface '$id' archive terminal must have state=terminal")
            if ($surface->{state} // '') ne 'terminal';
        my $manifest = required_scalar($surface, 'archive_manifest', "surface '$id'");
        if (defined($manifest)) {
            problem("surface '$id' archive manifest must be one repository-relative file")
                if !safe_relative_pattern($manifest) || $manifest =~ /[*?]/;
            problem("surface '$id' archive manifest '$manifest' is missing")
                if safe_relative_pattern($manifest) && $manifest !~ /[*?]/ && !-f absolute($manifest);
        }
    }
    validate_currency($surface, $id, $executed) if exists $surface->{currency};
}

sub validate_currency {
    my ($surface, $id, $executed) = @_;
    my $currency = $surface->{currency};
    if (ref($currency) ne 'HASH') {
        problem("surface '$id' currency contract must be an object");
        return;
    }
    my $status = $currency->{status} // '';
    my $verifier = $currency->{verifier} // '';
    if ($status eq 'transition_debt') {
        problem("surface '$id' currency debt lacks an owner")
            if !defined($currency->{owner}) || ref($currency->{owner}) || $currency->{owner} eq '';
        problem("surface '$id' currency debt verifier must use debt: identifier")
            if $verifier !~ /^debt:[a-z0-9][a-z0-9._-]*$/;
    } elsif ($status eq 'enforced') {
        if (!safe_relative_pattern($verifier) || $verifier =~ /[*?]/ || !-x absolute($verifier)) {
            problem("surface '$id' currency verifier '$verifier' is missing or non-executable");
        } elsif (!$executed->{$verifier}++) {
            my $script = absolute($verifier);
            my $exit = system { $script } $script;
            problem("surface '$id' currency verifier '$verifier' failed") if $exit != 0;
        }
    } else {
        problem("surface '$id' currency status must be transition_debt or enforced");
    }
}

sub validate_index {
    my ($surface, $paths, $id) = @_;
    my $index = required_scalar($surface, 'index', "surface '$id'");
    my $contract = $surface->{index_contract};
    if (ref($contract) ne 'HASH') {
        problem("surface '$id' lacks index_contract");
        return;
    }
    my $kind = $contract->{kind} // '';
    my $verifier = required_scalar($contract, 'verifier', "surface '$id' index_contract");
    problem("surface '$id' index_contract route_surface is only valid for routed_membership")
        if exists($contract->{route_surface}) && $kind ne 'routed_membership';
    if ($kind eq 'query') {
        problem("surface '$id' query index must be git:query") if ($index // '') ne 'git:query';
        problem("surface '$id' query index verifier must be builtin:registry_targets")
            if defined($verifier) && $verifier ne 'builtin:registry_targets';
        return;
    }
    if ($kind ne 'membership' && $kind ne 'external_membership' && $kind ne 'routed_membership') {
        problem("surface '$id' has unknown index kind '$kind'");
        return;
    }
    problem("surface '$id' $kind index verifier must be builtin:markdown_links")
        if defined($verifier) && $verifier ne 'builtin:markdown_links';
    return if !defined $index;
    if (!safe_relative_pattern($index) || $index =~ /[*?]/ || $index !~ /\.md\z/) {
        problem("surface '$id' $kind index '$index' must be one safe repository-relative Markdown path");
        return;
    }
    my $inside = grep { $_ eq $index } @$paths;
    if ($kind eq 'external_membership') {
        problem("surface '$id' external_membership index '$index' must be outside the surface") if $inside;
        problem("surface '$id' external_membership index '$index' is not a classified Markdown surface")
            if !$path_seen{$index};
    } else {
        problem("surface '$id' $kind index '$index' is outside the surface") if !$inside;
    }
    if (!-f absolute($index)) {
        problem("surface '$id' membership index '$index' is missing");
        return;
    }
    my $linked = markdown_link_targets($index);
    if (!defined $linked) {
        problem("surface '$id' cannot read membership index '$index'");
        return;
    }
    if ($kind eq 'routed_membership') {
        return if !expand_route_hop($contract, $paths, $id, $index, $linked);
    }
    for my $path (@$paths) {
        next if $path eq $index;
        problem("surface '$id' index '$index' does not link member '$path'") if !$linked->{$path};
    }
}

# One declared hop, never a chain: the routed index must link every file of the named route
# surface, and those route files' links join the index's own before member completeness is proven.
sub expand_route_hop {
    my ($contract, $paths, $id, $index, $linked) = @_;
    my $route_id = $contract->{route_surface};
    if (!defined($route_id) || ref($route_id) || $route_id eq '') {
        problem("surface '$id' routed_membership lacks a route_surface");
        return 0;
    }
    if ($route_id eq $id) {
        problem("surface '$id' routed_membership route_surface must name another surface");
        return 0;
    }
    if (!$surface_by_id{$route_id}) {
        problem("surface '$id' routed_membership route_surface '$route_id' is not a registered surface");
        return 0;
    }
    my $route_contract = $surface_by_id{$route_id}{index_contract};
    if (ref($route_contract) eq 'HASH' && ($route_contract->{kind} // '') eq 'routed_membership') {
        problem("surface '$id' routed_membership route_surface '$route_id' must not itself route");
        return 0;
    }
    my $route_paths = $matches_by_surface{$route_id} // [];
    if (!@$route_paths) {
        problem("surface '$id' routed_membership route surface '$route_id' matches no Markdown path");
        return 0;
    }
    for my $route_path (@$route_paths) {
        if (!$linked->{$route_path}) {
            problem("surface '$id' index '$index' does not link route member '$route_path'");
            next;
        }
        my $hop = markdown_link_targets($route_path);
        if (!defined $hop) {
            problem("surface '$id' cannot read route member '$route_path'");
            next;
        }
        $linked->{$_} = 1 for keys %$hop;
    }
    return 1;
}

sub markdown_link_targets {
    my ($relative) = @_;
    my $absolute = absolute($relative);
    open my $fh, '<:raw', $absolute or return;
    local $/;
    my $content = <$fh> // '';
    close $fh;
    my %linked;
    while ($content =~ /\]\(([^)]+)\)/g) {
        my $href = $1;
        $href =~ s/#.*\z//;
        next if $href eq '' || $href =~ m{^[a-z]+://}i;
        my $candidate = abs_path(File::Spec->catfile(dirname($absolute), split m{/}, $href));
        next if !defined $candidate || index($candidate, "$root/") != 0;
        $linked{relative_to_root($candidate)} = 1;
    }
    return \%linked;
}

sub validate_routes {
    my ($path, $path_seen, $surfaces, $matches) = @_;
    if (!-f $path) {
        problem('README route registry is missing');
        return;
    }
    open my $fh, '<:raw', $path or do {
        problem("cannot read README route registry: $!");
        return;
    };
    my $header = <$fh> // '';
    chomp $header;
    problem('README route registry schema is unknown')
        if $header ne "target\troute_kind\towner\tlifecycle\tpressure_control\tterminal";
    while (my $line = <$fh>) {
        chomp $line;
        my ($target) = split /\t/, $line, 2;
        next if !defined($target) || $target eq '' || $target =~ m{^(?:https://|git:)};
        if ($target =~ /\.md\z/) {
            problem("README route '$target' is not a classified Markdown surface") if !$path_seen->{$target};
        } elsif ($target =~ m{/\z}) {
            my $found = grep { index($_, $target) == 0 } keys %$path_seen;
            problem("README route collection '$target' contains no classified Markdown") if !$found;
        }
    }
    close $fh;
}

sub validate_ceiling_history {
    my ($current_surfaces, $authorities) = @_;
    my $git_top = git_top();
    return if $git_top ne $root;
    open my $tree_fh, '-|', 'git', '-C', $root, 'ls-tree', '-r', '--name-only', 'HEAD', '--', $registry_rel
        or return;
    my $tracked_registry = <$tree_fh> // '';
    close $tree_fh;
    return if $tracked_registry eq '';
    open my $fh, '-|', 'git', '-C', $root, 'show', "HEAD:$registry_rel" or return;
    local $/;
    my $previous_text = <$fh> // '';
    close $fh;
    return if $previous_text eq '';
    my @previous_records;
    for my $line (split /\n/, $previous_text) {
        next if $line eq '';
        my $record = eval { decode_json($line) };
        next if !$record || ref($record) ne 'HASH' || ($record->{record_type} // '') eq 'registry';
        push @previous_records, $record;
    }
    my %previous = map { ($_->{surface_id} // '') => $_ } @previous_records;
    my %authority_for = map { ($_->{surface_id} // '') => $_ } grep { ($_->{record_type} // '') eq 'increase' } @$authorities;
    my %used_authority;
    for my $current (@$current_surfaces) {
        my $id = $current->{surface_id} // next;
        my $old = $previous{$id} // next;
        if (($old->{state} // '') eq 'transition_debt' && ($current->{state} // '') eq 'transition_debt') {
            my $old_baseline = $json->encode($old->{baseline} // {});
            my $new_baseline = $json->encode($current->{baseline} // {});
            problem("surface '$id' moved its immutable transition baseline") if $old_baseline ne $new_baseline;
        }
        if (($old->{lifecycle} // '') eq 'maintained_reference'
            && ($current->{lifecycle} // '') eq 'maintained_reference') {
            my $old_change = $old->{reference_contract}{aggregate_change} // {};
            my $new_change = $current->{reference_contract}{aggregate_change} // {};
            my $aggregate_changed = 0;
            for my $dimension (qw(files lines_total bytes_total)) {
                my $old_expected = ($old_change->{baseline}{$dimension} // 0) + ($old_change->{delta}{$dimension} // 0);
                my $new_expected = ($new_change->{baseline}{$dimension} // 0) + ($new_change->{delta}{$dimension} // 0);
                $aggregate_changed = 1 if $old_expected != $new_expected;
            }
            if ($aggregate_changed
                && ($old_change->{authority_id} // '') eq ($new_change->{authority_id} // '')) {
                problem("surface '$id' reused maintained-reference authority across aggregate change");
            }
        }
        my @increased;
        for my $dimension (@dimensions) {
            my $before = $old->{enforcement_ceilings}{$dimension};
            my $after = $current->{enforcement_ceilings}{$dimension};
            push @increased, $dimension
                if defined($before) && (!defined($after) || $after > $before);
        }
        next if !@increased;
        my $authority = $authority_for{$id};
        $used_authority{$id} = 1 if $authority;
        if (!$authority
            || $json->encode($authority->{old} // {}) ne $json->encode($old->{enforcement_ceilings} // {})
            || $json->encode($authority->{new} // {}) ne $json->encode($current->{enforcement_ceilings} // {})
            || !defined($authority->{work_unit}) || $authority->{work_unit} eq ''
            || !defined($authority->{owner}) || $authority->{owner} eq ''
            || !defined($authority->{rationale}) || $authority->{rationale} eq '') {
            problem("surface '$id' increased ceiling dimensions without exact authority: " . join(', ', @increased));
        }
    }
    for my $authority (@$authorities) {
        my $id = $authority->{surface_id} // next;
        problem("surface '$id' has unused or banked ceiling-increase authority")
            if !$used_authority{$id};
    }
}

sub git_top {
    return '' if !-e File::Spec->catfile($root, '.git');
    open my $fh, '-|', 'git', '-C', $root, 'rev-parse', '--show-toplevel' or return '';
    my $top = <$fh> // '';
    close $fh;
    chomp $top;
    return $top;
}

sub relative_to_root {
    my ($path) = @_;
    my $relative = File::Spec->abs2rel($path, $root);
    $relative =~ s{\\}{/}g;
    return $relative;
}
