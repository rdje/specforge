#!/usr/bin/env perl
use strict;
use warnings;

use Cwd qw(abs_path);
use File::Basename qw(dirname);
use File::Spec;
use JSON::PP;

my $root;
my $registry_rel = 'doctrine/live_document_size/derived_state_contracts.jsonl';
my $contract_id;

while (@ARGV) {
    my $arg = shift @ARGV;
    if ($arg eq '--root') {
        $root = shift @ARGV // usage();
    } elsif ($arg eq '--registry') {
        $registry_rel = shift @ARGV // usage();
    } elsif ($arg eq '--contract') {
        $contract_id = shift @ARGV // usage();
    } else {
        usage();
    }
}

$root //= File::Spec->catdir(dirname(abs_path($0)), '..');
$root = abs_path($root) // die "derived-state-authority: repository root does not exist\n";
defined($contract_id) && $contract_id =~ /\A[a-z][a-z0-9._-]*\z/ or usage();
safe_relative_path($registry_rel)
    or die "derived-state-authority: unsafe registry path\n";

my $contract = load_contract($registry_rel, $contract_id);
($contract->{classification} // '') eq 'verified_copy'
    or die "derived-state-authority: contract '$contract_id' is not a verified copy\n";

if ($contract_id eq 'rust_prerequisite_copies') {
    check_rust_prerequisite($contract);
} elsif ($contract_id eq 'fsmgen_gitlink_copies') {
    check_fsmgen_gitlink($contract);
} else {
    die "derived-state-authority: contract '$contract_id' is not a declared project adapter\n";
}

print "derived-state-authority: $contract_id agrees with its canonical authority.\n";
exit 0;

sub usage {
    die "Usage: $0 [--root DIR] [--registry PATH] --contract ID\n";
}

sub absolute {
    my ($relative) = @_;
    return File::Spec->catfile($root, split m{/}, $relative);
}

sub safe_relative_path {
    my ($path) = @_;
    return 0 if !defined($path) || ref($path) || $path eq '';
    return 0 if $path =~ m{\A/} || $path =~ /\\/ || $path =~ /[\x00-\x1f\x7f]/;
    return 0 if $path =~ m{(?:\A|/)\.\.?(/|\z)} || $path =~ m{//};
    return 1;
}

sub slurp {
    my ($relative, $label) = @_;
    safe_relative_path($relative)
        or die "derived-state-authority: unsafe $label path '$relative'\n";
    my $path = absolute($relative);
    -f $path && !-l $path
        or die "derived-state-authority: missing or non-regular $label '$relative'\n";
    open my $fh, '<:raw', $path
        or die "derived-state-authority: cannot read $label '$relative': $!\n";
    local $/;
    my $content = <$fh> // '';
    close $fh or die "derived-state-authority: cannot close $label '$relative': $!\n";
    return $content;
}

sub load_contract {
    my ($relative, $wanted) = @_;
    my $content = slurp($relative, 'derived-state registry');
    my @matches;
    my $line_number = 0;
    for my $line (split /\n/, $content) {
        $line_number++;
        next if $line eq '';
        my $record = eval { decode_json($line) };
        die "derived-state-authority: invalid registry JSON on line $line_number\n"
            if $@ || ref($record) ne 'HASH';
        push @matches, $record if ($record->{contract_id} // '') eq $wanted;
    }
    @matches == 1
        or die "derived-state-authority: expected one registry contract '$wanted', found "
            . scalar(@matches) . "\n";
    return $matches[0];
}

sub one_capture {
    my ($content, $pattern, $label) = @_;
    my @values = ($content =~ /$pattern/g);
    @values == 1
        or die "derived-state-authority: expected one $label, found " . scalar(@values) . "\n";
    return $values[0];
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

sub declared_copy {
    my ($record, $label) = @_;
    ref($record) eq 'HASH'
        or die "derived-state-authority: $label declaration must be an object\n";
    my $path = $record->{path};
    my $marker = $record->{field_marker};
    safe_relative_path($path)
        or die "derived-state-authority: $label has unsafe or missing declared path\n";
    defined($marker) && !ref($marker) && $marker ne ''
        or die "derived-state-authority: $label lacks a declared field marker\n";
    my $content = slurp($path, $label);
    my $count = literal_occurrences($content, $marker);
    $count == 1
        or die "derived-state-authority: $label marker must occur exactly once in '$path', found $count\n";
    return {
        path => $path,
        marker => $marker,
        content => $content,
    };
}

sub required_secondary_copies {
    my ($contract, @required_roles) = @_;
    my $copies = $contract->{secondary_copies};
    ref($copies) eq 'ARRAY'
        or die "derived-state-authority: contract lacks secondary_copies declarations\n";
    my %required = map { $_ => 1 } @required_roles;
    my %by_role;
    for my $copy (@$copies) {
        ref($copy) eq 'HASH'
            or die "derived-state-authority: secondary copy declaration must be an object\n";
        my $role = $copy->{role};
        defined($role) && !ref($role) && $role =~ /\A[a-z][a-z0-9._-]*\z/
            or die "derived-state-authority: secondary copy has invalid or missing role\n";
        $required{$role}
            or die "derived-state-authority: unknown secondary copy role '$role'\n";
        !exists $by_role{$role}
            or die "derived-state-authority: secondary copy role '$role' is declared more than once\n";
        $by_role{$role} = declared_copy($copy, "secondary copy role '$role'");
    }
    for my $role (@required_roles) {
        exists $by_role{$role}
            or die "derived-state-authority: required secondary copy role '$role' is missing\n";
    }
    return \%by_role;
}

sub normalize_rust_version {
    my ($value, $label) = @_;
    $value =~ /\A(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)(?:\.(0|[1-9][0-9]*))?\z/
        or die "derived-state-authority: $label has unsupported Rust version '$value'\n";
    return join('.', $1, $2, defined($3) ? $3 : 0);
}

sub check_rust_prerequisite {
    my ($contract) = @_;
    my $cargo = slurp('Cargo.toml', 'workspace manifest');
    my $primary = declared_copy($contract, 'primary Rust prerequisite');
    my $secondary = required_secondary_copies($contract, qw(rust_book rust_ci));

    my $authority = one_capture(
        $cargo,
        qr/^rust-version\s*=\s*"([^"]+)"\s*$/m,
        'workspace rust-version',
    );
    my $readme_copy = one_capture(
        $primary->{content},
        qr/^- Rust `([0-9]+\.[0-9]+\.[0-9]+)`\s*$/m,
        "$primary->{path} Rust prerequisite",
    );
    my $book_copy = one_capture(
        $secondary->{rust_book}{content},
        qr/^- Rust `([0-9]+\.[0-9]+\.[0-9]+)`\s*$/m,
        "$secondary->{rust_book}{path} Rust prerequisite",
    );
    my $ci_copy = one_capture(
        $secondary->{rust_ci}{content},
        qr/^\s*toolchain:\s*([0-9]+\.[0-9]+\.[0-9]+)\s*$/m,
        "$secondary->{rust_ci}{path} Rust toolchain",
    );

    my $normalized = normalize_rust_version($authority, 'workspace rust-version');
    for my $copy (
        [$primary->{path}, $readme_copy],
        [$secondary->{rust_book}{path}, $book_copy],
        [$secondary->{rust_ci}{path}, $ci_copy],
    ) {
        my $actual = normalize_rust_version($copy->[1], "$copy->[0] Rust copy");
        $actual eq $normalized
            or die "derived-state-authority: $copy->[0] Rust copy '$actual' differs from "
                . "workspace authority '$normalized'\n";
    }
}

sub check_fsmgen_gitlink {
    my ($field_contract) = @_;
    my $primary = declared_copy($field_contract, 'primary FSMGen feedback copy');
    my $secondary = required_secondary_copies($field_contract, 'feedback_contract_json');
    my $feedback = $primary->{content};
    my $contract_text = $secondary->{feedback_contract_json}{content};
    my $contract = eval { decode_json($contract_text) };
    die "derived-state-authority: FSMGen feedback contract is invalid JSON\n"
        if $@ || ref($contract) ne 'HASH';

    my $boundary = one_capture(
        $feedback,
        qr/## Current downstream boundary\s+(.*?)\s+## Open correspondence/s,
        'current downstream boundary',
    );
    my $feedback_copy = one_capture(
        $boundary,
        qr/\b([0-9a-f]{40})\b/,
        'current feedback gitlink',
    );
    my $required = $contract->{current_root}{required_literals};
    ref($required) eq 'ARRAY'
        or die "derived-state-authority: feedback contract lacks required_literals\n";
    my @contract_copies = grep { !ref($_) && /\A[0-9a-f]{40}\z/ } @$required;
    @contract_copies == 1
        or die "derived-state-authority: expected one contract gitlink literal, found "
            . scalar(@contract_copies) . "\n";

    open my $fh, '-|', 'git', '-C', $root, 'ls-files', '--stage', '--', 'subs/fsmgen'
        or die "derived-state-authority: cannot query FSMGen Git-index entry\n";
    my @rows = <$fh>;
    close $fh;
    $? == 0 or die "derived-state-authority: Git-index query failed\n";
    @rows == 1
        or die "derived-state-authority: expected one FSMGen Git-index entry, found "
            . scalar(@rows) . "\n";
    chomp $rows[0];
    $rows[0] =~ /\A(\d{6}) ([0-9a-f]{40,64}) (\d)\tsubs\/fsmgen\z/
        or die "derived-state-authority: malformed FSMGen Git-index entry\n";
    my ($mode, $object, $stage) = ($1, $2, $3);
    $mode eq '160000'
        or die "derived-state-authority: subs/fsmgen index mode is '$mode', expected '160000'\n";
    $stage eq '0'
        or die "derived-state-authority: subs/fsmgen index stage is '$stage', expected '0'\n";

    for my $copy (
        [$primary->{path}, $feedback_copy],
        [$secondary->{feedback_contract_json}{path}, $contract_copies[0]],
    ) {
        $copy->[1] eq $object
            or die "derived-state-authority: $copy->[0] gitlink '$copy->[1]' differs from "
                . "Git-index authority '$object'\n";
    }
}
