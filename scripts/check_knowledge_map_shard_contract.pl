#!/usr/bin/env perl
use strict;
use warnings;
use utf8;

use Cwd qw(abs_path);
use Digest::SHA qw(sha256_hex);
use Encode qw(decode_utf8 encode_utf8);
use File::Basename qw(basename dirname);
use File::Glob qw(bsd_glob GLOB_NOSORT);
use File::Spec;
use FindBin qw($Bin);
use JSON::PP;

binmode STDOUT, ':encoding(UTF-8)';
binmode STDERR, ':encoding(UTF-8)';

my $root = abs_path(File::Spec->catdir($Bin, '..'));
my $contract_rel = 'doctrine/knowledge_map/shard_contract.json';
my $mode = 'check';
while (@ARGV) {
    my $arg = shift @ARGV;
    if ($arg eq '--check') {
        $mode = 'check';
    } elsif ($arg eq '--report') {
        $mode = 'report';
    } elsif ($arg eq '--self-test') {
        $mode = 'self-test';
    } elsif ($arg eq '--root') {
        die "knowledge-map-shards: --root requires a path\n" if !@ARGV;
        $root = abs_path(shift @ARGV);
        die "knowledge-map-shards: root does not exist\n" if !defined $root;
    } elsif ($arg eq '--contract') {
        die "knowledge-map-shards: --contract requires a repository-relative path\n" if !@ARGV;
        $contract_rel = shift @ARGV;
    } else {
        die "Usage: $0 [--check|--report|--self-test] [--root PROJECT_ROOT] [--contract PATH]\n";
    }
}

if ($mode eq 'self-test') {
    run_self_test();
    print "knowledge-map-shards: 13/13 identity, collision, ordering, wrapping, bound, and plane-pressure tests pass.\n";
    exit 0;
}

my $ok = eval {
    die "contract path is unsafe\n" if !safe_relative_path($contract_rel);
    my $contract = read_contract(absolute($contract_rel));
    validate_generator_contract($contract);
    my @facts = collect_facts($contract);
    my $plan = plan_projection($contract, \@facts);
    $plan->{pressure} = derive_plane_pressure($contract->{limits}, $plan);
    print STDERR "knowledge-map-shards: warning: $_\n"
      for @{ $plan->{pressure}{warnings} };
    if ($mode eq 'report') {
        print JSON::PP->new->canonical(1)->encode($plan), "\n";
    } else {
        print "knowledge-map-shards: contract is feasible for $plan->{facts} facts / "
          . "$plan->{question_keys} unique question keys in $plan->{shards} bounded shards; "
          . "canonical input SHA-256 $plan->{canonical_input_sha256}.\n";
    }
    1;
};
if (!$ok) {
    my $error = $@ || 'unknown failure';
    $error =~ s/\s+\z//;
    print STDERR "knowledge-map-shards: $error\n";
    exit 1;
}

