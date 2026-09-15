#!/usr/bin/env perl
use strict;
use warnings;

# scripts/check_published_assertions.pl — CLAIM-VERIFICATION-ADOPTION.7.1.
#
# The gate CLAIM-VERIFICATION-ADOPTION.7.0 froze. A digest-bound live document proves only that a region has
# not changed; it never proves that the numbers inside it still re-derive. Ten recorded instances in
# docs/tasks/CLAIM-VERIFICATION-ADOPTION.md went stale under a fully green gate for exactly that reason, and
# five consecutive rounds of prose correction were each invalidated by their own transaction.
#
# This checker binds a published value in a governed region to a producer field and RE-EXECUTES the producer.
# Every element below exists because a specific recorded instance defeated the alternative:
#   * execute-and-compare        — instances 1-6 and 10: values false while every digest control was green.
#   * four outcomes, no fifth    — `derived` / `gated` / `authored` / `dated`. There is deliberately no outcome
#                                  for "a trajectory shows it has held" (instance 8 disproved that licence).
#   * population derived on read — three commits in a row moved their own published population by describing it,
#                                  so a stored surface list is wrong at the moment it lands.
#   * excludes_self              — .11's ownership screen went green for the three surfaces it named BECAUSE it
#                                  named them: a check the act of writing satisfies shares a parent with the
#                                  thing it checks.
#   * cross-surface disagreement — instance 5: two surfaces published different values for one quantity and
#                                  nothing noticed. Costs one comparison and no producer run.
#   * membership as enumeration  — a set claim's size is not its content; instances 4, 5 and .11a are set claims
#                                  published without their enumeration or with a stale one.
# Mechanism claims are deliberately OUT of scope (.10): no checker can decide whether two accounts predict the
# same observation. Records may carry `adjudicated_against` to name the prior ruling a mechanism was checked
# against — shape, not truth.

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

my $LABEL = 'published-assertions';
my $HARD_RECORDS = 512;
my $HARD_BYTES = 262_144;
my $HARD_RECORD_BYTES = 8_192;

my $script_root = abs_path(File::Spec->catdir(dirname(abs_path($0)), '..'));
my $root;
my $contract_rel = 'doctrine/claim_verification/published_assertions.jsonl';
my $mode = 'check';

