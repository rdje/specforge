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
my $adapter = File::Spec->catfile($root, 'scripts', 'check_derived_state_authorities.pl');
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

sub git_command {
    my ($directory, @args) = @_;
    system 'git', '-C', $directory, @args;
    die "fixture Git command failed: git @args\n" if $? != 0;
}

sub git_output {
    my ($directory, @args) = @_;
    open my $fh, '-|', 'git', '-C', $directory, @args
        or die "cannot run fixture Git query: git @args\n";
    local $/;
    my $output = <$fh> // '';
    close $fh;
    $? == 0 or die "fixture Git query failed: git @args\n";
    $output =~ s/\r?\n\z//;
    return $output;
}

sub save_registry {
    my ($fixture) = @_;
    my $content = $json->encode({
        record_type => 'registry',
        schema_version => 1,
        max_records => 8,
        max_bytes => 8_192,
        max_record_bytes => 2_048,
        max_array_items => 8,
        max_scalar_bytes => 512,
    }) . "\n";
    $content .= $json->encode($_) . "\n" for @{ $fixture->{contracts} };
    write_text(
        $fixture->{root},
        'doctrine/live_document_size/derived_state_contracts.jsonl',
        $content,
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
    my ($fixture, $contract_id, $role) = @_;
    my $copies = contract($fixture, $contract_id)->{secondary_copies};
    for my $copy (@$copies) {
        return $copy if ($copy->{role} // '') eq $role;
    }
    die "fixture secondary copy '$contract_id/$role' does not exist\n";
}

sub new_fixture {
    my $directory = tempdir(
        'derived-state-authority-tests.XXXXXX',
        DIR => $generated,
        CLEANUP => 1,
    );
    write_text($directory, 'README.md', "fixture seed\n");
    git_command($directory, 'init', '-q');
    git_command($directory, 'config', 'user.name', 'SpecForge Fixture');
    git_command($directory, 'config', 'user.email', 'fixture@specforge.invalid');
    git_command($directory, 'config', 'core.hooksPath', '.git/no-hooks');
    git_command($directory, 'add', 'README.md');
    git_command($directory, 'commit', '-q', '-m', 'fixture authority object');
    my $gitlink = git_output($directory, 'rev-parse', 'HEAD');

    write_text(
        $directory,
        'Cargo.toml',
        "[workspace]\nmembers = []\n\n[workspace.package]\nrust-version = \"1.95\"\n",
    );
    write_text($directory, 'README.md', "# Fixture\n\n- Rust `1.95.0`\n");
    write_text(
        $directory,
        'docs/book/src/getting-started.md',
        "# Getting Started\n\n- Rust `1.95.0`\n",
    );
    write_text(
        $directory,
        '.github/workflows/ci.yml',
        "jobs:\n  rust:\n    steps:\n      - with:\n          toolchain: 1.95.0\n",
    );
    write_text(
        $directory,
        'docs/FSMGEN_FEEDBACK.md',
        "# Feedback\n\n## Current downstream boundary\n\nPinned object `$gitlink`.\n\n## Open correspondence\n\n- None.\n",
    );
    write_text(
        $directory,
        'doctrine/live_document_size/fsmgen_feedback.json',
        $json->encode({ current_root => { required_literals => [$gitlink] } }) . "\n",
    );

    my $fixture = {
        root => $directory,
        gitlink => $gitlink,
        contracts => [
            {
                record_type => 'contract',
                schema_version => 1,
                contract_id => 'rust_prerequisite_copies',
                classification => 'verified_copy',
                path => 'README.md',
                field_marker => '- Rust `1.95.0`',
                secondary_copies => [
                    {
                        role => 'rust_book',
                        ownership => 'surface',
                        surface_id => 'shipped_behavior',
                        path => 'docs/book/src/getting-started.md',
                        field_marker => '- Rust `1.95.0`',
                    },
                    {
                        role => 'rust_ci',
                        ownership => 'control',
                        path => '.github/workflows/ci.yml',
                        field_marker => 'toolchain: 1.95.0',
                    },
                ],
            },
            {
                record_type => 'contract',
                schema_version => 1,
                contract_id => 'fsmgen_gitlink_copies',
                classification => 'verified_copy',
                path => 'docs/FSMGEN_FEEDBACK.md',
                field_marker => $gitlink,
                secondary_copies => [
                    {
                        role => 'feedback_contract_json',
                        ownership => 'control',
                        path => 'doctrine/live_document_size/fsmgen_feedback.json',
                        field_marker => $gitlink,
                    },
                ],
            },
        ],
    };
    save_registry($fixture);
    git_command(
        $directory,
        'update-index',
        '--add',
        '--cacheinfo',
        "160000,$gitlink,subs/fsmgen",
    );
    return $fixture;
}

sub run_adapter {
    my ($fixture, $contract_id) = @_;
    my $log_path = path_in($fixture->{root}, 'adapter-output.log');
    open my $log, '>:raw', $log_path or die "cannot write adapter log: $!\n";
    my @command = (
        $^X,
        $adapter,
        '--root', $fixture->{root},
        '--registry', 'doctrine/live_document_size/derived_state_contracts.jsonl',
        '--contract', $contract_id,
    );
    my $pid = fork();
    die "cannot fork authority adapter test: $!\n" if !defined $pid;
    if ($pid == 0) {
        open STDOUT, '>&', $log or exit 126;
        open STDERR, '>&', $log or exit 126;
        exec { $command[0] } @command;
        exit 126;
    }
    waitpid($pid, 0);
    my $status = $? >> 8;
    close $log;
    return ($status, read_text($fixture->{root}, 'adapter-output.log'));
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
    my ($name, $contract_id, $expect_success, $pattern, $mutator) = @_;
    my $fixture = new_fixture();
    $mutator->($fixture) if $mutator;
    save_registry($fixture);
    my ($status, $output) = run_adapter($fixture, $contract_id);
    my $passed = $expect_success ? $status == 0 : $status != 0;
    $passed &&= $output =~ $pattern if defined $pattern;
    report_result($name, $passed, $output);
}

{
    my $source = read_text($root, 'scripts/check_derived_state_authorities.pl');
    my @hidden = grep { index($source, $_) >= 0 } (
        'docs/book/src/getting-started.md',
        '.github/workflows/ci.yml',
        'doctrine/live_document_size/fsmgen_feedback.json',
    );
    report_result(
        'adapter source contains no secondary copy path literals',
        !@hidden,
        @hidden ? "hidden paths: @hidden\n" : '',
    );
}

expect_case(
    'workspace short semver normalizes and all Rust copies agree',
    'rust_prerequisite_copies',
    1,
    qr/agrees with its canonical authority/,
    undef,
);
expect_case(
    'workspace patch semver remains stable',
    'rust_prerequisite_copies',
    1,
    qr/agrees with its canonical authority/,
    sub { write_text($_[0]{root}, 'Cargo.toml', "[workspace.package]\nrust-version = \"1.95.0\"\n"); },
);
expect_case('README Rust drift fails closed', 'rust_prerequisite_copies', 0, qr/README\.md Rust copy .* differs/, sub {
    my ($fixture) = @_;
    write_text($fixture->{root}, 'README.md', "# Fixture\n\n- Rust `1.96.0`\n");
    contract($fixture, 'rust_prerequisite_copies')->{field_marker} = '- Rust `1.96.0`';
});
expect_case('mdBook Rust drift fails closed', 'rust_prerequisite_copies', 0, qr/getting-started\.md Rust copy .* differs/, sub {
    my ($fixture) = @_;
    write_text($fixture->{root}, 'docs/book/src/getting-started.md', "# Getting Started\n\n- Rust `1.96.0`\n");
    secondary_copy($fixture, 'rust_prerequisite_copies', 'rust_book')->{field_marker} = '- Rust `1.96.0`';
});
expect_case('CI Rust drift fails closed', 'rust_prerequisite_copies', 0, qr/ci\.yml Rust copy .* differs/, sub {
    my ($fixture) = @_;
    write_text($fixture->{root}, '.github/workflows/ci.yml', "toolchain: 1.96.0\n");
    secondary_copy($fixture, 'rust_prerequisite_copies', 'rust_ci')->{field_marker} = 'toolchain: 1.96.0';
});
expect_case('unsupported workspace semver fails closed', 'rust_prerequisite_copies', 0, qr/unsupported Rust version/, sub {
    write_text($_[0]{root}, 'Cargo.toml', "[workspace.package]\nrust-version = \"1.95-beta\"\n");
});
expect_case('duplicate Rust declarations fail closed', 'rust_prerequisite_copies', 0, qr/primary Rust prerequisite marker must occur exactly once.*found 2/, sub {
    write_text($_[0]{root}, 'README.md', "- Rust `1.95.0`\n- Rust `1.95.0`\n");
});
expect_case(
    'feedback and JSON copies agree with the mode-160000 index object',
    'fsmgen_gitlink_copies',
    1,
    qr/agrees with its canonical authority/,
    undef,
);
expect_case('feedback gitlink drift fails closed', 'fsmgen_gitlink_copies', 0, qr/FSMGEN_FEEDBACK\.md gitlink .* differs/, sub {
    my ($fixture) = @_;
    my $wrong = '0' x 40;
    write_text(
        $fixture->{root},
        'docs/FSMGEN_FEEDBACK.md',
        "# Feedback\n\n## Current downstream boundary\n\nPinned object `$wrong`.\n\n## Open correspondence\n",
    );
    contract($fixture, 'fsmgen_gitlink_copies')->{field_marker} = $wrong;
});
expect_case('feedback JSON gitlink drift fails closed', 'fsmgen_gitlink_copies', 0, qr/fsmgen_feedback\.json gitlink .* differs/, sub {
    my ($fixture) = @_;
    write_text(
        $fixture->{root},
        'doctrine/live_document_size/fsmgen_feedback.json',
        $json->encode({ current_root => { required_literals => ['0' x 40] } }) . "\n",
    );
    secondary_copy($fixture, 'fsmgen_gitlink_copies', 'feedback_contract_json')->{field_marker} = '0' x 40;
});
expect_case('non-gitlink index mode fails closed', 'fsmgen_gitlink_copies', 0, qr/index mode is '100644'/, sub {
    my ($fixture) = @_;
    my $blob = git_output($fixture->{root}, 'hash-object', '-w', 'README.md');
    git_command(
        $fixture->{root},
        'update-index',
        '--add',
        '--cacheinfo',
        "100644,$blob,subs/fsmgen",
    );
});
expect_case('missing gitlink index entry fails closed', 'fsmgen_gitlink_copies', 0, qr/expected one FSMGen Git-index entry, found 0/, sub {
    git_command($_[0]{root}, 'update-index', '--force-remove', 'subs/fsmgen');
});
expect_case('duplicate current-boundary hashes fail closed', 'fsmgen_gitlink_copies', 0, qr/primary FSMGen feedback copy marker must occur exactly once.*found 2/, sub {
    my ($fixture) = @_;
    my $hash = $fixture->{gitlink};
    write_text(
        $fixture->{root},
        'docs/FSMGEN_FEEDBACK.md',
        "# Feedback\n\n## Current downstream boundary\n\n$hash and $hash.\n\n## Open correspondence\n",
    );
});
expect_case('invalid feedback JSON fails closed', 'fsmgen_gitlink_copies', 0, qr/contract is invalid JSON/, sub {
    my ($fixture) = @_;
    write_text(
        $fixture->{root},
        'doctrine/live_document_size/fsmgen_feedback.json',
        '{invalid "' . $fixture->{gitlink} . '"' . "\n",
    );
});
expect_case('declared alternate mdBook path works without fallback', 'rust_prerequisite_copies', 1, qr/agrees with its canonical authority/, sub {
    my ($fixture) = @_;
    write_text($fixture->{root}, 'declared/book-copy.md', "- Rust `1.95.0`\n");
    write_text($fixture->{root}, 'docs/book/src/getting-started.md', "- Rust `9.9.9`\n");
    secondary_copy($fixture, 'rust_prerequisite_copies', 'rust_book')->{path} = 'declared/book-copy.md';
});
expect_case('declared alternate CI path works without fallback', 'rust_prerequisite_copies', 1, qr/agrees with its canonical authority/, sub {
    my ($fixture) = @_;
    write_text($fixture->{root}, 'declared/ci-copy.yml', "toolchain: 1.95.0\n");
    write_text($fixture->{root}, '.github/workflows/ci.yml', "toolchain: 9.9.9\n");
    secondary_copy($fixture, 'rust_prerequisite_copies', 'rust_ci')->{path} = 'declared/ci-copy.yml';
});
expect_case('declared alternate JSON path works without fallback', 'fsmgen_gitlink_copies', 1, qr/agrees with its canonical authority/, sub {
    my ($fixture) = @_;
    my $content = read_text($fixture->{root}, 'doctrine/live_document_size/fsmgen_feedback.json');
    write_text($fixture->{root}, 'declared/feedback-copy.json', $content);
    write_text($fixture->{root}, 'doctrine/live_document_size/fsmgen_feedback.json', "{broken\n");
    secondary_copy($fixture, 'fsmgen_gitlink_copies', 'feedback_contract_json')->{path} = 'declared/feedback-copy.json';
});
expect_case('missing secondary roles fail closed', 'rust_prerequisite_copies', 0, qr/required secondary copy role 'rust_ci' is missing/, sub {
    pop @{ contract($_[0], 'rust_prerequisite_copies')->{secondary_copies} };
});
expect_case('duplicate secondary roles fail closed', 'rust_prerequisite_copies', 0, qr/secondary copy role 'rust_book' is declared more than once/, sub {
    secondary_copy($_[0], 'rust_prerequisite_copies', 'rust_ci')->{role} = 'rust_book';
});
expect_case('unknown secondary roles fail closed', 'rust_prerequisite_copies', 0, qr/unknown secondary copy role 'hidden_fallback'/, sub {
    secondary_copy($_[0], 'rust_prerequisite_copies', 'rust_ci')->{role} = 'hidden_fallback';
});
expect_case('unsafe declared secondary paths fail closed', 'rust_prerequisite_copies', 0, qr/secondary copy role 'rust_ci' has unsafe or missing declared path/, sub {
    secondary_copy($_[0], 'rust_prerequisite_copies', 'rust_ci')->{path} = '../ci.yml';
});
expect_case('missing declared secondary markers fail closed', 'rust_prerequisite_copies', 0, qr/secondary copy role 'rust_book' marker must occur exactly once.*found 0/, sub {
    secondary_copy($_[0], 'rust_prerequisite_copies', 'rust_book')->{field_marker} = '- Rust `1.94.0`';
});
expect_case('unknown adapter contracts fail closed', 'unknown_contract', 0, qr/expected one registry contract 'unknown_contract'/, undef);
expect_case('non-verified adapter contracts fail closed', 'rust_prerequisite_copies', 0, qr/is not a verified copy/, sub {
    $_[0]{contracts}[0]{classification} = 'authored_intent';
});

print "1..$test_number\n" if !$quiet;
if ($failures) {
    print STDERR "derived-state-authority-tests: $failures of $test_number checks failed\n";
    exit 1;
}
# PRODUCTION-GRAPH-CENSUS-PIN.3 — `all $test_number ... pass` is a running counter with nothing
# to compare it against: delete a check and the line simply reports one fewer. The expected
# count is declared here, independently of the suite, so a check removed — or one added and not
# declared — fails instead of silently shrinking the coverage this reports.
my $expected_checks = 25;
die "derived-state-authority-tests: ran $test_number checks, declaration expects $expected_checks — "
    . "re-derive the declaration beside the suite\n"
    if $test_number != $expected_checks;
print STDERR "derived-state-authority-tests: $test_number/$expected_checks Rust/gitlink authority checks pass\n";
exit 0;
