#!/usr/bin/env perl
# scripts/check_section_anchors.pl — SECTION-ANCHORS doctrine (structural archetype).
#
# LIVE-DOCUMENT-PRESSURE-HEADROOM.4f. A qualified cross-document section reference — the
# repository's `<path>.md` §`<section>` form — must resolve to a heading that actually exists in
# the file it names. Nothing checked this before, and the gap is not hypothetical: `.4e`
# partitioned a research record at a section seam, `scripts/check_doctrines.sh` reported ALL 12
# doctrines PASS, and the commit still moved the repository from 20 resolving / 0 unresolved
# anchors to 13 / 14. A content-preserving move is not a route-preserving move, and only a
# mechanical check can tell the difference.
#
# Why the target end and not the source end: seven of those fourteen links sit inside sealed
# `archive_terminal` rolling-ledger segments, which the COMMIT.md rollover doctrine forbids
# editing. The source end of a link from sealed history can never be repaired, so a partition must
# keep the cited headings resolvable — as redirects, if the content moved.
#
# HONEST LIMIT (DOCTRINE_ENFORCEMENT.md §9): this proves qualified references only. A bare
# §`<section>` with no path is resolved from prose context, which is not mechanically decidable,
# and is deliberately not counted rather than guessed at. Anchor TEXT is compared after stripping
# backticks and collapsing whitespace, because a citation writes `§`.5.ii measurement`` while the
# heading writes ``## `.5.ii` measurement (…)``; the citation must be a substring of the heading.
#
# Check-script contract (DOCTRINE_ENFORCEMENT.md §4): exit code is the verdict, breaches explain
# themselves on stderr, deterministic, read-only, repo-root resolved from this script's location.

use strict;
use warnings;
use utf8;

use Cwd qw(abs_path);
use Encode qw(decode_utf8);
use File::Spec;
use FindBin qw($Bin);

binmode STDOUT, ':encoding(UTF-8)';
binmode STDERR, ':encoding(UTF-8)';

my $root = abs_path(File::Spec->catdir($Bin, '..'));
my $mode = 'check';
while (@ARGV) {
    my $arg = shift @ARGV;
    if    ($arg eq '--check')     { $mode = 'check' }
    elsif ($arg eq '--report')    { $mode = 'report' }
    elsif ($arg eq '--self-test') { $mode = 'self-test' }
    elsif ($arg eq '--root') {
        die "section-anchors: --root requires a path\n" if !@ARGV;
        $root = abs_path(shift @ARGV) // die "section-anchors: --root does not resolve\n";
    } else {
        die "Usage: scripts/check_section_anchors.pl [--check|--report|--self-test] [--root DIR]\n";
    }
}

sub slurp {
    my ($abs) = @_;
    open my $fh, '<:raw', $abs or return undef;
    local $/;
    my $bytes = <$fh>;
    close $fh;
    return decode_utf8($bytes // '', Encode::FB_DEFAULT);
}

# Normalize a heading or a citation to one comparable form: no backticks, collapsed whitespace.
sub norm {
    my ($s) = @_;
    $s //= '';
    $s =~ s/`//g;
    $s =~ s/\s+/ /g;
    $s =~ s/\A\s+|\s+\z//g;
    return $s;
}

sub tracked_markdown {
    my ($r) = @_;
    open my $fh, '-|', 'git', '-C', $r, 'ls-files', '-z', '--cached', '--', '*.md'
        or die "section-anchors: cannot run git ls-files\n";
    local $/ = "\0";
    my @out;
    while (my $line = <$fh>) {
        chomp $line;
        push @out, decode_utf8($line, Encode::FB_DEFAULT) if length $line;
    }
    close $fh;
    return sort @out;
}

my %headings_for;
sub headings {
    my ($rel) = @_;
    return $headings_for{$rel} if exists $headings_for{$rel};
    my $text = slurp(File::Spec->catfile($root, $rel));
    if (!defined $text) { return $headings_for{$rel} = undef }
    my @h;
    for my $line (split /\n/, $text, -1) {
        push @h, norm($1) if $line =~ /\A\#{1,6}\s+(.*)\z/;
    }
    return $headings_for{$rel} = \@h;
}

my @violations;
my $resolved = 0;
my $bare = 0;

for my $rel (tracked_markdown($root)) {
    my $text = slurp(File::Spec->catfile($root, $rel));
    next if !defined $text;
    $bare += () = $text =~ /(?<!`)\x{00A7}`[^`]+`/g;
    while ($text =~ /`([\w.\/-]+\.md)`\s*\x{00A7}`([^`]+)`/g) {
        my ($target, $section) = ($1, norm($2));
        my $h = headings($target);
        if (!defined $h) {
            push @violations, "$rel cites '$target' \x{00A7}'$section' but that file is not readable";
            next;
        }
        if (grep { index($_, $section) >= 0 } @$h) { $resolved++; next }
        push @violations,
            "$rel cites '$target' \x{00A7}'$section' but no heading in that file contains it"
            . " (a moved section must keep a redirect heading at the cited target)";
    }
}

if ($mode eq 'self-test') {
    # RED control: a citation whose section does not exist must be refused. Proven against a
    # synthetic pair held in memory rather than by writing to the tree, so the check stays read-only.
    my @probe_headings = (norm('## `.5.ii` measurement (`2026-06-24`) — the gate is PER-MEMBER'));
    my $good = grep { index($_, norm('.5.ii measurement')) >= 0 } @probe_headings;
    my $bad  = grep { index($_, norm('.5.ii LANDED')) >= 0 } @probe_headings;
    if ($good != 1 || $bad != 0) {
        print STDERR "section-anchors: SELF-TEST FAILED (good=$good bad=$bad)\n";
        exit 1;
    }
    print "section-anchors: self-test passed (backtick/whitespace-normalized containment accepts the"
        . " cited heading and refuses a section that is absent).\n";
    exit 0;
}

if (@violations) {
    print STDERR "section-anchors: $_\n" for @violations;
    printf STDERR "section-anchors: FAILED with %d unresolved section reference(s).\n",
        scalar @violations;
    exit 1;
}

printf "section-anchors: %d qualified section reference(s) resolve to a real heading"
    . " (%d bare \x{00A7} reference(s) are context-relative and not mechanically checkable).\n",
    $resolved, $bare;
exit 0;