while (@ARGV) {
    my $arg = shift @ARGV;
    if    ($arg eq '--root')      { $root = shift @ARGV // usage(); }
    elsif ($arg eq '--contract')  { $contract_rel = shift @ARGV // usage(); }
    elsif ($arg eq '--check')     { $mode = 'check'; }
    elsif ($arg eq '--report')    { $mode = 'report'; }
    elsif ($arg eq '--produce')   { $mode = 'produce'; }
    elsif ($arg eq '--self-test') { $mode = 'self-test'; }
    else { usage(); }
}
$root //= $script_root;
$root = abs_path($root) // die "$LABEL: repository root does not exist\n";
run_self_test() if $mode eq 'self-test';

my $result = validate_registry(root => $root, contract_rel => $contract_rel, execute => 1);
if (@{$result->{errors}}) {
    print STDERR "$LABEL: $_\n" for @{$result->{errors}};
    print STDERR "$LABEL: FAILED with " . scalar(@{$result->{errors}}) . " violation(s).\n";
    exit 1;
}
if ($mode eq 'produce') {
    my $json = JSON::PP->new->canonical(1);
    print $json->encode($_), "\n" for @{$result->{unlisted}};
} elsif ($mode eq 'report') {
    print JSON::PP->new->canonical(1)->encode($result->{summary}), "\n";
} else {
    my $s = $result->{summary};
    printf STDERR "%s: %d assertion(s) — %d derived, %d gated, %d authored, %d dated — over %d governed "
        . "region(s) in %d governed and %d exempt claim-annotated file(s), '%s' phase, %d unlisted value(s).\n",
        $LABEL, $s->{assertions}, $s->{outcomes}{derived}, $s->{outcomes}{gated},
        $s->{outcomes}{authored}, $s->{outcomes}{dated}, $s->{governed_regions},
        $s->{governed_files}, $s->{exempt_files}, $s->{phase}, $s->{unlisted};
}
exit 0;

sub usage {
    print STDERR "Usage: scripts/check_published_assertions.pl [--root DIR] [--contract PATH] "
        . "[--check|--report|--produce|--self-test]\n";
    exit 2;
}

# ---------------------------------------------------------------------------------------------------------

sub validate_registry {
    my %args = @_;
    my ($base, $contract_rel, $execute) = @args{qw(root contract_rel execute)};
    my @errors;
    my $contract_abs = absolute($base, $contract_rel);
    my ($meta, $records) = read_bounded_jsonl($contract_abs, "$LABEL contract", \@errors);
    return {errors => \@errors, summary => {}, unlisted => []} if !$meta;

    reject_unknown($meta, "$LABEL registry record", \@errors,
        # LIVE-DOCUMENT-PRESSURE-HEADROOM.22b — the header also declares the pressure band its own
        # bounds are reported against; the band itself is computed centrally in check_live_document_size.pl.
        qw(record_type schema_version phase max_records max_bytes max_record_bytes max_array_items
           max_scalar_bytes milestones));
    exact_scalar($meta->{record_type}, 'registry', "$LABEL registry record_type", \@errors);
    exact_scalar($meta->{schema_version}, 1, "$LABEL registry schema_version", \@errors);
    my $phase = $meta->{phase} // '';
    push @errors, "$LABEL registry phase must be 'inventory' or 'frozen'"
        if $phase ne 'inventory' && $phase ne 'frozen';

    my $tracked = tracked_paths($base, \@errors);

    my (@assertions, %by_id, %by_field, %disposition, %reason, $surface_source);
    for my $rec (@$records) {
        my $type = $rec->{record_type} // '';
        if ($type eq 'source') {
            $surface_source = validate_surface_source($base, $rec, $tracked, \@errors);
            next;
        }
        if ($type eq 'surface_disposition') {
            my ($sid, $d, $why) = validate_disposition($rec, \@errors);
            next if !defined $sid;
            if (exists $disposition{$sid}) {
                push @errors, "$LABEL surface_disposition '$sid' is declared twice";
                next;
            }
            $disposition{$sid} = $d;
            $reason{$sid} = $why;
            next;
        }
        if ($type ne 'assertion') {
            push @errors, "$LABEL record_type '$type' is unknown";
            next;
        }
        my $a = validate_assertion($base, $rec, $tracked, $execute, \@errors);
        next if !$a;
        if ($by_id{$a->{assertion_id}}++) {
            push @errors, "$LABEL assertion_id '$a->{assertion_id}' is duplicated";
        }
        push @assertions, $a;
        if ($a->{outcome} eq 'derived') {
            my $key = join("\0", @{$a->{producer}{argv}}, $a->{producer}{field});
            push @{$by_field{$key}}, $a;
        }
    }

    # Cross-surface disagreement — needs no producer run (instance 5).
    for my $key (sort keys %by_field) {
        my %values = map { ($_->{value} => 1) } @{$by_field{$key}};
        next if keys(%values) <= 1;
        my ($argv_field) = join(' ', split /\0/, $key);
        push @errors, "$LABEL cross-surface disagreement on '$argv_field': "
            . join(' vs ', map { "'$_'" } sort keys %values);
    }

    # The scope is FAIL-CLOSED, which is the property .7.0 element 3 actually wants and a glob list cannot
    # give. Every tracked Markdown file carrying a claim tag is discovered by scanning, then resolved to the
    # live-document surface that owns it; a file whose surface has no declared disposition — or that no
    # surface claims at all — is an ERROR in both phases. So a new claim-annotated file joins the map by
    # itself, and nothing can leave the map without someone writing down which surface it left through.
    my $surfaces = load_surface_map($base, $surface_source, \@errors);
    my ($claim_files, $unclassified) =
        classify_claim_files($base, $tracked, $surfaces, \%disposition, \@errors);
    push @errors, @$unclassified;
    for my $sid (sort keys %disposition) {
        push @errors, "$LABEL surface_disposition '$sid' names no surface in the live-document registry"
            if $surfaces && !$surfaces->{by_id}{$sid};
    }

    my @governed_files = sort grep { ($disposition{$claim_files->{$_}} // '') eq 'governed' }
        keys %$claim_files;
    my @exempt_files = sort grep { ($disposition{$claim_files->{$_}} // '') eq 'exempt' } keys %$claim_files;

    my ($regions, $unlisted) = governed_coverage($base, \@governed_files, \@assertions);
    if (@$unlisted) {
        my $msg = "$LABEL published value '%s' at %s:%d sits in a claim-annotated region that no assertion "
            . "record lists";
        if ($phase eq 'frozen') {
            push @errors, sprintf($msg, $_->{value}, $_->{path}, $_->{line}) for @$unlisted;
        }
    }

    my %outcomes = (derived => 0, gated => 0, authored => 0, dated => 0);
    $outcomes{$_->{outcome}}++ for @assertions;
    return {
        errors => \@errors,
        unlisted => $unlisted,
        summary => {
            assertions => scalar(@assertions),
            outcomes => \%outcomes,
            governed_regions => scalar(@$regions),
            governed_files => scalar(@governed_files),
            exempt_files => scalar(@exempt_files),
            unlisted => scalar(@$unlisted),
            phase => $phase,
        },
    };
}

# The disposition of a SURFACE is authored; the membership of that surface is not. That is the whole
# distinction: a commit cannot move the population by mentioning a producer, because files are discovered,
# and it cannot quietly drop a surface either, because an undeclared one is fatal.
sub validate_disposition {
    my ($rec, $errors) = @_;
    reject_unknown($rec, "$LABEL surface_disposition", $errors,
        qw(record_type schema_version surface_id disposition reason));
    exact_scalar($rec->{schema_version}, 1, "$LABEL surface_disposition schema_version", $errors);
    # Surface ids are the live-document registry's own identifiers, which are snake_case; validating them
    # against this checker's slug shape would reject every real surface, so the producer's form wins.
    my $sid = required_scalar($rec, 'surface_id', "$LABEL surface_disposition", $errors);
    if (defined($sid) && $sid !~ /\A[a-z0-9][a-z0-9_]*\z/) {
        push @$errors, "$LABEL surface_disposition surface_id '$sid' is not a lowercase snake_case identifier";
        return (undef);
    }
    return (undef) if !defined $sid;
    my $d = required_scalar($rec, 'disposition', "$LABEL surface_disposition '$sid'", $errors) // '';
    push @$errors, "$LABEL surface_disposition '$sid' disposition '$d' is not 'governed' or 'exempt'"
        if $d ne 'governed' && $d ne 'exempt';
    my $why = $rec->{reason};
    if ($d eq 'exempt') {
        # An exemption without a stated reason is how a population silently shrinks.
        push @$errors, "$LABEL surface_disposition '$sid' is exempt and must state a reason"
            if !defined($why) || ref($why) || $why !~ /\S/;
    } elsif (defined $why) {
        push @$errors, "$LABEL surface_disposition '$sid' is governed and must not state an exemption reason";
    }
    return ($sid, $d, $why);
}

sub validate_surface_source {
    my ($base, $rec, $tracked, $errors) = @_;
    reject_unknown($rec, "$LABEL source", $errors, qw(record_type schema_version source_id path sha256));
    exact_scalar($rec->{schema_version}, 1, "$LABEL source schema_version", $errors);
    exact_scalar($rec->{source_id}, 'surface_registry', "$LABEL source source_id", $errors);
    my $path = required_scalar($rec, 'path', "$LABEL source", $errors);
    return undef if !defined $path;
    push @$errors, "$LABEL source path '$path' is not repository-tracked" if !$tracked->{$path};
    my $abs = absolute($base, $path);
    if (!-f $abs || -l $abs) {
        push @$errors, "$LABEL source path '$path' is missing or not a regular file";
        return undef;
    }
    my $want = $rec->{sha256};
    if (!defined($want) || ref($want) || $want !~ /\A[0-9a-f]{64}\z/) {
        push @$errors, "$LABEL source sha256 must be 64 lowercase hex characters";
        return undef;
    }
    my $actual = sha256_hex(read_raw($abs, $errors, "$LABEL source"));
    push @$errors, "$LABEL source '$path' is stale: SHA-256 $actual != $want" if $actual ne $want;
    return $path;
}

# Membership is derived FROM THE PRODUCER — the live-document surface registry that already owns which
# surface a path belongs to — never from a description of it (CLAIM_VERIFICATION.md §3 Leg 2).
sub load_surface_map {
    my ($base, $source_path, $errors) = @_;
    if (!defined $source_path) {
        push @$errors, "$LABEL contract must declare one 'surface_registry' source record";
        return undef;
    }
    my $raw = read_raw(absolute($base, $source_path), $errors, "$LABEL surface registry");
    my (%by_id, @rules);
    for my $line (split /\n/, $raw) {
        next if $line !~ /\S/;
        my $rec = eval { JSON::PP->new->decode($line) };
        if ($@ || ref($rec) ne 'HASH') {
            push @$errors, "$LABEL surface registry has a line that is not one JSON object";
            return undef;
        }
        my $sid = $rec->{surface_id};
        next if !defined($sid) || ref($sid);
        $by_id{$sid} = 1;
        for my $target (@{$rec->{targets} || []}) {
            next if ref($target);
            push @rules, [$sid, glob_regex($target)];
        }
    }
    return {by_id => \%by_id, rules => \@rules};
}

sub classify_claim_files {
    my ($base, $tracked, $surfaces, $disposition, $errors) = @_;
    my (%owner, @problems);
    for my $rel (sort keys %$tracked) {
        next if $rel !~ /\.md\z/;
        my $abs = absolute($base, $rel);
        next if !-f $abs || -l $abs;
        my $raw = read_raw($abs, undef, $rel);
        next if !defined($raw) || $raw !~ /\[claim:\s*[a-z0-9][a-z0-9-]*\s*\]/;
        my @owners = $surfaces ? (grep { $rel =~ $_->[1] } @{$surfaces->{rules}}) : ();
        if (!@owners) {
            push @problems, "$LABEL claim-annotated file '$rel' belongs to no live-document surface, so its "
                . "disposition cannot be decided";
            next;
        }
        my $sid = $owners[0][0];
        $owner{$rel} = $sid;
        push @problems, "$LABEL claim-annotated file '$rel' belongs to surface '$sid', which declares no "
            . "disposition"
            if !exists $disposition->{$sid};
    }
    return (\%owner, \@problems);
}

sub validate_assertion {
    my ($base, $rec, $tracked, $execute, $errors) = @_;
    reject_unknown($rec, "$LABEL assertion", $errors,
        qw(record_type schema_version assertion_id path region value outcome producer control decision
           revision membership excludes_self adjudicated_against));
    exact_scalar($rec->{schema_version}, 1, "$LABEL assertion schema_version", $errors);
    my $id = required_slug($rec, 'assertion_id', "$LABEL assertion", $errors);
    return undef if !defined $id;
    my $label = "$LABEL assertion '$id'";

    my $path = required_scalar($rec, 'path', $label, $errors);
    return undef if !defined $path;
    push @$errors, "$label path '$path' is not repository-tracked" if !$tracked->{$path};

    my $value = required_scalar($rec, 'value', $label, $errors);
    my $outcome = required_scalar($rec, 'outcome', $label, $errors) // '';
    push @$errors, "$label outcome '$outcome' is not one of derived/gated/authored/dated"
        if $outcome !~ /\A(?:derived|gated|authored|dated)\z/;

    my $slice = validate_region($base, $path, $rec->{region}, $label, $errors);
    if (defined($slice) && defined($value) && index($slice, $value) < 0) {
        push @$errors, "$label value '$value' does not appear in its own governed region";
    }

    if ($outcome eq 'derived') {
        my $p = $rec->{producer};
        if (ref($p) ne 'HASH') {
            push @$errors, "$label outcome 'derived' requires a producer object";
            return undef;
        }
        reject_unknown($p, "$label producer", $errors, qw(argv field expected_exit));
        my $argv = scalar_array($p->{argv}, "$label producer argv", $errors);
        my $field = required_scalar($p, 'field', "$label producer", $errors);
        push @$errors, "$label producer argv must not be empty" if !@$argv;
        push @$errors, "$label producer '$argv->[0]' must be an interpreter, not a shell"
            if @$argv && $argv->[0] =~ /\A(?:sh|bash|zsh|env)\z/;
        my ($prod_path) = grep { $tracked->{$_} } @$argv;
        push @$errors, "$label producer names no repository-tracked path" if !defined $prod_path;
        if ($execute && @$argv && defined $field) {
            my ($out, $err, $exit) = capture_command($base, $argv);
            my $want = $p->{expected_exit} // 0;
            if ($exit != $want) {
                push @$errors, "$label producer exited $exit, expected $want";
            } else {
                my $decoded = eval { JSON::PP->new->decode($out) };
                if ($@ || ref($decoded) ne 'HASH') {
                    push @$errors, "$label producer stdout is not one JSON object";
                } else {
                    my ($found, $actual) = extract_field($decoded, $field);
                    if (!$found) {
                        push @$errors, "$label producer report has no field '$field'";
                    } elsif (!values_agree($actual, $value)) {
                        push @$errors, "$label is stale: '$field' re-derives to '$actual', published '$value'";
                    }
                }
            }
        }
    } elsif ($outcome eq 'gated') {
        my $c = $rec->{control};
        if (ref($c) ne 'HASH') {
            push @$errors, "$label outcome 'gated' requires a control object";
            return undef;
        }
        reject_unknown($c, "$label control", $errors, qw(argv diagnostic red_case));
        my $argv = scalar_array($c->{argv}, "$label control argv", $errors);
        push @$errors, "$label control argv must not be empty" if !@$argv;
        required_scalar($c, 'diagnostic', "$label control", $errors);
        my $case = $c->{red_case};
        if (ref($case) ne 'HASH') {
            push @$errors, "$label control has no known-bad case bound to an exact source region";
        } else {
            reject_unknown($case, "$label control red_case", $errors,
                qw(path start_line end_line sha256));
            my $cp = required_scalar($case, 'path', "$label control red_case", $errors);
            push @$errors, "$label control red_case path '" . ($cp // '') . "' is not repository-tracked"
                if defined($cp) && !$tracked->{$cp};
            validate_region($base, $cp, {kind => 'line_range_sha256', start_line => $case->{start_line},
                end_line => $case->{end_line}, sha256 => $case->{sha256}},
                "$label control red_case", $errors) if defined $cp;
        }
    } elsif ($outcome eq 'authored') {
        my $d = required_scalar($rec, 'decision', $label, $errors);
        push @$errors, "$label decision '" . ($d // '') . "' is not repository-tracked"
            if defined($d) && !$tracked->{$d};
    } elsif ($outcome eq 'dated') {
        my $rev = required_scalar($rec, 'revision', $label, $errors);
        if (defined $rev) {
            push @$errors, "$label revision '$rev' is not a hexadecimal object name"
                if $rev !~ /\A[0-9a-f]{7,40}\z/;
            if ($execute) {
                my (undef, undef, $exit) = capture_command($base,
                    ['git', 'rev-parse', '--verify', '--quiet', "$rev^{commit}"]);
                push @$errors, "$label revision '$rev' does not resolve in this repository" if $exit != 0;
            }
        }
    }

    if (defined $rec->{membership}) {
        my $m = $rec->{membership};
        if (ref($m) ne 'HASH') {
            push @$errors, "$label membership must be an object";
        } else {
            reject_unknown($m, "$label membership", $errors, qw(enumerator expected));
            my $argv = scalar_array($m->{enumerator}, "$label membership enumerator", $errors);
            my $expected = scalar_array($m->{expected}, "$label membership expected", $errors);
            push @$errors, "$label membership enumerator must not be empty" if !@$argv;
            if ($execute && @$argv) {
                my ($out, undef, $exit) = capture_command($base, $argv);
                if ($exit != 0) {
                    push @$errors, "$label membership enumerator exited $exit";
                } else {
                    my @actual = sort grep { length } map { s/\s+\z//r } split /\n/, $out;
                    my @want = sort @$expected;
                    # The enumeration is compared, never its size (a set claim's size is not its content).
                    if (join("\0", @actual) ne join("\0", @want)) {
                        my %a = map { ($_ => 1) } @actual;
                        my %w = map { ($_ => 1) } @want;
                        my @extra = grep { !$w{$_} } @actual;
                        my @missing = grep { !$a{$_} } @want;
                        push @$errors, "$label membership enumeration drifted"
                            . (@extra ? " (unexpected: " . join(', ', @extra) . ")" : '')
                            . (@missing ? " (absent: " . join(', ', @missing) . ")" : '');
                    }
                    # Self-reference: the surface that publishes the claim must not be counted by it
                    # unless the record declares the exclusion (.11's classifier went green by being written).
                    my $self_in = grep { $_ eq $path } @actual;
                    my $declared = $rec->{excludes_self} ? 1 : 0;
                    if ($self_in && !$declared) {
                        push @$errors, "$label membership counts its own publishing surface '$path' and does "
                            . "not declare excludes_self";
                    } elsif ($declared && $self_in) {
                        push @$errors, "$label declares excludes_self but its enumerator still returns '$path'";
                    }
                }
            }
        }
    } elsif ($rec->{excludes_self}) {
        push @$errors, "$label declares excludes_self without a membership enumeration to exclude from";
    }

    return {assertion_id => $id, path => $path, value => $value, outcome => $outcome,
        region => $rec->{region}, producer => $rec->{producer}};
}

# ---------------------------------------------------------------------------------------------------------
# The governed population is derived on every run and never stored: any commit that annotates a region joins
# the set, so a carried member list is wrong at the moment it lands.

sub governed_coverage {
    my ($base, $files, $assertions) = @_;
    my %listed;
    for my $a (@$assertions) {
        my $start = $a->{region}{start_line} // 0;
        my $end = $a->{region}{end_line} // $start;
        for my $line ($start .. $end) {
            $listed{"$a->{path}:$line:$a->{value}"} = 1;
        }
    }
    my (@regions, @unlisted);
    for my $rel (@$files) {
        my $abs = absolute($base, $rel);
        next if !-f $abs || -l $abs;
        my @lines = split /\n/, read_raw($abs, [], $rel), -1;
        # A [claim: <id>] tag governs the PARAGRAPH it closes, not the line it happens to sit on: authors put
        # the annotation at the end of a block and the values it covers are above it. Keying coverage on the
        # tag's own line would make the map blind to exactly the sentences it exists to watch.
        my %seen_block;
        for my $i (0 .. $#lines) {
            next if $lines[$i] !~ /\[claim:\s*[a-z0-9][a-z0-9-]*\s*\]/;
            my ($from, $to) = ($i, $i);
            $from-- while $from > 0 && $lines[$from - 1] =~ /\S/;
            $to++ while $to < $#lines && $lines[$to + 1] =~ /\S/;
            next if $seen_block{"$from-$to"}++;
            push @regions, {path => $rel, start_line => $from + 1, end_line => $to + 1};
            for my $j ($from .. $to) {
                my $line = $j + 1;
                my $scan = $lines[$j];
                $scan =~ s/\[claim:[^\]]*\]//g;   # the tag itself is not a published value
                $scan =~ s/`[^`]*`//g;             # inline code is a literal, not a published quantity
                # The numeral grammar decides the POPULATION, so its blind spots are values no record can
                # ever be asked for. Two were measured on this repository's own governed regions (.7.1a):
                #   * a comma belongs to a numeral only when it separates exactly three digits, so `1,922`
                #     stays one value while `40, noticed` publishes `40` and not `40,`;
                #   * a numeral closing a compound adjective (`27-case`) or a ratio (`15/15`) is a published
                #     quantity. Only `-` followed by a DIGIT stays excluded, because that is a date or an
                #     identifier fragment (`2026-08-28`, `segment-0013`), never a quantity.
                # The lookbehind is unchanged: a numeral glued to a preceding word, dot, slash or hyphen is
                # part of an identifier (`SHA-256`, `.7.2`, `1.95.0`), not a value.
                for my $token ($scan =~ /(?<![\w.\/-])(\d+(?:,\d{3})*(?:\.\d+)?%?)(?![\w.]|-\d)/g) {
                    next if $listed{"$rel:$line:$token"};
                    push @unlisted, {path => $rel, line => $line, value => $token};
                }
            }
        }
    }
    return (\@regions, \@unlisted);
}

sub extract_field {
    my ($node, $field) = @_;
    for my $part (split /\./, $field) {
        if (ref($node) eq 'HASH') {
            return (0, undef) if !exists $node->{$part};
            $node = $node->{$part};
        } elsif (ref($node) eq 'ARRAY' && $part =~ /\A\d+\z/) {
            return (0, undef) if $part > $#$node;
            $node = $node->[$part];
        } else {
            return (0, undef);
        }
    }
    return (0, undef) if ref($node);
    return (1, defined($node) ? "$node" : '');
}

sub values_agree {
    my ($actual, $published) = @_;
    return 0 if !defined($actual) || !defined($published);
    return 1 if $actual eq $published;
    my ($a, $p) = ($actual, $published);
    s/,//g for $a, $p;
    s/%\z// for $a, $p;
    return 0 if $a !~ /\A-?\d+(?:\.\d+)?\z/ || $p !~ /\A-?\d+(?:\.\d+)?\z/;
    return $a == $p ? 1 : 0;
}

# ---------------------------------------------------------------------------------------------------------

sub validate_region {
    my ($base, $path, $region, $label, $errors) = @_;
    if (ref($region) ne 'HASH') {
        push @$errors, "$label region must be an object";
        return undef;
    }
    reject_unknown($region, "$label region", $errors, qw(kind start_line end_line sha256));
    exact_scalar($region->{kind}, 'line_range_sha256', "$label region kind", $errors);
    my $start = positive_integer($region->{start_line}, "$label region start_line", $errors) // 0;
    my $end = positive_integer($region->{end_line}, "$label region end_line", $errors) // 0;
    push @$errors, "$label region end_line precedes start_line" if $end < $start;
    return undef if !defined($path) || $start < 1 || $end < $start;
    my $abs = absolute($base, $path);
    if (!-f $abs || -l $abs) {
        push @$errors, "$label region path '$path' is missing or not a regular file";
        return undef;
    }
    my @lines = split_lines(read_raw($abs, $errors, $label));
    if ($end > @lines) {
        push @$errors, "$label region $start..$end is outside 1.." . scalar(@lines);
        return undef;
    }
    my $slice = join('', @lines[$start - 1 .. $end - 1]);
    my $expected = $region->{sha256};
    if (!defined($expected) || ref($expected) || $expected !~ /\A[0-9a-f]{64}\z/) {
        push @$errors, "$label region sha256 must be 64 lowercase hex characters";
        return $slice;
    }
    my $actual = sha256_hex($slice);
    push @$errors, "$label exact region is stale: SHA-256 $actual != $expected" if $actual ne $expected;
    return $slice;
}

sub read_bounded_jsonl {
    my ($path, $label, $errors) = @_;
    if (!-f $path || -l $path) {
        push @$errors, "$label is missing or not a regular file";
        return (undef, []);
    }
    my $file_bytes = -s $path;
    push @$errors, "$label exceeds portable hard byte cap $HARD_BYTES" if $file_bytes > $HARD_BYTES;
    my $raw = read_raw($path, $errors, $label);
    my (@records, @lengths);
    my $n = 0;
    for my $line (split /\n/, $raw, -1) {
        $n++;
        next if $line eq '' && $n > 1;
        if ($line eq '') { push @$errors, "$label line $n is blank"; next; }
        push @$errors, "$label line $n exceeds portable raw-record cap $HARD_RECORD_BYTES"
            if length(encode_utf8($line)) > $HARD_RECORD_BYTES;
        my $rec = eval { JSON::PP->new->decode($line) };
        if ($@ || ref($rec) ne 'HASH') { push @$errors, "$label line $n is not one JSON object"; next; }
        push @records, $rec;
        push @lengths, length(encode_utf8($line));
    }
    return (undef, []) if !@records;
    my $meta = shift @records;
    my $max_records = positive_integer($meta->{max_records}, "$label max_records", $errors);
    my $max_bytes = positive_integer($meta->{max_bytes}, "$label max_bytes", $errors);
    my $max_record_bytes = positive_integer($meta->{max_record_bytes}, "$label max_record_bytes", $errors);
    my $max_array = positive_integer($meta->{max_array_items}, "$label max_array_items", $errors);
    my $max_scalar = positive_integer($meta->{max_scalar_bytes}, "$label max_scalar_bytes", $errors);
    push @$errors, "$label max_records exceeds portable hard cap $HARD_RECORDS"
        if defined($max_records) && $max_records > $HARD_RECORDS;
    push @$errors, "$label max_bytes exceeds portable hard cap $HARD_BYTES"
        if defined($max_bytes) && $max_bytes > $HARD_BYTES;
    push @$errors, "$label max_record_bytes exceeds portable hard cap $HARD_RECORD_BYTES"
        if defined($max_record_bytes) && $max_record_bytes > $HARD_RECORD_BYTES;
    push @$errors, "$label max_array_items exceeds portable hard cap 64"
        if defined($max_array) && $max_array > 64;
    push @$errors, "$label max_scalar_bytes exceeds portable hard cap 2048"
        if defined($max_scalar) && $max_scalar > 2_048;
    push @$errors, "$label has more data records than max_records"
        if defined($max_records) && @records > $max_records;
    push @$errors, "$label exceeds declared max_bytes" if defined($max_bytes) && $file_bytes > $max_bytes;
    push @$errors, "$label has a record above max_record_bytes"
        if defined($max_record_bytes) && grep { $_ > $max_record_bytes } @lengths;
    if (defined($max_array) && defined($max_scalar)) {
        validate_value_bounds($meta, "$label registry record", $max_array, $max_scalar, $errors);
        validate_value_bounds($_, $label, $max_array, $max_scalar, $errors) for @records;
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
    } elsif (ref($value) && !JSON::PP::is_bool($value)) {
        push @$errors, "$label contains an unsupported value type";
    } elsif (defined($value) && !ref($value) && length(encode_utf8("$value")) > $max_scalar) {
        push @$errors, "$label scalar exceeds $max_scalar bytes";
    }
}

sub tracked_paths {
    my ($base, $errors) = @_;
    my ($stdout, $stderr, $exit) = capture_command($base, ['git', 'ls-files', '-z']);
    if ($exit != 0) {
        push @$errors, "$LABEL cannot list tracked files: $stderr";
        return {};
    }
    return {map { ($_ => 1) } grep { length } split /\0/, $stdout};
}

sub capture_command {
    my ($base, $argv) = @_;
    my $old = getcwd();
    chdir $base or return ('', "cannot chdir to $base: $!", 127);
    my ($in, $out);
    my $err = gensym;
    my $pid = eval { open3($in, $out, $err, @$argv) };
    if ($@) {
        chdir $old or die "$LABEL: cannot restore cwd: $!\n";
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
            if (!defined($read) || $read == 0) { $select->remove($fh); close $fh; }
            elsif ($stdout_fd{$fd}) { $stdout .= $buffer; }
            else { $stderr .= $buffer; }
        }
    }
    waitpid($pid, 0);
    my $exit = $? == -1 ? 127 : ($? >> 8);
    chdir $old or die "$LABEL: cannot restore cwd: $!\n";
    return ($stdout, $stderr, $exit);
}

sub reject_unknown {
    my ($hash, $label, $errors, @allowed) = @_;
    return if ref($hash) ne 'HASH';
    my %ok = map { ($_ => 1) } @allowed;
    for my $key (sort keys %$hash) {
        push @$errors, "$label has unknown field '$key'" if !$ok{$key};
    }
}

sub required_scalar {
    my ($hash, $key, $label, $errors) = @_;
    my $v = $hash->{$key};
    if (!defined($v) || ref($v) || $v eq '') {
        push @$errors, "$label lacks a nonempty '$key'";
        return undef;
    }
    return "$v";
}

sub required_slug {
    my ($hash, $key, $label, $errors) = @_;
    my $v = required_scalar($hash, $key, $label, $errors);
    return undef if !defined $v;
    if ($v !~ /\A[a-z0-9]+(?:-[a-z0-9]+)*\z/) {
        push @$errors, "$label '$key' must be a lowercase hyphenated slug";
        return undef;
    }
    return $v;
}

sub exact_scalar {
    my ($value, $want, $label, $errors) = @_;
    push @$errors, "$label must be '$want'" if !defined($value) || ref($value) || "$value" ne "$want";
}

sub positive_integer {
    my ($value, $label, $errors) = @_;
    if (!defined($value) || ref($value) || $value !~ /\A[1-9][0-9]*\z/) {
        push @$errors, "$label must be a positive integer";
        return undef;
    }
    return $value + 0;
}

sub scalar_array {
    my ($value, $label, $errors) = @_;
    if (ref($value) ne 'ARRAY') {
        push @$errors, "$label must be an array";
        return [];
    }
    my @out;
    for my $item (@$value) {
        if (!defined($item) || ref($item)) { push @$errors, "$label must contain only scalars"; next; }
        push @out, "$item";
    }
    return \@out;
}

sub glob_regex {
    my ($pattern) = @_;
    my $re = '';
    my @chars = split //, $pattern;
    for (my $i = 0; $i <= $#chars; $i++) {
        my $c = $chars[$i];
        if ($c eq '*' && $i < $#chars && $chars[$i + 1] eq '*') {
            $i++;
            $i++ if $i < $#chars && $chars[$i + 1] eq '/';
            $re .= '.*';
        } elsif ($c eq '*') { $re .= '[^/]*'; }
        elsif ($c eq '?')   { $re .= '[^/]'; }
        else                { $re .= quotemeta($c); }
    }
    return qr/\A$re\z/;
}

sub absolute {
    my ($base, $rel) = @_;
    return File::Spec->file_name_is_absolute($rel) ? $rel : File::Spec->catfile($base, $rel);
}

sub read_raw {
    my ($path, $errors, $label) = @_;
    open my $fh, '<:raw', $path or do {
        push @$errors, "$label cannot be read: $!" if $errors;
        return '';
    };
    local $/;
    my $raw = <$fh>;
    close $fh;
    return defined($raw) ? $raw : '';
}

sub split_lines {
    my ($raw) = @_;
    my @lines;
    while ($raw =~ /\G([^\n]*\n|[^\n]+)/gc) { push @lines, $1; }
    return @lines;
}

sub write_raw {
    my ($path, $content) = @_;
    make_path(dirname($path)) if !-d dirname($path);
    open my $fh, '>:raw', $path or die "$LABEL: cannot write $path: $!\n";
    print $fh $content;
    close $fh;
}

sub write_jsonl {
    my ($path, $records) = @_;
    my $json = JSON::PP->new->canonical(1);
    write_raw($path, join('', map { $json->encode($_) . "\n" } @$records));
}

sub clone { return JSON::PP->new->decode(JSON::PP->new->canonical(1)->encode($_[0])); }

sub command_ok {
    my ($base, $argv) = @_;
    my (undef, $stderr, $exit) = capture_command($base, $argv);
    die "$LABEL self-test: '@$argv' failed: $stderr\n" if $exit != 0;
}

# ---------------------------------------------------------------------------------------------------------
# Self-test. Every fault in CLAIM-VERIFICATION-ADOPTION.7.0's matrix is driven RED on a disposable,
# repository-local fixture before any PASS on the real tree is trusted. A control that has never been observed
# failing on a known-bad input is not a control (CLAIM_VERIFICATION.md §9).

sub run_self_test {
    my $generated = absolute($script_root, 'generated');
    make_path($generated) if !-d $generated;
    my $fixture = File::Spec->catdir($generated, ".published-assertions-self-test.$$");
    die "$LABEL self-test: unsafe fixture path\n"
        if index($fixture, $generated . File::Spec->catfile('')) != 0;
    remove_tree($fixture) if -e $fixture;
    make_path(absolute($fixture, 'doctrine/claim_verification'));
    make_path(absolute($fixture, 'scripts'));

    # A producer that reports a JSON object, exactly as every real producer in this repository does.
    write_raw(absolute($fixture, 'scripts/probe.pl'),
        "#!/usr/bin/env perl\nprint qq({\"counts\":{\"units\":8,\"views\":5},\"unresolved\":0}\\n);\n");
    # An enumerator whose output is a set, not a size.
    write_raw(absolute($fixture, 'scripts/members.pl'),
        "#!/usr/bin/env perl\nprint qq(alpha.md\\nbeta.md\\n);\n");
    write_raw(absolute($fixture, 'scripts/control.pl'),
        "#!/usr/bin/env perl\nprint qq{control PASS\\n};\n");
    write_raw(absolute($fixture, 'decision.md'), "# Decision\n\nFive views are authored.\n");
    write_raw(absolute($fixture, 'alpha.md'), "alpha\n");
    write_raw(absolute($fixture, 'beta.md'), "beta\n");

    # Each governed paragraph is blank-separated so the fixture exercises paragraph-scoped coverage.
    my $doc = "# Fixture\n"
        . "\nThe census reports 8 units. [claim: fixture-claim]\n"
        . "\nUnresolved is 0 because the control fails otherwise. [claim: fixture-claim]\n"
        . "\nThere are 5 views by decision. [claim: fixture-claim]\n"
        . "\nMeasured 2 at the boundary. [claim: fixture-claim]\n"
        . "\nThe governed members are 2 files. [claim: fixture-claim]\n"
        # Punctuation and separators live in one governed paragraph so the numeral grammar is exercised
        # positively: `40,` publishes 40, `1,922` is one value, and `922` after a space is its own.
        . "\nPunctuated 40, plus 1,922 and 922 today. [claim: fixture-claim]\n";
    write_raw(absolute($fixture, 'surface.md'), $doc);
    write_raw(absolute($fixture, 'other.md'), "Elsewhere the census reports 8 units. [claim: fixture-claim]\n");
    # A claim-annotated file on an EXEMPT surface. Its 4321 must never be reported, and the moment its
    # surface loses its declared disposition the run must fail rather than quietly stop scanning it.
    write_raw(absolute($fixture, 'archive.md'), "Sealed on 2026-01-01: 4321 units. [claim: fixture-claim]\n");

    command_ok($fixture, ['git', 'init', '-q']);
    command_ok($fixture, ['git', 'config', 'user.email', 'assertions-self-test@example.invalid']);
    command_ok($fixture, ['git', 'config', 'user.name', 'Assertions Self Test']);
    command_ok($fixture, ['git', 'add', '-A']);
    command_ok($fixture, ['git', 'commit', '-q', '-m', 'FIXTURE.0 — seed']);
    my ($rev) = capture_command($fixture, ['git', 'rev-parse', 'HEAD']);
    chomp $rev;

    my @lines = split_lines(read_raw(absolute($fixture, 'surface.md'), [], 'surface.md'));
    my $region = sub {
        my ($n) = @_;
        return {kind => 'line_range_sha256', start_line => $n, end_line => $n,
            sha256 => sha256_hex($lines[$n - 1])};
    };
    my @control_lines = split_lines(read_raw(absolute($fixture, 'scripts/control.pl'), [], 'control'));

    my $meta = {record_type => 'registry', schema_version => 1, phase => 'frozen',
        max_records => 64, max_bytes => 32_768, max_record_bytes => 4_096,
        max_array_items => 8, max_scalar_bytes => 512};
    # The scope is resolved through the live-document surface registry, so the fixture carries one.
    my $surface_rel = 'doctrine/live_document_size/surfaces.jsonl';
    my $surface_body = join('', map { JSON::PP->new->canonical(1)->encode($_) . "\n" }
        ({surface_id => 'fixture_governed', targets => ['surface.md', 'other.md']},
         {surface_id => 'fixture_exempt', targets => ['archive.md']}));
    write_raw(absolute($fixture, $surface_rel), $surface_body);
    command_ok($fixture, ['git', 'add', '-A']);
    command_ok($fixture, ['git', 'commit', '-q', '-m', 'FIXTURE.1 — surface registry']);
    my $surface_source = sub {
        return {record_type => 'source', schema_version => 1, source_id => 'surface_registry',
            path => $surface_rel,
            sha256 => sha256_hex(read_raw(absolute($fixture, $surface_rel), [], 'surfaces'))};
    };
    my $gov_disp = {record_type => 'surface_disposition', schema_version => 1,
        surface_id => 'fixture_governed', disposition => 'governed'};
    my $exempt_disp = {record_type => 'surface_disposition', schema_version => 1,
        surface_id => 'fixture_exempt', disposition => 'exempt',
        reason => 'Sealed dated evidence capture whose currentness is not asserted.'};

    my $derived = {record_type => 'assertion', schema_version => 1, assertion_id => 'units-derived',
        path => 'surface.md', region => $region->(3), value => '8', outcome => 'derived',
        producer => {argv => ['perl', 'scripts/probe.pl'], field => 'counts.units', expected_exit => 0}};
    my $gated = {record_type => 'assertion', schema_version => 1, assertion_id => 'unresolved-gated',
        path => 'surface.md', region => $region->(5), value => '0', outcome => 'gated',
        control => {argv => ['perl', 'scripts/control.pl'], diagnostic => 'control PASS',
            red_case => {path => 'scripts/control.pl', start_line => 2, end_line => 2,
                sha256 => sha256_hex($control_lines[1])}}};
    my $authored = {record_type => 'assertion', schema_version => 1, assertion_id => 'views-authored',
        path => 'surface.md', region => $region->(7), value => '5', outcome => 'authored',
        decision => 'decision.md'};
    my $dated = {record_type => 'assertion', schema_version => 1, assertion_id => 'boundary-dated',
        path => 'surface.md', region => $region->(9), value => '2', outcome => 'dated', revision => $rev};
    my $membership = {record_type => 'assertion', schema_version => 1, assertion_id => 'members-derived',
        path => 'surface.md', region => $region->(11), value => '2', outcome => 'derived',
        producer => {argv => ['perl', 'scripts/probe.pl'], field => 'counts.views', expected_exit => 0},
        membership => {enumerator => ['perl', 'scripts/members.pl'], expected => ['alpha.md', 'beta.md']}};
    # `members-derived` publishes 2 and binds counts.views (5) — deliberately wrong, fixed below.
    $membership->{producer}{field} = 'counts.units';
    $membership->{value} = '8';
    $membership->{region} = $region->(11);
    # line 6 publishes "2", so bind the honest pair instead: value 2 via a dated outcome plus the enumeration.
    delete $membership->{producer};
    $membership->{outcome} = 'dated';
    $membership->{revision} = $rev;
    $membership->{value} = '2';

    my $contract = 'doctrine/claim_verification/published_assertions.jsonl';
    my $abs_contract = absolute($fixture, $contract);
    my $write = sub {
        my (@records) = @_;
        write_jsonl($abs_contract,
            [clone($meta), $surface_source->(), clone($gov_disp), clone($exempt_disp),
             map { clone($_) } @records]);
    };
    my $write_scope = sub {
        my ($scope, @records) = @_;
        write_jsonl($abs_contract, [clone($meta), @$scope, map { clone($_) } @records]);
    };
    my $run = sub {
        return validate_registry(root => $fixture, contract_rel => $contract, execute => 1);
    };
    my @cases;
    my $case = sub {
        my ($name, $want_fail, $pattern, $setup) = @_;
        $setup->();
        my $res = $run->();
        my $failed = scalar(@{$res->{errors}}) ? 1 : 0;
        my $ok = $failed == $want_fail;
        $ok &&= (grep { $_ =~ $pattern } @{$res->{errors}}) ? 1 : 0 if $want_fail && $pattern;
        push @cases, [$name, $ok, $res->{errors}];
    };

    my @o0 = split_lines(read_raw(absolute($fixture, 'other.md'), [], 'other'));
    my $other = {record_type => 'assertion', schema_version => 1, assertion_id => 'units-derived-other',
        path => 'other.md', value => '8', outcome => 'derived',
        region => {kind => 'line_range_sha256', start_line => 1, end_line => 1,
            sha256 => sha256_hex($o0[0])},
        producer => {argv => ['perl', 'scripts/probe.pl'], field => 'counts.units', expected_exit => 0}};

    # Line 13 is the punctuation/separator paragraph. Before .7.1a the grammar consumed any comma, so the
    # token was `40,` and the honest record below was reported unlisted; the three records are the positive
    # side of that repair and the RED case that follows pins the negative side.
    my $punct_comma = {record_type => 'assertion', schema_version => 1, assertion_id => 'punct-comma-dated',
        path => 'surface.md', region => $region->(13), value => '40', outcome => 'dated', revision => $rev};
    my $punct_thousands = {record_type => 'assertion', schema_version => 1,
        assertion_id => 'punct-thousands-dated', path => 'surface.md', region => $region->(13),
        value => '1,922', outcome => 'dated', revision => $rev};
    my $punct_bare = {record_type => 'assertion', schema_version => 1, assertion_id => 'punct-bare-dated',
        path => 'surface.md', region => $region->(13), value => '922', outcome => 'dated', revision => $rev};

    my @base = ($derived, $gated, $authored, $dated, $membership, $other,
        $punct_comma, $punct_thousands, $punct_bare);

    $case->('positive: all four outcomes plus a membership enumeration', 0, undef, sub { $write->(@base); });

    # 1 — a drifted value.
    $case->('RED: published value drifted from its producer field', 1, qr/is stale: 'counts\.units'/, sub {
        my $bad = clone($derived);
        write_raw(absolute($fixture, 'surface.md'),
            join('', @lines[0 .. 1], "The census reports 9 units. [claim: fixture-claim]\n", @lines[3 .. $#lines]));
        my @now = split_lines(read_raw(absolute($fixture, 'surface.md'), [], 's'));
        $bad->{value} = '9';
        $bad->{region} = {kind => 'line_range_sha256', start_line => 3, end_line => 3,
            sha256 => sha256_hex($now[2])};
        $write->($bad, $gated, $authored, $dated, $membership, $other);
    });
    write_raw(absolute($fixture, 'surface.md'), $doc);

    # 2 — a value bound to the wrong field.
    $case->('RED: value bound to the wrong producer field', 1, qr/is stale: 'counts\.views'/, sub {
        my $bad = clone($derived);
        $bad->{producer}{field} = 'counts.views';
        $write->($bad, $gated, $authored, $dated, $membership, $other);
    });

    # 3 — a published value in a governed region that no record lists.
    $case->('RED: unlisted published value in a claim-annotated region', 1, qr/no assertion record lists/, sub {
        $write->($gated, $authored, $dated, $membership, $other);
    });

    # 4 — a gated record whose control has no known-bad case.
    $case->('RED: gated record with no known-bad control region', 1, qr/no known-bad case/, sub {
        my $bad = clone($gated);
        delete $bad->{control}{red_case};
        $write->($derived, $bad, $authored, $dated, $membership, $other);
    });

    # 5 — a self-referential membership that does not declare the exclusion.
    $case->('RED: membership counts its own publishing surface without excludes_self', 1,
        qr/counts its own publishing surface/, sub {
        write_raw(absolute($fixture, 'scripts/members.pl'),
            "#!/usr/bin/env perl\nprint qq(alpha.md\\nbeta.md\\nsurface.md\\n);\n");
        my $bad = clone($membership);
        $bad->{membership}{expected} = ['alpha.md', 'beta.md', 'surface.md'];
        $write->($derived, $gated, $authored, $dated, $bad, $other);
    });
    write_raw(absolute($fixture, 'scripts/members.pl'),
        "#!/usr/bin/env perl\nprint qq(alpha.md\\nbeta.md\\n);\n");

    # 6 — two records disagreeing on one producer + field. This leg runs no producer at all: it is the
    # cheap signal instance 5 identified, where two surfaces published different values for one quantity.
    $case->('RED: two surfaces disagree on one producer field', 1, qr/cross-surface disagreement/, sub {
        write_raw(absolute($fixture, 'other.md'),
            "Elsewhere the census reports 7 units. [claim: fixture-claim]\n");
        my @o2 = split_lines(read_raw(absolute($fixture, 'other.md'), [], 'other'));
        my $bad = clone($other);
        $bad->{value} = '7';
        $bad->{region}{sha256} = sha256_hex($o2[0]);
        $write->($derived, $bad, $gated, $authored, $dated, $membership);
    });
    write_raw(absolute($fixture, 'other.md'), "Elsewhere the census reports 8 units. [claim: fixture-claim]\n");

    # 7 — a membership enumeration that drifted while its size held.
    $case->('RED: membership enumeration drifted while its size held', 1, qr/membership enumeration drifted/,
        sub {
        write_raw(absolute($fixture, 'scripts/members.pl'),
            "#!/usr/bin/env perl\nprint qq(alpha.md\\ngamma.md\\n);\n");
        $write->($derived, $gated, $authored, $dated, $membership, $other);
    });
    write_raw(absolute($fixture, 'scripts/members.pl'),
        "#!/usr/bin/env perl\nprint qq(alpha.md\\nbeta.md\\n);\n");

    # 8 — a stale exact region.
    $case->('RED: exact region digest is stale', 1, qr/exact region is stale/, sub {
        my $bad = clone($derived);
        $bad->{region}{sha256} = sha256_hex('not the region');
        $write->($bad, $gated, $authored, $dated, $membership, $other);
    });

    # 9 — an untracked producer.
    $case->('RED: producer names no tracked path', 1, qr/names no repository-tracked path/, sub {
        my $bad = clone($derived);
        $bad->{producer}{argv} = ['perl', '-e', 'print q({"counts":{"units":8}})'];
        $write->($bad, $gated, $authored, $dated, $membership, $other);
    });

    # 10 — a fifth outcome is refused: there is no slot for "a trajectory shows it has held".
    $case->('RED: an outcome outside the four is refused', 1, qr/is not one of derived\/gated\/authored\/dated/,
        sub {
        my $bad = clone($derived);
        $bad->{outcome} = 'trajectory';
        delete $bad->{producer};
        $write->($bad, $gated, $authored, $dated, $membership, $other);
    });

    # 11 — excludes_self without a membership to exclude from.
    $case->('RED: excludes_self declared with no membership enumeration', 1, qr/without a membership/, sub {
        my $bad = clone($authored);
        $bad->{excludes_self} = JSON::PP::true;
        $write->($derived, $gated, $bad, $dated, $membership, $other);
    });

    # 12 — a duplicated assertion id.
    $case->('RED: duplicated assertion_id', 1, qr/is duplicated/, sub {
        my $dup = clone($gated);
        $dup->{assertion_id} = 'units-derived';
        $write->($derived, $gated, $authored, $dated, $membership, $other, $dup);
    });

    # 13 — a value that does not appear in its own governed region.
    $case->('RED: value absent from its own governed region', 1, qr/does not appear in its own governed region/,
        sub {
        my $bad = clone($dated);
        $bad->{value} = '42';
        $write->($derived, $gated, $authored, $bad, $membership, $other);
    });

    # 14 — bounds are enforced.
    $case->('RED: record count exceeds the declared bound', 1, qr/more data records than max_records/, sub {
        my $small = clone($meta);
        $small->{max_records} = 1;
        write_jsonl($abs_contract, [$small, map { clone($_) } @base]);
    });

    # 15 — a numeral closing a compound adjective is a published quantity, not an identifier fragment.
    # Before .7.1a the lookahead excluded every `-`, so `27-case` was invisible and no record could be
    # asked for it: a blind spot in the population is worse than an unlisted value, because it is silent.
    $case->('RED: compound-adjective value is visible to the coverage grammar', 1,
        qr/published value '27' at surface\.md:\d+ sits/, sub {
        write_raw(absolute($fixture, 'surface.md'),
            $doc . "\nThe 27-case self-test covers it. [claim: fixture-claim]\n");
        $write->(@base);
    });
    write_raw(absolute($fixture, 'surface.md'), $doc);

    # 16 — a ratio-form pass count is a published quantity. The same excluded lookahead hid `15/15`.
    $case->('RED: ratio-form value is visible to the coverage grammar', 1,
        qr/published value '15' at surface\.md:\d+ sits/, sub {
        write_raw(absolute($fixture, 'surface.md'),
            $doc . "\nControls pass 15/15 today. [claim: fixture-claim]\n");
        $write->(@base);
    });
    write_raw(absolute($fixture, 'surface.md'), $doc);

    # 17 — the mirror of the comma repair: a record that absorbed the sentence comma into its value no
    # longer covers the value actually published, and the gate says so instead of accepting the near-miss.
    $case->('RED: a record whose value absorbed sentence punctuation covers nothing', 1,
        qr/published value '40' at surface\.md:\d+ sits/, sub {
        my $bad = clone($punct_comma);
        $bad->{value} = '40,';
        $write->($derived, $gated, $authored, $dated, $membership, $other,
            $bad, $punct_thousands, $punct_bare);
    });

    # 18 — a claim-annotated file whose surface declares no disposition. This is the property a glob list
    # could not give: the map cannot shrink by omission, because omission is the error.
    $case->('RED: claim-annotated file on a surface with no declared disposition', 1,
        qr/declares no disposition/, sub {
        $write_scope->([$surface_source->(), clone($gov_disp)], @base);
    });

    # 19 — a claim-annotated file that no surface claims at all. Its disposition cannot be decided, so it
    # is an error rather than a silent pass; a file outside every surface is exactly what nobody notices.
    $case->('RED: claim-annotated file owned by no live-document surface', 1,
        qr/belongs to no live-document surface/, sub {
        write_raw(absolute($fixture, 'orphan.md'), "An orphan publishes 77 units. [claim: fixture-claim]\n");
        command_ok($fixture, ['git', 'add', 'orphan.md']);
        $write->(@base);
    });
    unlink absolute($fixture, 'orphan.md');
    command_ok($fixture, ['git', 'rm', '-q', '--cached', 'orphan.md']);

    # 20 — an exemption with no stated reason. An exemption is how a population legitimately shrinks, so
    # the reason is the only thing standing between that and shrinking it by preference.
    $case->('RED: exempt surface without a stated reason', 1, qr/must state a reason/, sub {
        my $bad = clone($exempt_disp);
        delete $bad->{reason};
        $write_scope->([$surface_source->(), clone($gov_disp), $bad], @base);
    });

    # 21 — a disposition naming a surface the live-document registry does not have. Without this a typo
    # exempts nothing and governs nothing, and the run still passes.
    $case->('RED: disposition names an unknown surface', 1, qr/names no surface in the live-document/, sub {
        my $bad = clone($exempt_disp);
        $bad->{surface_id} = 'fixture_typo';
        $write_scope->([$surface_source->(), clone($gov_disp), clone($exempt_disp), $bad], @base);
    });

    # 22 — the surface registry moved without the contract being re-derived. The scope is derived FROM that
    # registry, so its identity is a dependency of every classification this checker makes.
    $case->('RED: surface registry moved under a stale source digest', 1, qr/source .* is stale/, sub {
        my $stale = $surface_source->();
        $stale->{sha256} = sha256_hex('not the surface registry');
        $write_scope->([$stale, clone($gov_disp), clone($exempt_disp)], @base);
    });

    # 23 — the exemption is load-bearing, not incidental. The positive cases pass with archive.md's 4321
    # unreported; flipping that one surface to `governed` must make the very same value fatal. Without this
    # the green above would be equally consistent with the file never having been discovered at all, which
    # is evidence consistent with both hypotheses and therefore no evidence (CLAIM_VERIFICATION.md §3 Leg 2).
    $case->('RED: a value under an exemption is reported the moment that surface is governed', 1,
        qr/published value '4321' at archive\.md:1 sits/, sub {
        my $flip = clone($exempt_disp);
        $flip->{disposition} = 'governed';
        delete $flip->{reason};
        $write_scope->([$surface_source->(), clone($gov_disp), $flip], @base);
    });

    $case->('positive: restored contract is green again', 0, undef, sub { $write->(@base); });

    remove_tree($fixture);
    my $pass = grep { $_->[1] } @cases;
    for my $c (@cases) {
        next if $c->[1];
        print STDERR "$LABEL self-test FAILED case '$c->[0]'\n";
        print STDERR "  saw: $_\n" for @{$c->[2]};
    }
    if ($pass != scalar(@cases)) {
        printf STDERR "%s: self-test %d/%d cases pass.\n", $LABEL, $pass, scalar(@cases);
        exit 1;
    }
    printf STDERR "%s: self-test %d/%d positive, drift, wrong-field, unlisted, control, self-reference, "
        . "disagreement, enumeration, region, producer, outcome, duplicate, bound, compound-adjective, "
        . "ratio, absorbed-punctuation, undeclared-surface, orphan-file, reasonless-exemption, "
        . "unknown-surface, stale-surface-source, and load-bearing-exemption cases pass.\n",
        $LABEL, $pass, scalar(@cases);
    exit 0;
}