sub read_contract {
    my ($path) = @_;
    die "contract exceeds 8,192 bytes\n" if (-s $path) > 8_192;
    my $contract = decode_json_file($path);
    require_exact_keys(
        $contract,
        'shard contract',
        qw(schema_version canonical_input_globs landing_path shard_directory shard_prefix fact_catalog limits),
    );
    die "shard contract schema_version must be 1\n" if ($contract->{schema_version} // 0) != 1;
    die "canonical_input_globs must contain 1..8 paths\n"
      if ref($contract->{canonical_input_globs}) ne 'ARRAY'
      || !@{ $contract->{canonical_input_globs} }
      || @{ $contract->{canonical_input_globs} } > 8;
    for my $glob (@{ $contract->{canonical_input_globs} }) {
        die "unsafe canonical input glob\n"
          if !safe_relative_path($glob) || $glob !~ /\.md\z/;
    }
    for my $field (qw(landing_path shard_directory fact_catalog)) {
        die "unsafe or empty contract path '$field'\n"
          if !defined($contract->{$field}) || ref($contract->{$field})
          || !safe_relative_path($contract->{$field});
    }
    die "landing_path and fact_catalog must be Markdown files\n"
      if $contract->{landing_path} !~ /\.md\z/ || $contract->{fact_catalog} !~ /\.md\z/;
    die "shard_prefix is invalid\n"
      if !defined($contract->{shard_prefix}) || ref($contract->{shard_prefix})
      || $contract->{shard_prefix} !~ /\A[a-z0-9][a-z0-9-]*-\z/;

    my @limit_fields = qw(
      max_facts max_question_keys max_question_bytes max_id_bytes max_path_bytes
      max_link_line_bytes wrap_line_bytes target_shard_lines target_shard_bytes
      max_shard_lines max_shard_bytes max_shards
      max_projection_lines max_projection_bytes max_landing_lines max_landing_bytes
      max_landing_line_bytes
    );
    require_exact_keys($contract->{limits}, 'shard limits', @limit_fields);
    my %hard_cap = (
        max_facts => 512,
        max_question_keys => 4_096,
        max_question_bytes => 4_096,
        max_id_bytes => 128,
        max_path_bytes => 256,
        max_link_line_bytes => 512,
        wrap_line_bytes => 512,
        target_shard_lines => 512,
        target_shard_bytes => 65_536,
        max_shard_lines => 512,
        max_shard_bytes => 65_536,
        max_shards => 64,
        max_projection_lines => 32_768,
        max_projection_bytes => 2_097_152,
        max_landing_lines => 128,
        max_landing_bytes => 16_384,
        max_landing_line_bytes => 512,
    );
    for my $field (@limit_fields) {
        my $value = $contract->{limits}{$field};
        die "shard limit '$field' must be a positive integer\n"
          if !defined($value) || ref($value) || $value !~ /^\d+$/ || $value == 0;
        die "shard limit '$field' exceeds hard cap $hard_cap{$field}\n"
          if $value > $hard_cap{$field};
    }
    die "wrap_line_bytes must not exceed max_link_line_bytes\n"
      if $contract->{limits}{wrap_line_bytes} > $contract->{limits}{max_link_line_bytes};
    die "target shard bounds must not exceed hard shard bounds\n"
      if $contract->{limits}{target_shard_lines} > $contract->{limits}{max_shard_lines}
      || $contract->{limits}{target_shard_bytes} > $contract->{limits}{max_shard_bytes};
    return $contract;
}

sub validate_generator_contract {
    my ($contract) = @_;
    my $generator = absolute('knowledge-map/scripts/gen_knowledge_map.sh');
    return if !-f $generator;
    open(my $fh, '-|', 'bash', $generator, '--print-contract')
      or die "cannot execute Knowledge Map generator contract query: $!\n";
    local $/;
    my $raw = <$fh> // '';
    close($fh) or die "Knowledge Map generator contract query failed\n";
    my $actual = eval { JSON::PP->new->decode($raw) };
    die "Knowledge Map generator returned invalid contract JSON\n"
      if !$actual || ref($actual) ne 'HASH';
    my $expected = {
        landing_path => $contract->{landing_path},
        shard_directory => $contract->{shard_directory},
        shard_prefix => $contract->{shard_prefix},
        fact_catalog => $contract->{fact_catalog},
        limits => $contract->{limits},
    };
    my $json = JSON::PP->new->canonical(1);
    die "Knowledge Map generator settings drift from doctrine/knowledge_map/shard_contract.json\n"
      if $json->encode($actual) ne $json->encode($expected);
}

sub collect_facts {
    my ($contract) = @_;
    my %path_seen;
    my @paths;
    for my $pattern (@{ $contract->{canonical_input_globs} }) {
        my @matches = bsd_glob(absolute($pattern), GLOB_NOSORT);
        die "canonical input glob '$pattern' has no matches\n" if !@matches;
        for my $absolute_path (@matches) {
            next if !-f $absolute_path;
            my $relative = File::Spec->abs2rel($absolute_path, $root);
            $relative =~ s{\\}{/}g;
            push @paths, $relative if !$path_seen{$relative}++;
        }
    }

    my @facts;
    for my $path (sort { byte_cmp($a, $b) } @paths) {
        my $raw = slurp_raw(absolute($path));
        my ($text, $raw_sha256) = decode_fact_bytes($raw);
        my $fact = parse_fact($path, $text);
        next if !defined $fact;
        $fact->{raw_sha256} = $raw_sha256;
        push @facts, $fact;
    }
    return @facts;
}

sub decode_fact_bytes {
    my ($raw) = @_;
    my $raw_sha256 = sha256_hex($raw);
    my $decode_input = $raw;
    my $text = decode_utf8($decode_input, 1);
    return ($text, $raw_sha256);
}

sub parse_fact {
    my ($path, $text) = @_;
    my ($front_matter) = $text =~ /\A---\r?\n(.*?)\r?\n---(?:\r?\n|\z)/s;
    return undef if !defined $front_matter;
    my %scalar;
    my @answers;
    my $current_key = '';
    for my $line (split /\r?\n/, $front_matter) {
        if ($line =~ /^[ \t]+-[ \t]+(.+)\z/) {
            push @answers, clean_scalar($1) if $current_key eq 'answers';
            next;
        }
        next if $line =~ /^\s*\z/;
        my ($key, $rest) = $line =~ /\A([A-Za-z_][A-Za-z0-9_]*):[ \t]*(.*)\z/;
        next if !defined $key;
        $current_key = $key;
        if ($rest =~ /^\[(.*)\]\z/) {
            push @answers, map { clean_scalar($_) } split /,/, $1, -1 if $key eq 'answers';
            $current_key = '';
        } elsif ($rest ne '') {
            $scalar{$key} = clean_scalar($rest);
            $current_key = '';
        }
    }
    @answers = grep { $_ ne '' } @answers;
    return undef if !@answers;
    return {
        id => $scalar{id} // '',
        path => $path,
        answers => \@answers,
    };
}

# LIVE-DOCUMENT-PRESSURE-HEADROOM.28 — `max_facts` and `max_question_keys` were the only capacity
# authorities left in this repository that refused with no band. Every file, line, and byte dimension is
# banded by the surface registry, and every bounded-registry record count by the central discovery
# `.22b` added over `doctrine/**/*.jsonl`; but a fact and a question key are neither a file nor a
# registry record, so these two fell between both mechanisms and stayed silent right up to an
# unconditional `die` — the exact LIVE-DOC-STOP-RISK shape this tree exists to refuse. `max_shards` is
# deliberately NOT banded here: the `knowledge_map` surface already bounds this projection at 33 files,
# which is 1 landing plus `max_shards`, so the two report the same state and a second band would only
# duplicate it (the rule `.25` derived for `fact_card_titles`).
sub plane_milestones {
    return {warning_pct => 80, rollover_pct => 90};
}

# Pure: takes the declared limits and a rendered plan, returns the derived pressure. Keeping it free of
# I/O is what lets a self-test case pin the WARNING TEXT — `.29c` shipped a wrong warning arm precisely
# because its suite could only inspect errors, and an arm no case can read is an arm nothing checks.
sub derive_plane_pressure {
    my ($limits, $plan) = @_;
    my $milestones = plane_milestones();
    my @warnings;
    for my $dimension (
        ['facts', $plan->{facts}, $limits->{max_facts}, 'max_facts'],
        ['question keys', $plan->{question_keys}, $limits->{max_question_keys}, 'max_question_keys'],
    ) {
        my ($label, $actual, $bound, $field) = @$dimension;
        next if !defined($bound) || !defined($actual) || $bound == 0;
        my $percent = 100 * $actual / $bound;
        next if $percent < $milestones->{warning_pct};
        my $band = $percent >= $milestones->{rollover_pct} ? 'rollover' : 'warning';
        push @warnings, sprintf(
            '%s is at or above %s (%.1f%%) — %d below its %d %s',
            $label, $band, $percent, $bound - $actual, $bound, $field,
        );
    }

    # The declared fact capacity is FUNDED by a fixed eight keys per fact rounded up to a 512-key
    # quantum (`check_fact_card_catalog.pl` derives `max_question_keys` that way), but a fact measures
    # more than eight questions. The two agree today only because the quantum rounds 449 x 8 = 3,592 up
    # to 4,096, and that slack is an artifact of rounding, not a funded margin: above the funded count
    # the contract would accept a declaration the projection then refuses. Derive the funded count from
    # the measured population in integers so the figure reproduces exactly from the two numbers named.
    my $funded;
    my ($facts, $keys) = ($plan->{facts}, $plan->{question_keys});
    if (defined($facts) && defined($keys) && $facts > 0 && $keys > 0
        && defined($limits->{max_question_keys})) {
        $funded = int($limits->{max_question_keys} * $facts / $keys);
        push @warnings, sprintf(
            'declared max_facts %d exceeds the %d facts a %d-key budget funds '
              . 'at the measured %d keys over %d facts',
            $limits->{max_facts}, $funded, $limits->{max_question_keys}, $keys, $facts,
        ) if defined($limits->{max_facts}) && $limits->{max_facts} > $funded;
    }

    return {
        warning_pct => $milestones->{warning_pct},
        rollover_pct => $milestones->{rollover_pct},
        funded_facts => $funded,
        warnings => \@warnings,
    };
}

sub plan_projection {
    my ($contract, $facts) = @_;
    my $limits = $contract->{limits};
    die "fact count exceeds $limits->{max_facts}\n" if @$facts > $limits->{max_facts};

    my %id_seen;
    my %question_seen;
    my @entries;
    my @identity;
    for my $fact (@$facts) {
        my $id = $fact->{id};
        die "$fact->{path} lacks a fact id\n" if $id eq '';
        die "fact id '$id' is not a safe kebab-case identifier\n"
          if $id !~ /\A[a-z0-9][a-z0-9-]*\z/;
        die "fact id '$id' exceeds $limits->{max_id_bytes} UTF-8 bytes\n"
          if utf8_bytes($id) > $limits->{max_id_bytes};
        die "duplicate fact id '$id'\n" if $id_seen{$id}++;
        die "fact path '$fact->{path}' exceeds $limits->{max_path_bytes} UTF-8 bytes\n"
          if utf8_bytes($fact->{path}) > $limits->{max_path_bytes};
        push @identity, $fact->{path} . "\0" . ($fact->{raw_sha256} // 'self-test');
        for my $question (@{ $fact->{answers} }) {
            die "question for '$id' exceeds $limits->{max_question_bytes} UTF-8 bytes\n"
              if utf8_bytes($question) > $limits->{max_question_bytes};
            if (exists $question_seen{$question}) {
                my $prior = $question_seen{$question};
                die "question collision between '$prior' and '$id': $question\n";
            }
            $question_seen{$question} = $id;
            push @entries, {
                id => $id,
                path => $fact->{path},
                question => $question,
            };
        }
    }
    die "question-key count exceeds $limits->{max_question_keys}\n"
      if @entries > $limits->{max_question_keys};
    @entries = sort {
        byte_cmp($a->{question}, $b->{question}) || byte_cmp($a->{id}, $b->{id})
    } @entries;

    my @shards = pack_shards($contract, \@entries);
    die "projection requires " . scalar(@shards) . " shards; limit is $limits->{max_shards}\n"
      if @shards > $limits->{max_shards};
    my $digest = sha256_hex(join("\n", sort { byte_cmp($a, $b) } @identity) . "\n");
    my $landing = render_landing($contract, \@shards, scalar(@$facts), scalar(@entries), $digest);
    my ($landing_lines, $landing_bytes, $landing_width) = text_metrics($landing);
    die "landing index exceeds line bound\n" if $landing_lines > $limits->{max_landing_lines};
    die "landing index exceeds byte bound\n" if $landing_bytes > $limits->{max_landing_bytes};
    die "landing index exceeds line-width bound\n"
      if $landing_width > $limits->{max_landing_line_bytes};

    my ($max_lines, $max_bytes, $max_width, $total_lines, $total_bytes) = (0, 0, 0, 0, 0);
    for my $shard (@shards) {
        $max_lines = $shard->{lines} if $shard->{lines} > $max_lines;
        $max_bytes = $shard->{bytes} if $shard->{bytes} > $max_bytes;
        $max_width = $shard->{line_bytes} if $shard->{line_bytes} > $max_width;
        $total_lines += $shard->{lines};
        $total_bytes += $shard->{bytes};
    }
    die "projection set exceeds aggregate line bound\n"
      if $total_lines + $landing_lines > $limits->{max_projection_lines};
    die "projection set exceeds aggregate byte bound\n"
      if $total_bytes + $landing_bytes > $limits->{max_projection_bytes};
    return {
        facts => scalar(@$facts),
        question_keys => scalar(@entries),
        shards => scalar(@shards),
        canonical_input_sha256 => $digest,
        landing => {
            lines => $landing_lines,
            bytes => $landing_bytes,
            line_bytes => $landing_width,
        },
        shard_max => {
            lines => $max_lines,
            bytes => $max_bytes,
            line_bytes => $max_width,
        },
        shard_total => {
            lines => $total_lines,
            bytes => $total_bytes,
        },
        output_files => 1 + scalar(@shards),
    };
}

sub pack_shards {
    my ($contract, $entries) = @_;
    my $limits = $contract->{limits};
    my @shards;
    my @current;
    for my $entry (@$entries) {
        my @candidate = (@current, $entry);
        my $number = @shards + 1;
        my $text = render_shard($contract, $number, \@candidate);
        my ($lines, $bytes) = text_metrics($text);
        if (@current && ($lines > $limits->{target_shard_lines} || $bytes > $limits->{target_shard_bytes})) {
            push @shards, finish_shard($contract, $number, \@current);
            @current = ($entry);
        } else {
            @current = @candidate;
        }
    }
    push @shards, finish_shard($contract, scalar(@shards) + 1, \@current) if @current;
    return @shards;
}

sub finish_shard {
    my ($contract, $number, $entries) = @_;
    my $text = render_shard($contract, $number, $entries);
    my ($lines, $bytes, $width) = text_metrics($text);
    my $limits = $contract->{limits};
    die "shard $number exceeds line bound\n" if $lines > $limits->{max_shard_lines};
    die "shard $number exceeds byte bound\n" if $bytes > $limits->{max_shard_bytes};
    die "shard $number exceeds line-width bound\n" if $width > $limits->{max_link_line_bytes};
    return {
        number => $number,
        keys => scalar(@$entries),
        lines => $lines,
        bytes => $bytes,
        line_bytes => $width,
        text => $text,
    };
}

sub render_shard {
    my ($contract, $number, $entries) = @_;
    my @lines = (
        '# Knowledge questions — shard ' . sprintf('%04d', $number),
        '',
        '> **AUTO-GENERATED — DO NOT EDIT.** Canonical questions live in fact front matter.',
        '',
    );
    my $shard_dir = absolute($contract->{shard_directory});
    for my $entry (@$entries) {
        my $relative = File::Spec->abs2rel(absolute($entry->{path}), $shard_dir);
        $relative =~ s{\\}{/}g;
        my $link = '- [' . $entry->{id} . '](' . $relative . ')';
        die "question link line exceeds $contract->{limits}{max_link_line_bytes} bytes\n"
          if utf8_bytes($link) > $contract->{limits}{max_link_line_bytes};
        push @lines, $link;
        push @lines, wrap_question($entry->{question}, $contract->{limits}{wrap_line_bytes});
    }
    return join("\n", @lines) . "\n";
}

sub wrap_question {
    my ($question, $limit) = @_;
    $question =~ s/\s+/ /g;
    $question =~ s/^ | $//g;
    my @words = split / /, $question;
    my @lines;
    my $line = '  >';
    for my $word (@words) {
        die "question token exceeds wrap bound\n" if utf8_bytes('  > ' . $word) > $limit;
        my $candidate = "$line $word";
        if (utf8_bytes($candidate) > $limit) {
            push @lines, $line;
            $line = "  > $word";
        } else {
            $line = $candidate;
        }
    }
    push @lines, $line if $line ne '  >';
    return @lines;
}

sub render_landing {
    my ($contract, $shards, $facts, $questions, $digest) = @_;
    my @lines = (
        '# Knowledge Map',
        '',
        '> **AUTO-GENERATED — DO NOT EDIT.** Canonical facts live in front-mattered source files.',
        '',
        "- Facts: **$facts**",
        "- Unique question keys: **$questions**",
        "- Canonical input SHA-256: `$digest`",
        '- Browse by id/title: [`' . $contract->{fact_catalog} . '`](' . $contract->{fact_catalog} . ')',
        '- Search all question shards: `rg -i --glob \'' . $contract->{shard_prefix}
          . "*.md' 'terms' " . $contract->{shard_directory} . '`',
        '',
        '## Question shards',
        '',
    );
    for my $shard (@$shards) {
        my $name = sprintf('%s%04d.md', $contract->{shard_prefix}, $shard->{number});
        push @lines, '- [' . sprintf('Shard %04d', $shard->{number}) . ']('
          . $contract->{shard_directory} . "/$name) — $shard->{keys} keys";
    }
    return join("\n", @lines) . "\n";
}

sub text_metrics {
    my ($text) = @_;
    my $bytes = utf8_bytes($text);
    my $lines = ($text =~ tr/\n/\n/);
    my $width = 0;
    for my $line (split /\n/, $text, -1) {
        my $size = utf8_bytes($line);
        $width = $size if $size > $width;
    }
    return ($lines, $bytes, $width);
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

sub byte_cmp {
    return encode_utf8($_[0]) cmp encode_utf8($_[1]);
}

sub utf8_bytes {
    return length(encode_utf8($_[0] // ''));
}

sub safe_relative_path {
    my ($path) = @_;
    return 0 if !defined($path) || ref($path) || $path eq '' || $path =~ m{\A/} || $path =~ /\\/;
    return 0 if $path =~ m{(?:\A|/)\.\.(?:/|\z)} || $path =~ /[\x00-\x1f]/;
    return 1;
}

sub absolute {
    my ($relative) = @_;
    return File::Spec->catfile($root, split m{/}, $relative);
}

sub slurp_raw {
    my ($path) = @_;
    open(my $fh, '<:raw', $path) or die "cannot read '$path': $!\n";
    local $/;
    my $bytes = <$fh> // '';
    close($fh) or die "cannot close '$path': $!\n";
    return $bytes;
}

sub decode_json_file {
    my ($path) = @_;
    my $raw = slurp_raw($path);
    my $value = eval { JSON::PP->new->utf8->decode($raw) };
    die "invalid JSON in '$path': $@" if !$value || ref($value) ne 'HASH';
    return $value;
}

sub require_exact_keys {
    my ($record, $label, @keys) = @_;
    die "$label must be an object\n" if ref($record) ne 'HASH';
    my %allowed = map { $_ => 1 } @keys;
    die "$label has unknown field '$_'\n" for grep { !$allowed{$_} } sort keys %$record;
    die "$label lacks field '$_'\n" for grep { !exists $record->{$_} } @keys;
}

sub self_test_contract {
    return {
        canonical_input_globs => ['docs/knowledge/*.md'],
        landing_path => 'KNOWLEDGE_MAP.md',
        shard_directory => 'docs/knowledge-map',
        shard_prefix => 'questions-',
        fact_catalog => 'docs/knowledge/INDEX.md',
        limits => {
            max_facts => 10,
            max_question_keys => 20,
            max_question_bytes => 64,
            max_id_bytes => 32,
            max_path_bytes => 64,
            max_link_line_bytes => 96,
            wrap_line_bytes => 32,
            target_shard_lines => 10,
            target_shard_bytes => 384,
            max_shard_lines => 12,
            max_shard_bytes => 512,
            max_shards => 10,
            max_projection_lines => 128,
            max_projection_bytes => 4_096,
            max_landing_lines => 32,
            max_landing_bytes => 2_048,
            max_landing_line_bytes => 256,
        },
    };
}

sub run_self_test {
    my $utf8_bytes = encode_utf8("identity éclair\n");
    my ($decoded, $identity) = decode_fact_bytes($utf8_bytes);
    die "raw-content identity case failed\n"
      if $decoded ne "identity éclair\n" || $identity ne sha256_hex($utf8_bytes);

    my $block = <<'FACT';
---
id: alpha
answers:
  - "where is alpha"
date: 2026-08-08
---
FACT
    my $inline = <<'FACT';
---
id: beta
answers: [what is beta]
date: 2026-08-08
---
FACT
    my $alpha = parse_fact('docs/knowledge/alpha.md', $block);
    die "block-answer parser case failed\n" if $alpha->{answers}[0] ne 'where is alpha';
    my $beta = parse_fact('docs/knowledge/beta.md', $inline);
    die "inline-answer parser case failed\n" if $beta->{answers}[0] ne 'what is beta';
    $alpha->{raw_sha256} = 'a' x 64;
    $beta->{raw_sha256} = 'b' x 64;

    my $contract = self_test_contract();
    my $collision = eval {
        my %copy = %$beta;
        $copy{answers} = ['where is alpha'];
        plan_projection($contract, [$alpha, \%copy]);
        1;
    };
    die "question-collision case did not fail\n" if $collision;

    my @ordered = sort { byte_cmp($a, $b) } ('zeta', 'alpha', 'éclair');
    die "UTF-8 byte-order case failed\n" if join('|', @ordered) ne 'alpha|zeta|éclair';

    my @wrapped = wrap_question('one two three four five six seven eight nine', 18);
    die "question-wrap case failed\n"
      if grep { utf8_bytes($_) > 18 } @wrapped;

    my %oversized = %$alpha;
    $oversized{answers} = ['x' x 65];
    my $question_bound = eval { plan_projection($contract, [\%oversized]); 1 };
    die "question-byte case did not fail\n" if $question_bound;

    my @many_answers = map { "question number $_ with enough words" } 1 .. 8;
    my %many = %$alpha;
    $many{answers} = \@many_answers;
    my $rollover = plan_projection($contract, [\%many]);
    die "shard-rollover case failed\n" if $rollover->{shards} < 2;

    my $landing_bound = self_test_contract();
    $landing_bound->{limits}{max_landing_lines} = 1;
    my $landing = eval { plan_projection($landing_bound, [$alpha]); 1 };
    die "landing-bound case did not fail\n" if $landing;

    # LIVE-DOCUMENT-PRESSURE-HEADROOM.28 — four cases on the band itself, and they pin the TEXT rather
    # than a count, because the arm being checked never fails the build: a warning that reads wrongly
    # is invisible to a suite that can only observe a refusal.
    my $plane = self_test_contract()->{limits};
    my $quiet = derive_plane_pressure($plane, {facts => 7, question_keys => 14});
    die "plane-silence case failed\n"
      if @{ $quiet->{warnings} } || ($quiet->{funded_facts} // -1) != 10;

    my $banded = derive_plane_pressure($plane, {facts => 8, question_keys => 16});
    die "plane-warning-band case failed\n"
      if join('|', @{ $banded->{warnings} })
      ne 'facts is at or above warning (80.0%) — 2 below its 10 max_facts'
      . '|question keys is at or above warning (80.0%) — 4 below its 20 max_question_keys';

    my $rolled = derive_plane_pressure($plane, {facts => 9, question_keys => 18});
    die "plane-rollover-band case failed\n"
      if join('|', @{ $rolled->{warnings} })
      ne 'facts is at or above rollover (90.0%) — 1 below its 10 max_facts'
      . '|question keys is at or above rollover (90.0%) — 2 below its 20 max_question_keys';

    # Declared capacity above what the key budget funds at the measured ratio: the state an eighth
    # title part would create, reported in the commit that declares it instead of at a later refusal.
    my $unfunded = derive_plane_pressure($plane, {facts => 4, question_keys => 10});
    die "plane-funding case failed\n"
      if ($unfunded->{funded_facts} // -1) != 8
      || join('|', @{ $unfunded->{warnings} })
      ne 'declared max_facts 10 exceeds the 8 facts a 20-key budget funds '
      . 'at the measured 10 keys over 4 facts';
}
