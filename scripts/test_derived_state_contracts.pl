#!/usr/bin/env perl
use strict;
use warnings;

use Cwd qw(abs_path);
use File::Basename qw(dirname);
use File::Path qw(make_path);
use File::Spec;
use File::Temp qw(tempdir);
use JSON::PP;

my $root = abs_path(File::Spec->catdir(dirname(abs_path($0)), '..'));
my $checker = File::Spec->catfile($root, 'scripts', 'check_derived_state_contracts.pl');
my $generated = File::Spec->catdir($root, 'generated');
make_path($generated);

my $quiet = 0;
if (@ARGV) {
    die "Usage: $0 [--quiet]\n" if @ARGV != 1 || $ARGV[0] ne '--quiet';
    $quiet = 1;
}

my $json = JSON::PP->new->canonical(1);
my $test_number = 0;
my $failures = 0;

sub path_in {
    my ($directory, $relative) = @_;
    return File::Spec->catfile($directory, split m{/}, $relative);
}

sub write_text {
    my ($directory, $relative, $content) = @_;
    my $path = path_in($directory, $relative);
    make_path(dirname($path));
    open my $fh, '>:raw', $path or die "cannot write fixture '$path': $!\n";
    print {$fh} $content;
    close $fh or die "cannot close fixture '$path': $!\n";
}

sub read_text {
    my ($directory, $relative) = @_;
    open my $fh, '<:raw', path_in($directory, $relative)
        or die "cannot read fixture '$relative': $!\n";
    local $/;
    my $content = <$fh> // '';
    close $fh;
    return $content;
}

sub save_jsonl {
    my ($fixture, $relative, $meta, $records) = @_;
    my $content = $json->encode($meta) . "\n";
    $content .= $json->encode($_) . "\n" for @$records;
    write_text($fixture->{root}, $relative, $content);
}

sub save_fixture {
    my ($fixture) = @_;
    save_jsonl(
        $fixture,
        'control/surfaces.jsonl',
        $fixture->{surface_meta},
        $fixture->{surfaces},
    );
    save_jsonl(
        $fixture,
        'control/derived.jsonl',
        $fixture->{derived_meta},
        $fixture->{contracts},
    );
}

