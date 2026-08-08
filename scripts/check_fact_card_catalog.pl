#!/usr/bin/env perl
use strict;
use warnings;
use utf8;

use Cwd qw(abs_path);
use Encode qw(decode_utf8 encode_utf8);
use Fcntl qw(O_CREAT O_EXCL O_WRONLY);
use File::Spec;
use FindBin qw($Bin);

binmode STDOUT, ':encoding(UTF-8)';
binmode STDERR, ':encoding(UTF-8)';

my $MAX_CARDS = 160;
my $MAX_INDEX_BYTES = 32_768;
my $MAX_ROW_BYTES = 320;
my $MAX_ID_BYTES = 64;
my $MAX_SOURCE_TITLE_BYTES = 1_024;
my $MAX_TITLE_CELL_BYTES = 112;
my $MAX_ANSWERS = 64;
my $MAX_ANSWER_BYTES = 2_048;
my %VALID_STATUS = map { $_ => 1 } qw(current superseded deprecated);

my $root = abs_path(File::Spec->catdir($Bin, '..'));
my $mode = 'check';
while (@ARGV) {
    my $arg = shift @ARGV;
    if ($arg eq '--check') {
        $mode = 'check';
    } elsif ($arg eq '--write') {
        $mode = 'write';
    } elsif ($arg eq '--self-test') {
        $mode = 'self-test';
    } elsif ($arg eq '--root') {
        die "fact-card-catalog: --root requires a path\n" if !@ARGV;
        $root = abs_path(shift @ARGV);
        die "fact-card-catalog: root does not exist\n" if !defined $root;
    } else {
        die "Usage: $0 [--check|--write|--self-test] [--root PROJECT_ROOT]\n";
    }
}

if ($mode eq 'self-test') {
    run_self_test();
    print "fact-card-catalog: 9/9 parser and bound tests pass.\n";
    exit 0;
}

my $ok = eval {
    my @cards = collect_cards($root);
    my $expected = render_catalog(\@cards);
    my $index_path = File::Spec->catfile($root, 'docs', 'knowledge', 'INDEX.md');

    if ($mode eq 'write') {
        my $current = -f $index_path ? slurp_utf8($index_path) : '';
        atomic_write($index_path, $expected) if $current ne $expected;
        print "fact-card-catalog: wrote " . scalar(@cards)
          . " concise fact-card routes to docs/knowledge/INDEX.md.\n";
    } else {
        die "docs/knowledge/INDEX.md is missing; run perl scripts/check_fact_card_catalog.pl --write\n"
          if !-f $index_path;
        my $current = slurp_utf8($index_path);
        die "derived catalog differs; run perl scripts/check_fact_card_catalog.pl --write\n"
          if $current ne $expected;
        print "fact-card-catalog: " . scalar(@cards)
          . " fact cards are linked exactly once in the bounded derived catalog.\n";
    }
    1;
};
if (!$ok) {
    my $error = $@ || 'unknown failure';
    $error =~ s/\s+\z//;
    print STDERR "fact-card-catalog: $error\n";
    exit 1;
}

sub collect_cards {
    my ($project_root) = @_;
    my $card_dir = File::Spec->catdir($project_root, 'docs', 'knowledge');
    die "fact-card-catalog: fact-card directory is missing: docs/knowledge\n" if !-d $card_dir;
    die "fact-card-catalog: collection guide is missing: docs/knowledge/README.md\n"
      if !-f File::Spec->catfile($card_dir, 'README.md');

    opendir(my $dh, $card_dir) or die "cannot open docs/knowledge: $!\n";
    my @names = sort grep {
        /\.md\z/ && $_ ne 'README.md' && $_ ne 'INDEX.md'
    } readdir($dh);
    closedir($dh) or die "cannot close docs/knowledge: $!\n";
    die "catalog has no fact cards\n" if !@names;
    die "catalog has " . scalar(@names) . " fact cards; limit is $MAX_CARDS\n"
      if @names > $MAX_CARDS;

    my @cards;
    for my $name (@names) {
        die "unsafe fact-card filename '$name'\n"
          if $name !~ /\A[a-z0-9][a-z0-9-]*\.md\z/;
        my $path = File::Spec->catfile($card_dir, $name);
        push @cards, parse_card($name, slurp_utf8($path));
    }
    return @cards;
}

