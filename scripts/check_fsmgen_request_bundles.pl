#!/usr/bin/env perl
# FSMGEN-REQUESTS.0 — enforce the signoff standard of docs/fsmgen-issues/PROCESS.md on every request
# bundle: layout, summary fields, request.json, signoff boxes, verification on the pin and upstream,
# ownership (task leaf + open feedback record), and portability (no machine-local absolute path).
# ADR 0054 (UPSTREAM-CHANNEL-UNIFICATION.2): a question is a packet too, and the ledger indexes the packets —
# every outgoing open record has exactly one packet, cites its README, and agrees with it on kind and status.
# Run from anywhere: perl scripts/check_fsmgen_request_bundles.pl [--check] [--root PROJECT_ROOT]
use strict;
use warnings;
use Cwd qw(abs_path);
use File::Basename qw(dirname);
use File::Find qw(find);
use File::Copy qw(copy);
use File::Path qw(make_path);
use File::Temp qw(tempdir);
use JSON::PP;

my $root = abs_path(dirname(__FILE__) . '/..');
my $self_test = 0;
while (@ARGV) {
    my $arg = shift @ARGV;
    if ($arg eq '--check') {
        next;
    } elsif ($arg eq '--self-test') {
        $self_test = 1;
    } elsif ($arg eq '--root') {
        $root = abs_path(shift(@ARGV) // die usage());
    } else {
        die usage();
    }
}
sub usage { "Usage: $0 [--check|--self-test] [--root PROJECT_ROOT]\n" }

# Bundles filed before PROCESS.md existed; closed history, exempt from the signoff standard.
my %LEGACY = map { $_ => 1 } qw(
    sf-isf-contract-eventually-flat
    sf-isf-stage-ready-valid
    sf-isf-explicit-fsm-declaration
);
# ADR 0054: a question travels as the same packet as a request (UPSTREAM-CHANNEL-UNIFICATION.3).
my %KIND = map { $_ => 1 } qw(bug_report feature_request question);
my @LIFECYCLE = qw(drafted signed_off forwarded acknowledged resolved_upstream integrated rejected withdrawn);
my %RANK = map { $LIFECYCLE[$_] => $_ } 0 .. $#LIFECYCLE;
my %CLOSED = map { $_ => 1 } qw(integrated rejected withdrawn);
# PROCESS.md §6: the ledger status each open packet status requires, and the kinds that go out.
my %RECORD_STATUS = (
    drafted => [qw(open)], signed_off => [qw(open)], forwarded => [qw(awaiting_response)],
    acknowledged => [qw(accepted)], resolved_upstream => [qw(shipped resolved answered)],
);
my %OUTGOING_KIND = map { $_ => 1 } qw(bug_report feature_request question clarity_request);
my @SUMMARY_FIELDS = (
    'Title', 'FSMGen command or public API entrypoint', 'FSMGen-facing primary artifact path',
    'Failure class', 'FSMGen commit or capability-manifest producer commit', 'SPECFORGE version/commit',
    'Expected behavior', 'Observed behavior', 'First failing command or API call',
    'Exit status or exception', 'Is the attached artifact bundle minimized',
    'Does the minimized/redacted bundle still reproduce',
);
my @REQUEST_FIELDS = qw(schema_version id kind title owner feedback_record failure_class verified_on
    specforge_side status signed_off forwarded upstream_reference resolution);
# A machine-local absolute path: a root-anchored home, volume or temp directory not preceded by a word,
# `>` (a placeholder such as `<FSMGEN_ROOT>/`) or `.` (a relative `.artifacts/tmp/`).
my $ABSOLUTE = qr{(?<![\w>.])/(?:Users|Volumes|home|private|tmp)/};

sub slurp {
    my ($path) = @_;
    open my $fh, '<:raw', $path or return undef;
    local $/;
    my $text = <$fh>;
    return $text;
}

our @ERRORS;
sub problem { push @ERRORS, "fsmgen-request-bundles: $_[0]" }

# Every violation of PROCESS.md in the tree rooted at $root, and the number of bundles checked.
sub check_root {
    my ($root) = @_;
    local @ERRORS = ();

    my $issues = "$root/docs/fsmgen-issues";
    opendir my $dh, $issues or return (["fsmgen-request-bundles: cannot read $issues: $!"], 0);
    my @bundles = sort grep { -d "$issues/$_" && $_ !~ /^\./ } readdir $dh;
    closedir $dh;

    my $feedback = slurp("$root/docs/FSMGEN_FEEDBACK.md") // '';
    utf8::decode($feedback);
    my ($open_region) = $feedback =~ /<!-- fsmgen_feedback_open:start -->(.*?)<!-- fsmgen_feedback_open:end -->/s;
    my %open_records = map { $_ => 1 } (($open_region // '') =~ /^### ([a-z0-9]+(?:-[a-z0-9]+)*)\b/mg);
    # ADR 0054 (UPSTREAM-CHANNEL-UNIFICATION.2): the ledger indexes the packets. Each open record's fields, so an
    # outgoing record can be matched to exactly one packet and compared with it.
    my %record_fields;
    for my $block (split /^(?=### )/m, $open_region // '') {
        my ($name) = $block =~ /\A### ([a-z0-9]+(?:-[a-z0-9]+)*)\b/ or next;
        my %fields;
        $fields{$1} = $2 while $block =~ /^- (\w+): `([^`]*)`/mg;
        $record_fields{$name} = \%fields;
    }
    my %packets_for_record;

    my %declared_leaves;
    find(
        sub {
            return unless /\.md\z/ && -f $_;
            my $text = slurp($_) // return;
            $declared_leaves{$1} = 1 while $text =~ /ID: `([A-Z0-9][A-Z0-9-]*(?:\.[0-9A-Za-z]+)*)`/g;
        },
        "$root/docs/tasks",
    );

    my $checked = 0;
    for my $id (@bundles) {
        next if $LEGACY{$id};
        $checked++;
        my $dir = "$issues/$id";
        problem("$id: a request id starts with 'sf-'") unless $id =~ /^sf-[a-z0-9]+(?:-[a-z0-9]+)*\z/;

        my $request_text = slurp("$dir/request.json");
        if (!defined $request_text) {
            problem("$id: missing request.json");
            next;
        }
        my $request = eval { JSON::PP->new->utf8->decode($request_text) };
        if (!$request || ref $request ne 'HASH') {
            problem("$id: request.json is not a JSON object");
            next;
        }
        for my $field (@REQUEST_FIELDS) {
            problem("$id: request.json lacks '$field'") unless exists $request->{$field};
        }
        my %known = map { $_ => 1 } @REQUEST_FIELDS;
        problem("$id: request.json has unknown field '$_'") for grep { !$known{$_} } sort keys %$request;
        problem("$id: request.json id '" . ($request->{id} // '') . "' differs from the directory")
            unless ($request->{id} // '') eq $id;
        problem("$id: request.json kind must be bug_report, feature_request or question")
            unless $KIND{ $request->{kind} // '' };
        my $status = $request->{status} // '';
        if (!exists $RANK{$status}) {
            problem("$id: request.json status '$status' is not a lifecycle state");
            next;
        }
        my $past_signoff = $RANK{$status} >= $RANK{signed_off} && !$CLOSED{$status} || $status eq 'integrated';

        my $owner = $request->{owner} // '';
        problem("$id: owner '$owner' is not a leaf any docs/tasks tree declares") unless $declared_leaves{$owner};
        my $record = $request->{feedback_record} // '';
        push @{ $packets_for_record{$record} }, $id;
        if (!$CLOSED{$status}) {
            problem("$id: feedback_record '$record' is not an open record of docs/FSMGEN_FEEDBACK.md")
                unless $open_records{$record};
        }
        if (my $fields = $record_fields{$record}) {
            my $kind = $request->{kind} // '';
            my $record_kind = $fields->{Kind} // '';
            problem("$id: kind '$kind' but record '$record' says '$record_kind'")
                unless $record_kind eq $kind || ($kind eq 'question' && $record_kind eq 'clarity_request');
            my $wanted = $RECORD_STATUS{$status};
            my $record_status = $fields->{Status} // '';
            problem("$id: status '$status' needs record status " . join(' or ', @$wanted)
                . ", but record '$record' says '$record_status'")
                if $wanted && !grep { $_ eq $record_status } @$wanted;
            my $evidence = $fields->{Evidence} // '';
            problem("$id: record '$record' cites '$evidence', not this packet's README")
                unless $evidence eq "docs/fsmgen-issues/$id/README.md";
        }

        # Layout (every state).
        for my $file (qw(README.md commands.sh env.txt SIGNOFF.md expected/expected-notes.md)) {
            problem("$id: missing $file") unless -f "$dir/$file";
        }
        for my $subdir (qw(sources/fsmgen-input observed)) {
            opendir my $sub, "$dir/$subdir" or do { problem("$id: missing $subdir/"); next };
            my @entries = grep { !/^\./ } readdir $sub;
            closedir $sub;
            problem("$id: $subdir/ is empty") unless @entries;
        }
        problem("$id: commands.sh is not executable") if -f "$dir/commands.sh" && !-x "$dir/commands.sh";

        my $readme = slurp("$dir/README.md") // '';
        for my $field (@SUMMARY_FIELDS) {
            problem("$id: README.md lacks the summary field '$field:'")
                unless $readme =~ /^\Q$field\E: \S/m;
        }

        # Portability (every state).
        find(
            {
                no_chdir => 1,
                wanted   => sub {
                    return unless -f $File::Find::name;
                    my $text = slurp($File::Find::name) // return;
                    return if $text =~ /\x00/;
                    (my $rel = $File::Find::name) =~ s{^\Q$dir\E/}{};
                    problem("$id: $rel contains a machine-local absolute path") if $text =~ $ABSOLUTE;
                },
            },
            $dir,
        );

        next unless $past_signoff;

        # Signoff standard (signed_off and later).
        my $signoff = slurp("$dir/SIGNOFF.md") // '';
        my $open_boxes = () = $signoff =~ /^- \[ \]/mg;
        my $ticked = () = $signoff =~ /^- \[x\]/mg;
        problem("$id: SIGNOFF.md has $open_boxes open box(es) at status '$status'") if $open_boxes;
        problem("$id: SIGNOFF.md ticks $ticked of the 9 required boxes") if $ticked < 9;
        my %roles;
        for my $entry (@{ ref $request->{verified_on} eq 'ARRAY' ? $request->{verified_on} : [] }) {
            next unless ref $entry eq 'HASH';
            my $revision = $entry->{fsmgen_revision} // '';
            problem("$id: verified_on revision '$revision' is not a 40-hex commit")
                unless $revision =~ /\A[0-9a-f]{40}\z/;
            $roles{ $entry->{role} // '' } = 1 if length($entry->{result} // '');
        }
        for my $role (qw(pinned upstream)) {
            problem("$id: verified_on lacks a '$role' revision with a result") unless $roles{$role};
        }
        problem("$id: signed_off date missing") unless ($request->{signed_off} // '') =~ /\A\d{4}-\d{2}-\d{2}\z/;
        if ($RANK{$status} >= $RANK{forwarded} && !$CLOSED{$status}) {
            problem("$id: status '$status' requires 'forwarded' {date, channel}")
                unless ref $request->{forwarded} eq 'HASH';
        }
        if ($status eq 'integrated') {
            problem("$id: an integrated request needs RESOLUTION.md") unless -f "$dir/RESOLUTION.md";
        }
    }
    # The other direction: an outgoing record with no packet, or with several, is two channels again.
    for my $name (sort keys %record_fields) {
        my $fields = $record_fields{$name};
        next unless ($fields->{Direction} // '') eq "SpecForge \x{2192} FSMGen" && $OUTGOING_KIND{ $fields->{Kind} // '' };
        my $count = scalar @{ $packets_for_record{$name} // [] };
        problem("record '$name': an outgoing record needs exactly one packet (ADR 0054); $count name it") if $count != 1;
    }
    return ([@ERRORS], $checked);
}

if ($self_test) {
    exit self_test();
}
my ($errors, $checked) = check_root($root);
if (@$errors) {
    print STDERR "$_\n" for @$errors;
    print STDERR "fsmgen-request-bundles: FAILED with " . scalar(@$errors) . " violation(s).\n";
    exit 1;
}
print "fsmgen-request-bundles: $checked request bundle(s) meet docs/fsmgen-issues/PROCESS.md; "
    . scalar(keys %LEGACY) . " legacy bundle(s) exempt.\n";
exit 0;

# The refusal matrix: each case breaks one rule in a copy of the real bundles and must be refused
# with a message naming it; the unmodified copy must pass.
sub self_test {
    my @cases = (
        [baseline => undef, sub { }],
        ['open signoff box' => qr/open box/, sub { edit($_[1] . '/SIGNOFF.md', sub { s/^- \[x\] \*\*MINIMIZED/- [ ] **MINIMIZED/m }) }],
        ['no upstream verification' => qr/lacks a 'upstream' revision/, sub { json_edit($_[1], sub { $_[0]{verified_on} = [ $_[0]{verified_on}[0] ] }) }],
        ['absolute path' => qr/machine-local absolute path/, sub { edit($_[1] . '/env.txt', sub { $_ .= "see /Users/someone/x\n" }) }],
        ['missing summary field' => qr/summary field 'Failure class:'/, sub { edit($_[1] . '/README.md', sub { s/^Failure class:.*\n//m }) }],
        ['unknown status' => qr/not a lifecycle state/, sub { json_edit($_[1], sub { $_[0]{status} = 'ready' }) }],
        ['commands.sh not executable' => qr/not executable/, sub { chmod 0644, $_[1] . '/commands.sh' }],
        ['unknown owner' => qr/not a leaf any docs\/tasks tree declares/, sub { json_edit($_[1], sub { $_[0]{owner} = 'FSMGEN-REQUESTS.99' }) }],
        ['no open feedback record' => qr/not an open record/, sub { my $name = $_[2]; edit($_[0] . '/docs/FSMGEN_FEEDBACK.md', sub { s/^### \Q$name\E\b/### renamed-record/m }) }],
        ['forwarded without details' => qr/requires 'forwarded'/, sub { json_edit($_[1], sub { $_[0]{status} = 'forwarded'; $_[0]{forwarded} = undef }) }],
        ['missing expected notes' => qr/missing expected\/expected-notes\.md/, sub { unlink $_[1] . '/expected/expected-notes.md' }],
        ['unknown kind' => qr/kind must be bug_report, feature_request or question/, sub { json_edit($_[1], sub { $_[0]{kind} = 'clarity_request' }) }],
        ['status disagrees with its record' => qr/needs record status accepted/, sub { json_edit($_[1], sub { $_[0]{status} = 'acknowledged'; $_[0]{forwarded} //= { date => '2026-09-25', channel => 'test' } }) }],
        ['kind disagrees with its record' => qr/record '[^']+' says/, sub { json_edit($_[1], sub { $_[0]{kind} = 'question' }) }],
        ['record cites another packet' => qr/not this packet's README/, sub { my $name = $_[2]; edit($_[0] . '/docs/FSMGEN_FEEDBACK.md', sub { s/(### \Q$name\E\b.*?- Evidence: `)[^`]+/${1}docs\/elsewhere.md/s }) }],
        ['outgoing record without a packet' => qr/record 'no-such-packet': an outgoing record needs exactly one packet/, sub { my $name = $_[2]; edit($_[0] . '/docs/FSMGEN_FEEDBACK.md', sub { s/(### \Q$name\E\b.*?)(?=\n### )/$1\n\n### no-such-packet — a question with no packet\n\n- Direction: `SpecForge \xe2\x86\x92 FSMGen`\n- Kind: `question`\n- Status: `open`\n- Owner: `none`\n- Evidence: `none`\n/s }) }],
    );
    my ($id) = grep { !$LEGACY{$_} && -f "$root/docs/fsmgen-issues/$_/request.json" }
        sort map { (split m{/}, $_)[-1] } glob("$root/docs/fsmgen-issues/*");
    die "self-test: no request bundle to mutate\n" unless defined $id;
    my $record = JSON::PP->new->utf8->decode(slurp("$root/docs/fsmgen-issues/$id/request.json"))->{feedback_record};
    my $failures = 0;
    for my $case (@cases) {
        my ($name, $expect, $mutate) = @$case;
        my $copy = tempdir(CLEANUP => 1);
        copy_tree("$root/docs/fsmgen-issues", "$copy/docs/fsmgen-issues");
        copy_tree("$root/docs/tasks", "$copy/docs/tasks");
        copy("$root/docs/FSMGEN_FEEDBACK.md", "$copy/docs/FSMGEN_FEEDBACK.md") or die "copy: $!\n";
        $mutate->($copy, "$copy/docs/fsmgen-issues/$id", $record);
        my ($errors) = check_root($copy);
        # A refusal counts only when it names the mutated bundle, so a case cannot pass on another's error.
        # A record-level refusal names the record instead (ADR 0054).
        my $ok = defined $expect ? (grep { (/: \Q$id\E: / || /: record '/) && $_ =~ $expect } @$errors) : !@$errors;
        next if $ok;
        $failures++;
        print STDERR "fsmgen-request-bundles self-test: '$name' was not "
            . (defined $expect ? 'refused as expected' : 'accepted') . ": @$errors\n";
    }
    return 1 if $failures;
    print "fsmgen-request-bundles self-test: " . scalar(@cases) . " cases (1 accepted, "
        . (@cases - 1) . " refused) behave as specified.\n";
    return 0;
}

sub edit {
    my ($path, $change) = @_;
    local $_ = slurp($path) // die "self-test: cannot read $path\n";
    $change->();
    open my $fh, '>:raw', $path or die "self-test: cannot write $path: $!\n";
    print {$fh} $_;
}

sub json_edit {
    my ($dir, $change) = @_;
    my $path = "$dir/request.json";
    my $data = JSON::PP->new->utf8->decode(slurp($path));
    $change->($data);
    open my $fh, '>:raw', $path or die "self-test: cannot write $path: $!\n";
    print {$fh} JSON::PP->new->utf8->canonical->pretty->encode($data);
}

sub copy_tree {
    my ($from, $to) = @_;
    find(
        {
            no_chdir => 1,
            wanted   => sub {
                (my $rel = $File::Find::name) =~ s{^\Q$from\E}{};
                if (-d $File::Find::name) {
                    make_path("$to$rel");
                } else {
                    copy($File::Find::name, "$to$rel") or die "self-test: copy $File::Find::name: $!\n";
                    chmod((stat $File::Find::name)[2] & 07777, "$to$rel");
                }
            },
        },
        $from,
    );
}