sub contract {
    my ($fixture, $id) = @_;
    for my $candidate (@{ $fixture->{contracts} }) {
        return $candidate if ($candidate->{contract_id} // '') eq $id;
    }
    die "fixture contract '$id' does not exist\n";
}

sub secondary_copy {
    my ($fixture, $role) = @_;
    my $copies = contract($fixture, 'adapter_copy')->{secondary_copies};
    for my $copy (@$copies) {
        return $copy if ($copy->{role} // '') eq $role;
    }
    die "fixture secondary copy '$role' does not exist\n";
}

sub new_fixture {
    my $directory = tempdir(
        'derived-state-tests.XXXXXX',
        DIR => $generated,
        CLEANUP => 1,
    );
    write_text(
        $directory,
        'current.md',
        "# Current\n" .
        "Reader accessor: fixture-current\n" .
        "Verified core: yes\n" .
        "Verified adapter: yes\n" .
        "Authored intent: selected\n" .
        "Undeclared date 2099-01-01 and hash deadbeef remain ordinary prose.\n",
    );
    write_text(
        $directory,
        'evidence.md',
        "# Evidence\nImmutable measurement: 7\nCaptured by fixture revision\n",
    );
    write_text($directory, 'history.md', "# Historical terminal\n");
    write_text($directory, 'secondary.md', "# Secondary\nSurface copy: yes\n");
    write_text($directory, 'control.yml', "control_copy: yes\n");
    write_text(
        $directory,
        'scripts/core-ok.pl',
        "use strict; use warnings; open my \$fh, '>', 'core-proof.txt' or die \$!; " .
        "print {\$fh} \"ok\\n\"; close \$fh; exit 0;\n",
    );
    write_text($directory, 'scripts/core-fail.pl', "exit 9;\n");
    write_text(
        $directory,
        'scripts/adapter-ok.pl',
        "use strict; use warnings; open my \$fh, '>', 'adapter-proof.txt' or die \$!; " .
        "print {\$fh} join(' ', \@ARGV), \"\\n\"; close \$fh; exit 0;\n",
    );
    write_text($directory, 'scripts/adapter-fail.pl', "exit 7;\n");

    my $surface_meta = {
        record_type => 'registry',
        schema_version => 1,
        max_records => 8,
        max_bytes => 8_192,
        max_record_bytes => 2_048,
        max_array_items => 8,
        max_scalar_bytes => 512,
    };
    my $derived_meta = {
        record_type => 'registry',
        schema_version => 1,
        max_records => 12,
        max_bytes => 16_384,
        max_record_bytes => 2_048,
        max_array_items => 8,
        max_scalar_bytes => 512,
    };
    my $fixture = {
        root => $directory,
        surface_meta => $surface_meta,
        surfaces => [
            {
                surface_id => 'current_surface',
                targets => ['current.md'],
                lifecycle => 'bounded_snapshot',
            },
            {
                surface_id => 'evidence_surface',
                targets => ['evidence.md'],
                lifecycle => 'partitioned_canonical',
            },
            {
                surface_id => 'historical_surface',
                targets => ['history.md'],
                lifecycle => 'archive_terminal',
            },
            {
                surface_id => 'secondary_surface',
                targets => ['secondary.md'],
                lifecycle => 'maintained_reference',
            },
        ],
        derived_meta => $derived_meta,
        contracts => [
            {
                record_type => 'contract',
                schema_version => 1,
                contract_id => 'revision',
                surface_id => 'current_surface',
                path => 'current.md',
                field_id => 'revision',
                classification => 'derive_on_read',
                field_marker => 'Reader accessor: fixture-current',
                authority => 'fixture revision authority',
                accessor => 'fixture-current',
                forbidden_storage_marker => 'Stored revision:',
                verifier => 'builtin:derive_on_read',
            },
            {
                record_type => 'contract',
                schema_version => 1,
                contract_id => 'core_copy',
                surface_id => 'current_surface',
                path => 'current.md',
                field_id => 'core_copy',
                classification => 'verified_copy',
                field_marker => 'Verified core: yes',
                authority => 'fixture core authority',
                accessor => 'scripts/core-ok.pl',
                verifier => 'core:scripts/core-ok.pl',
            },
            {
                record_type => 'contract',
                schema_version => 1,
                contract_id => 'adapter_copy',
                surface_id => 'current_surface',
                path => 'current.md',
                field_id => 'adapter_copy',
                classification => 'verified_copy',
                field_marker => 'Verified adapter: yes',
                authority => 'fixture adapter authority',
                accessor => 'scripts/adapter-ok.pl',
                verifier => 'adapter:scripts/adapter-ok.pl',
                secondary_copies => [
                    {
                        role => 'surface_copy',
                        ownership => 'surface',
                        surface_id => 'secondary_surface',
                        path => 'secondary.md',
                        field_marker => 'Surface copy: yes',
                    },
                    {
                        role => 'control_copy',
                        ownership => 'control',
                        path => 'control.yml',
                        field_marker => 'control_copy: yes',
                    },
                ],
            },
            {
                record_type => 'contract',
                schema_version => 1,
                contract_id => 'intent',
                surface_id => 'current_surface',
                path => 'current.md',
                field_id => 'intent',
                classification => 'authored_intent',
                field_marker => 'Authored intent: selected',
                authority => 'fixture author judgement',
            },
            {
                record_type => 'contract',
                schema_version => 1,
                contract_id => 'evidence',
                surface_id => 'evidence_surface',
                path => 'evidence.md',
                field_id => 'measurement',
                classification => 'immutable_evidence',
                field_marker => 'Immutable measurement: 7',
                authority => 'fixture capture record',
                capture_boundary => 'Captured by fixture revision',
            },
        ],
    };
    save_fixture($fixture);
    return $fixture;
}

sub run_checker {
    my ($fixture) = @_;
    my $log_path = path_in($fixture->{root}, 'check-output.log');
    open my $log, '>:raw', $log_path or die "cannot write checker log: $!\n";
    my @command = (
        $^X,
        $checker,
        '--root', $fixture->{root},
        '--registry', 'control/derived.jsonl',
        '--surfaces', 'control/surfaces.jsonl',
    );
    my $pid = fork();
    die "cannot fork derived-state test: $!\n" if !defined $pid;
    if ($pid == 0) {
        open STDOUT, '>&', $log or exit 126;
        open STDERR, '>&', $log or exit 126;
        exec { $command[0] } @command;
        exit 126;
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
    save_fixture($fixture);
    my ($status, $output) = run_checker($fixture);
    my $passed = $expect_success ? $status == 0 : $status != 0;
    $passed &&= $output =~ $pattern if defined $pattern;
    report_result($name, $passed, $output);
}

{
    my $source = read_text($root, 'scripts/check_derived_state_contracts.pl');
    my @project_terms = grep { index($source, $_) >= 0 } qw(
      rust_prerequisite_copies fsmgen_gitlink_copies rust_book rust_ci feedback_contract_json
    );
    report_result(
        'neutral checker contains no project contract identifiers or adapter roles',
        !@project_terms,
        @project_terms ? "project terms: @project_terms\n" : '',
    );
}

{
    my $fixture = new_fixture();
    my ($status, $output) = run_checker($fixture);
    my $passed = $status == 0
        && -f path_in($fixture->{root}, 'core-proof.txt')
        && -f path_in($fixture->{root}, 'adapter-proof.txt');
    report_result('all four classifications and both verifier kinds pass', $passed, $output);
}

expect_case('undeclared date, number, and hash lookalikes are not guessed', 1, qr/5 explicit field contracts/, sub {
    my ($fixture) = @_;
    write_text(
        $fixture->{root},
        'current.md',
        read_text($fixture->{root}, 'current.md') .
            "Build 999 at 2099-12-31 has token 0123456789abcdef.\n",
    );
});
expect_case('invalid classification fails closed', 0, qr/invalid classification/, sub {
    contract($_[0], 'intent')->{classification} = 'ambient_guess';
});
expect_case('unknown contract fields fail closed', 0, qr/unknown field 'heuristic'/, sub {
    contract($_[0], 'intent')->{heuristic} = 'date';
});
expect_case('duplicate contract identifiers fail closed', 0, qr/declared more than once/, sub {
    my ($fixture) = @_;
    my %copy = %{ contract($fixture, 'intent') };
    push @{ $fixture->{contracts} }, \%copy;
});
expect_case('duplicate surface path field declarations fail closed', 0, qr/field is declared more than once/, sub {
    my ($fixture) = @_;
    my %copy = %{ contract($fixture, 'intent') };
    $copy{contract_id} = 'intent_duplicate_field';
    push @{ $fixture->{contracts} }, \%copy;
});
expect_case('duplicate exact markers fail closed', 0, qr/marker is declared more than once/, sub {
    my ($fixture) = @_;
    my %copy = %{ contract($fixture, 'intent') };
    $copy{contract_id} = 'intent_duplicate_marker';
    $copy{field_id} = 'intent_other_field';
    push @{ $fixture->{contracts} }, \%copy;
});
expect_case('secondary copies reject unknown fields', 0, qr/unknown field 'fallback_path'/, sub {
    secondary_copy($_[0], 'control_copy')->{fallback_path} = 'hidden.yml';
});
expect_case('non-copy classes reject secondary declarations', 0, qr/unknown field 'secondary_copies'/, sub {
    contract($_[0], 'intent')->{secondary_copies} = [];
});
expect_case('secondary copies must be a non-empty array', 0, qr/secondary_copies must be a non-empty array/, sub {
    contract($_[0], 'adapter_copy')->{secondary_copies} = [];
});
expect_case('secondary copy roles must be unique', 0, qr/secondary role 'surface_copy' is declared more than once/, sub {
    secondary_copy($_[0], 'control_copy')->{role} = 'surface_copy';
});
expect_case('secondary copy roles use bounded identifiers', 0, qr/role 'Surface Copy' has an invalid identifier shape/, sub {
    secondary_copy($_[0], 'surface_copy')->{role} = 'Surface Copy';
});
expect_case('secondary copy ownership is closed', 0, qr/invalid ownership 'external'/, sub {
    secondary_copy($_[0], 'control_copy')->{ownership} = 'external';
});
expect_case('secondary copy paths reject traversal', 0, qr/secondary_copies\[1\] path is absolute, escaping, or malformed/, sub {
    secondary_copy($_[0], 'control_copy')->{path} = '../control.yml';
});
expect_case('secondary copy paths require regular files', 0, qr/path is missing or not a regular file: missing.yml/, sub {
    secondary_copy($_[0], 'control_copy')->{path} = 'missing.yml';
});
expect_case('secondary copy paths reject symlinks', 0, qr/path is missing or not a regular file: linked.yml/, sub {
    my ($fixture) = @_;
    symlink path_in($fixture->{root}, 'control.yml'), path_in($fixture->{root}, 'linked.yml')
        or die "cannot create fixture symlink: $!\n";
    secondary_copy($fixture, 'control_copy')->{path} = 'linked.yml';
});
expect_case('surface copies require a surface identifier', 0, qr/surface ownership requires surface_id/, sub {
    delete secondary_copy($_[0], 'surface_copy')->{surface_id};
});
expect_case('surface copies reject unknown surfaces', 0, qr/names unknown surface 'missing_surface'/, sub {
    secondary_copy($_[0], 'surface_copy')->{surface_id} = 'missing_surface';
});
expect_case('surface copies reject historical surfaces', 0, qr/must govern a current maintained surface/, sub {
    my ($fixture) = @_;
    my $copy = secondary_copy($fixture, 'surface_copy');
    $copy->{surface_id} = 'historical_surface';
    $copy->{path} = 'history.md';
    $copy->{field_marker} = '# Historical terminal';
});
expect_case('surface copies reject off-surface paths', 0, qr/path is outside surface 'secondary_surface'/, sub {
    my ($fixture) = @_;
    my $copy = secondary_copy($fixture, 'surface_copy');
    $copy->{path} = 'current.md';
    $copy->{field_marker} = '# Current';
});
expect_case('control copies reject surface identifiers', 0, qr/control ownership must not declare surface_id/, sub {
    secondary_copy($_[0], 'control_copy')->{surface_id} = 'secondary_surface';
});
expect_case('Markdown secondary copies require surface ownership', 0, qr/Markdown path must use surface ownership/, sub {
    my ($fixture) = @_;
    my $copy = secondary_copy($fixture, 'control_copy');
    $copy->{path} = 'secondary.md';
    $copy->{field_marker} = 'Surface copy: yes';
});
expect_case('secondary copies reject missing exact markers', 0, qr/secondary_copies\[1\] field marker must occur exactly once.*found 0/, sub {
    secondary_copy($_[0], 'control_copy')->{field_marker} = 'control_copy: missing';
});
expect_case('secondary copies reject duplicated exact markers', 0, qr/secondary_copies\[1\] field marker must occur exactly once.*found 2/, sub {
    my ($fixture) = @_;
    write_text($fixture->{root}, 'control.yml', "control_copy: yes\ncontrol_copy: yes\n");
});
expect_case('secondary and primary locations cannot duplicate exact markers', 0, qr/marker is declared more than once/, sub {
    my ($fixture) = @_;
    my $copy = secondary_copy($fixture, 'control_copy');
    $copy->{path} = 'current.md';
    $copy->{field_marker} = 'Verified adapter: yes';
});
expect_case('unsafe paths fail closed', 0, qr/path is absolute, escaping, or malformed/, sub {
    contract($_[0], 'intent')->{path} = '../escape.md';
});
expect_case('unknown surfaces fail closed', 0, qr/names unknown surface/, sub {
    contract($_[0], 'intent')->{surface_id} = 'unknown_surface';
});
expect_case('historical surfaces reject current-state contracts', 0, qr/must govern a current maintained surface/, sub {
    my ($fixture) = @_;
    my $record = contract($fixture, 'intent');
    $record->{surface_id} = 'historical_surface';
    $record->{path} = 'history.md';
    $record->{field_marker} = '# Historical terminal';
});
expect_case('off-surface paths fail closed', 0, qr/path is outside surface/, sub {
    my ($fixture) = @_;
    my $record = contract($fixture, 'intent');
    $record->{path} = 'evidence.md';
    $record->{field_marker} = '# Evidence';
});
expect_case('missing contract files fail closed', 0, qr/path is missing or not a regular file/, sub {
    my ($fixture) = @_;
    $fixture->{surfaces}[0]{targets} = ['missing.md'];
    contract($fixture, 'intent')->{path} = 'missing.md';
});
expect_case('missing exact field markers fail closed', 0, qr/field marker must occur exactly once.*found 0/, sub {
    contract($_[0], 'intent')->{field_marker} = 'Missing intent marker';
});
expect_case('duplicated exact field markers fail closed', 0, qr/field marker must occur exactly once.*found 2/, sub {
    my ($fixture) = @_;
    write_text(
        $fixture->{root},
        'current.md',
        read_text($fixture->{root}, 'current.md') . "Authored intent: selected\n",
    );
});
expect_case('derive-on-read rejects a stored current value', 0, qr/still stores forbidden current-state marker/, sub {
    my ($fixture) = @_;
    write_text(
        $fixture->{root},
        'current.md',
        read_text($fixture->{root}, 'current.md') . "Stored revision: abc123\n",
    );
});
expect_case('derive-on-read rejects a missing reader accessor', 0, qr/accessor is absent/, sub {
    contract($_[0], 'revision')->{accessor} = 'fixture-missing-accessor';
});
expect_case('derive-on-read rejects the wrong verifier class', 0, qr/must use builtin:derive_on_read/, sub {
    contract($_[0], 'revision')->{verifier} = 'core:scripts/core-ok.pl';
});
expect_case('verified copies reject undeclared verifier schemes', 0, qr/must use core: or adapter:/, sub {
    contract($_[0], 'core_copy')->{verifier} = 'external:trust-me';
});
expect_case('verified copies reject missing core verifiers', 0, qr/verifier is missing/, sub {
    contract($_[0], 'core_copy')->{verifier} = 'core:scripts/missing.pl';
});
expect_case('verified copies reject failing core verifiers', 0, qr/verifier failed .*core-fail\.pl/, sub {
    contract($_[0], 'core_copy')->{verifier} = 'core:scripts/core-fail.pl';
});
expect_case('verified copies reject failing adapters', 0, qr/verifier failed .*adapter-fail\.pl/, sub {
    contract($_[0], 'adapter_copy')->{verifier} = 'adapter:scripts/adapter-fail.pl';
});
expect_case('authored intent rejects mechanical verifier fields', 0, qr/unknown field 'verifier'/, sub {
    contract($_[0], 'intent')->{verifier} = 'builtin:none';
});
expect_case('immutable evidence rejects a missing capture boundary', 0, qr/capture boundary must occur exactly once.*found 0/, sub {
    contract($_[0], 'evidence')->{capture_boundary} = 'Missing capture boundary';
});
expect_case('registry record-count bounds fail closed', 0, qr/more records than its declared max_records/, sub {
    $_[0]{derived_meta}{max_records} = 4;
});
expect_case('registry total-byte bounds fail closed', 0, qr/exceeds its declared max_bytes/, sub {
    $_[0]{derived_meta}{max_bytes} = 512;
});
expect_case('registry raw-record bounds fail closed', 0, qr/record above its declared max_record_bytes/, sub {
    $_[0]{derived_meta}{max_record_bytes} = 128;
});
expect_case('registry scalar bounds fail closed', 0, qr/exceeds the declared scalar byte limit 8/, sub {
    $_[0]{derived_meta}{max_scalar_bytes} = 8;
});
expect_case('registry schema version fails closed', 0, qr/schema_version must be 1/, sub {
    $_[0]{derived_meta}{schema_version} = 2;
});

print "1..$test_number\n" if !$quiet;
if ($failures) {
    print STDERR "derived-state-tests: $failures of $test_number checks failed\n";
    exit 1;
}
print STDERR "derived-state-tests: all $test_number classification and authority-control checks pass\n";
exit 0;