sub parse_card {
    my ($name, $text) = @_;
    my ($front_matter) = $text =~ /\A---\r?\n(.*?)\r?\n---(?:\r?\n|\z)/s;
    die "$name lacks leading YAML front matter\n" if !defined $front_matter;

    my %scalar;
    my @answers;
    my %seen;
    my $current_key = '';
    for my $line (split /\r?\n/, $front_matter) {
        if ($line =~ /^[ \t]+-[ \t]+(.+)\z/) {
            my $value = clean_scalar($1);
            if ($current_key eq 'answers') {
                die "$name has an empty answer\n" if $value eq '';
                push @answers, $value;
            }
            next;
        }
        next if $line =~ /^\s*\z/;
        my ($key, $rest) = $line =~ /\A([A-Za-z_][A-Za-z0-9_]*):[ \t]*(.*)\z/;
        die "$name has unsupported front-matter syntax: $line\n" if !defined $key;
        die "$name repeats front-matter key '$key'\n" if $seen{$key}++;
        $current_key = $key;
        if ($rest =~ /^\[(.*)\]\z/) {
            if ($key eq 'answers') {
                my $inner = $1;
                push @answers, map { clean_scalar($_) } split /,/, $inner, -1;
            }
            $current_key = '';
        } elsif ($rest ne '') {
            $scalar{$key} = clean_scalar($rest);
            $current_key = '';
        }
    }

    my ($expected_id) = $name =~ /\A(.+)\.md\z/;
    my $id = $scalar{id} // '';
    die "$name lacks non-empty id\n" if $id eq '';
    die "$name id '$id' does not match filename '$expected_id'\n" if $id ne $expected_id;
    die "$name id exceeds $MAX_ID_BYTES UTF-8 bytes\n"
      if length(encode_utf8($id)) > $MAX_ID_BYTES;

    my $title = $scalar{title} // '';
    die "$name lacks non-empty title\n" if $title eq '';
    die "$name title exceeds $MAX_SOURCE_TITLE_BYTES UTF-8 bytes\n"
      if length(encode_utf8($title)) > $MAX_SOURCE_TITLE_BYTES;

    my $date = $scalar{date} // '';
    die "$name date must be YYYY-MM-DD\n"
      if $date !~ /\A\d{4}-(?:0[1-9]|1[0-2])-(?:0[1-9]|[12]\d|3[01])\z/;
    my $status = $scalar{status} // 'current';
    die "$name has unknown status '$status'\n" if !$VALID_STATUS{$status};

    @answers = grep { $_ ne '' } @answers;
    die "$name lacks a non-empty answers list\n" if !@answers;
    die "$name has " . scalar(@answers) . " answers; limit is $MAX_ANSWERS\n"
      if @answers > $MAX_ANSWERS;
    for my $answer (@answers) {
        die "$name answer exceeds $MAX_ANSWER_BYTES UTF-8 bytes\n"
          if length(encode_utf8($answer)) > $MAX_ANSWER_BYTES;
    }
    die "$name lacks evidence or reverify\n"
      if ($scalar{evidence} // '') eq '' && ($scalar{reverify} // '') eq '';

    return {
        id => $id,
        title => $title,
        date => $date,
        status => $status,
        name => $name,
    };
}

sub clean_scalar {
    my ($value) = @_;
    $value =~ s/^[ \t]+//;
    $value =~ s/[ \t]+\z//;
    if ($value =~ /\A"(.*)"\z/s || $value =~ /\A'(.*)'\z/s) {
        $value = $1;
    }
    return $value;
}

sub render_catalog {
    my ($cards) = @_;
    my @lines = (
        '# Knowledge fact-card catalog',
        '',
        '> **AUTO-GENERATED — DO NOT EDIT.** Regenerate with',
        '> `perl scripts/check_fact_card_catalog.pl --write`.',
        '',
        'This bounded human route lists every immediate fact card once. Search by question in the',
        '[generated Knowledge Map](../../KNOWLEDGE_MAP.md); browse durable rationale in the',
        '[decision index](../decisions/INDEX.md). Full titles, questions, evidence, and prose remain',
        'canonical in the linked cards.',
        '',
        'Collection guidance: [authoring and lifecycle README](README.md).',
        '',
        '| Fact | Established | Status | Title |',
        '| --- | --- | --- | --- |',
    );
    for my $card (@$cards) {
        my $title = compact_title($card->{title});
        my $row = '| [' . $card->{id} . '](' . $card->{name} . ') | '
          . $card->{date} . ' | `' . $card->{status} . '` | ' . $title . ' |';
        die "$card->{name} catalog row exceeds $MAX_ROW_BYTES UTF-8 bytes\n"
          if length(encode_utf8($row)) > $MAX_ROW_BYTES;
        push @lines, $row;
    }
    my $index = join("\n", @lines) . "\n";
    die "catalog exceeds $MAX_INDEX_BYTES UTF-8 bytes\n"
      if length(encode_utf8($index)) > $MAX_INDEX_BYTES;
    return $index;
}

sub compact_title {
    my ($title) = @_;
    $title =~ s/\\/\\\\/g;
    $title =~ s/\|/\\|/g;
    return $title if length(encode_utf8($title)) <= $MAX_TITLE_CELL_BYTES;

    my $limit = $MAX_TITLE_CELL_BYTES - length(encode_utf8('…'));
    my $short = '';
    for my $character (split //, $title) {
        last if length(encode_utf8($short . $character)) > $limit;
        $short .= $character;
    }
    $short =~ s/\s+\z//;
    return $short . '…';
}

sub slurp_utf8 {
    my ($path) = @_;
    open(my $fh, '<:raw', $path) or die "cannot read '$path': $!\n";
    local $/;
    my $bytes = <$fh>;
    close($fh) or die "cannot close '$path': $!\n";
    return decode_utf8($bytes, 1);
}

sub atomic_write {
    my ($path, $text) = @_;
    my $temporary = $path . '.fact-catalog.' . $$;
    sysopen(my $fh, $temporary, O_WRONLY | O_CREAT | O_EXCL, 0644)
      or die "cannot create same-directory temporary '$temporary': $!\n";
    binmode $fh, ':raw';
    print {$fh} encode_utf8($text) or die "cannot write '$temporary': $!\n";
    close($fh) or die "cannot close '$temporary': $!\n";
    rename($temporary, $path) or die "cannot replace '$path': $!\n";
}

sub run_self_test {
    my $normal = <<'CARD';
---
id: alpha
title: Alpha title
answers:
  - "where is alpha"
date: 2026-08-08
evidence: docs/example.md
---
CARD
    my $inline = <<'CARD';
---
id: beta
title: Beta | title
answers: [what is beta]
date: 2026-08-08
status: deprecated
reverify: test -f beta
---
CARD
    my $alpha = parse_card('alpha.md', $normal);
    die "default-status parser case failed\n" if $alpha->{status} ne 'current';
    my $beta = parse_card('beta.md', $inline);
    die "inline-answer parser case failed\n" if $beta->{status} ne 'deprecated';
    my $rendered = render_catalog([$alpha, $beta]);
    die "Markdown pipe escaping failed\n" if index($rendered, 'Beta \\| title') < 0;

    my $missing_answers = eval {
        parse_card('gamma.md', $normal =~ s/answers:/questions:/r); 1;
    };
    die "missing-answers case did not fail\n" if $missing_answers;
    my $mismatch = eval {
        parse_card('other.md', $normal); 1;
    };
    die "filename/id mismatch case did not fail\n" if $mismatch;
    my $bad_date = eval {
        parse_card('alpha.md', $normal =~ s/2026-08-08/2026-13-08/r); 1;
    };
    die "invalid-date case did not fail\n" if $bad_date;
    my $no_evidence = eval {
        parse_card('alpha.md', $normal =~ s/^evidence:.*\n//mr); 1;
    };
    die "missing-evidence case did not fail\n" if $no_evidence;
    my $long_title = 'x' x ($MAX_SOURCE_TITLE_BYTES + 1);
    my $oversized = eval {
        parse_card('alpha.md', $normal =~ s/Alpha title/$long_title/r); 1;
    };
    die "oversized-title case did not fail\n" if $oversized;
    my $compacted = compact_title('word ' x 80);
    die "title compaction bound failed\n"
      if length(encode_utf8($compacted)) > $MAX_TITLE_CELL_BYTES || $compacted !~ /…\z/;
}
