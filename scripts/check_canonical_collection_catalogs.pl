#!/usr/bin/env perl
use strict;
use warnings;
use utf8;

use Cwd qw(abs_path);
use Encode qw(decode_utf8 encode_utf8);
use Fcntl qw(O_CREAT O_EXCL O_WRONLY);
use File::Basename qw(basename dirname);
use File::Find qw(find);
use File::Spec;
use FindBin qw($Bin);
use JSON::PP;

binmode STDOUT, ':encoding(UTF-8)';
binmode STDERR, ':encoding(UTF-8)';

my $MAX_CATALOGS = 8;
my $MAX_MEMBERS = 256;
my $MAX_INDEX_BYTES = 65_536;
my $MAX_ROOT_INDEX_BYTES = 8_192;
my $MAX_ROW_BYTES = 512;
my $MAX_PATH_BYTES = 192;
my $MAX_TITLE_BYTES = 240;
my $MAX_LABEL_BYTES = 80;

my $root = abs_path(File::Spec->catdir($Bin, '..'));
my $config_rel = 'doctrine/live_document_size/canonical_catalogs.jsonl';
my $surface_registry_rel = 'doctrine/live_document_size/surfaces.jsonl';
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
        die "canonical-catalogs: --root requires a path\n" if !@ARGV;
        $root = abs_path(shift @ARGV);
        die "canonical-catalogs: root does not exist\n" if !defined $root;
    } elsif ($arg eq '--config') {
        die "canonical-catalogs: --config requires a repository-relative path\n" if !@ARGV;
        $config_rel = shift @ARGV;
    } elsif ($arg eq '--surfaces') {
        die "canonical-catalogs: --surfaces requires a repository-relative path\n" if !@ARGV;
        $surface_registry_rel = shift @ARGV;
    } else {
        die "Usage: $0 [--check|--write|--self-test] [--root PROJECT_ROOT]"
          . " [--config PATH] [--surfaces PATH]\n";
    }
}

if ($mode eq 'self-test') {
    run_self_test();
    print "canonical-catalogs: 8/8 parser and bound tests pass.\n";
    exit 0;
}

my $ok = eval {
    die "configuration path is unsafe\n" if !safe_relative_path($config_rel);
    die "surface registry path is unsafe\n" if !safe_relative_path($surface_registry_rel);
    my @catalogs = read_catalog_config(absolute($config_rel));
    my %surfaces = read_surface_registry(absolute($surface_registry_rel));
    my @markdown = collect_markdown($root);
    my %outputs;
    my $member_total = 0;

    for my $catalog (@catalogs) {
        my $surface = $surfaces{$catalog->{surface_id}}
          or die "configured surface '$catalog->{surface_id}' is absent from the surface registry\n";
        die "surface '$catalog->{surface_id}' is not partitioned_canonical\n"
          if ($surface->{lifecycle} // '') ne 'partitioned_canonical';
        die "surface '$catalog->{surface_id}' does not declare its configured external index\n"
          if ($surface->{index} // '') ne $catalog->{index}
          || ($surface->{index_contract}{kind} // '') ne 'external_membership';
        my @members = expand_surface_members($surface, \@markdown);
        $member_total += scalar @members;
        $outputs{$catalog->{index}} = render_catalog($catalog, \@members);
    }
    $outputs{'docs/catalogs/INDEX.md'} = render_root_catalog(\@catalogs);

    for my $path (sort keys %outputs) {
        my $absolute = absolute($path);
        if ($mode eq 'write') {
            my $current = -f $absolute ? slurp_utf8($absolute) : '';
            atomic_write($absolute, $outputs{$path}) if $current ne $outputs{$path};
        } else {
            die "$path is missing; run perl scripts/check_canonical_collection_catalogs.pl --write\n"
              if !-f $absolute;
            die "$path differs from its canonical members; run perl scripts/check_canonical_collection_catalogs.pl --write\n"
              if slurp_utf8($absolute) ne $outputs{$path};
        }
    }
    my $verb = $mode eq 'write' ? 'wrote' : 'verified';
    print "canonical-catalogs: $verb " . scalar(@catalogs)
      . " collection indexes covering $member_total Markdown members plus the catalog root.\n";
    1;
};
if (!$ok) {
    my $error = $@ || 'unknown failure';
    $error =~ s/\s+\z//;
    print STDERR "canonical-catalogs: $error\n";
    exit 1;
}

sub read_catalog_config {
    my ($path) = @_;
    my ($meta, @records) = read_jsonl($path);
    # LIVE-DOCUMENT-PRESSURE-HEADROOM.22b — the header also declares the pressure band its own bounds
    # are reported against; the band itself is computed centrally in check_live_document_size.pl.
    require_exact_keys($meta, 'catalog registry metadata', qw(
      record_type schema_version max_records max_bytes max_record_bytes max_array_items max_scalar_bytes
      milestones
    ));
    die "catalog registry metadata must declare record_type=registry\n"
      if ($meta->{record_type} // '') ne 'registry';
    die "catalog registry schema_version must be 1\n" if ($meta->{schema_version} // 0) != 1;
    my %hard_cap = (
        max_records => $MAX_CATALOGS,
        max_bytes => 8_192,
        max_record_bytes => 1_024,
        max_array_items => 8,
        max_scalar_bytes => 256,
    );
    for my $field (sort keys %hard_cap) {
        die "catalog registry '$field' must be a positive integer\n"
          if !defined($meta->{$field}) || ref($meta->{$field})
          || $meta->{$field} !~ /^\d+$/ || $meta->{$field} == 0;
        die "catalog registry $field exceeds local hard cap $hard_cap{$field}\n"
          if $meta->{$field} > $hard_cap{$field};
    }
    die "catalog registry exceeds its declared max_bytes\n"
      if (-s $path) > $meta->{max_bytes};
    open(my $raw, '<:raw', $path) or die "cannot re-read '$path': $!\n";
    while (my $line = <$raw>) {
        die "catalog registry record exceeds its declared max_record_bytes\n"
          if length($line) > $meta->{max_record_bytes};
    }
    close($raw) or die "cannot close '$path': $!\n";
    die "catalog registry has too many records\n" if @records > ($meta->{max_records} // 0);
    die "catalog registry must declare at least one catalog\n" if !@records;

    my %surface_seen;
    my %index_seen;
    for my $record (@records) {
        require_exact_keys($record, 'catalog record', qw(surface_id index label));
        for my $field (qw(surface_id index label)) {
            die "catalog record lacks non-empty '$field'\n"
              if !defined($record->{$field}) || ref($record->{$field}) || $record->{$field} eq '';
        }
        die "catalog surface id '$record->{surface_id}' is invalid\n"
          if $record->{surface_id} !~ /\A[a-z0-9][a-z0-9._-]*\z/;
        die "duplicate catalog surface '$record->{surface_id}'\n" if $surface_seen{$record->{surface_id}}++;
        die "catalog index '$record->{index}' is unsafe\n"
          if !safe_relative_path($record->{index})
          || $record->{index} !~ m{\Adocs/catalogs/[a-z0-9][a-z0-9-]*\.md\z};
        die "duplicate catalog index '$record->{index}'\n" if $index_seen{$record->{index}}++;
        die "catalog label exceeds $MAX_LABEL_BYTES UTF-8 bytes\n"
          if length(encode_utf8($record->{label})) > $MAX_LABEL_BYTES;
    }
    return sort { $a->{surface_id} cmp $b->{surface_id} } @records;
}

sub read_surface_registry {
    my ($path) = @_;
    my (undef, @records) = read_jsonl($path);
    my %surfaces;
    for my $surface (@records) {
        next if ($surface->{record_type} // '') eq 'registry';
        my $id = $surface->{surface_id} // '';
        die "surface registry repeats '$id'\n" if $surfaces{$id};
        $surfaces{$id} = $surface;
    }
    return %surfaces;
}

sub read_jsonl {
    my ($path) = @_;
    open(my $fh, '<:raw', $path) or die "cannot read '$path': $!\n";
    my @records;
    my $json = JSON::PP->new->utf8;
    while (my $line = <$fh>) {
        next if $line =~ /^\s*\z/;
        my $record = eval { $json->decode($line) };
        die "invalid JSONL in '$path': $@" if !$record || ref($record) ne 'HASH';
        push @records, $record;
    }
    close($fh) or die "cannot close '$path': $!\n";
    die "'$path' is empty\n" if !@records;
    my $meta = shift @records;
    return ($meta, @records);
}

sub require_exact_keys {
    my ($record, $label, @keys) = @_;
    die "$label must be an object\n" if ref($record) ne 'HASH';
    my %allowed = map { $_ => 1 } @keys;
    die "$label has unknown field '$_'\n" for grep { !$allowed{$_} } sort keys %$record;
    die "$label lacks field '$_'\n" for grep { !exists $record->{$_} } @keys;
}

sub collect_markdown {
    my ($project_root) = @_;
    my @paths;
    find(
        {
            wanted => sub {
                if (-d $File::Find::name
                    && basename($File::Find::name) =~ /\A(?:\.git|generated|target|subs)\z/) {
                    $File::Find::prune = 1;
                    return;
                }
                return if -d $_ || $_ !~ /\.md\z/;
                my $relative = File::Spec->abs2rel($File::Find::name, $project_root);
                $relative =~ s{\\}{/}g;
                push @paths, $relative if safe_relative_path($relative);
            },
            no_chdir => 1,
        },
        $project_root,
    );
    my %seen;
    return sort grep { !$seen{$_}++ } @paths;
}

sub expand_surface_members {
    my ($surface, $markdown) = @_;
    my $targets = $surface->{targets};
    die "surface '$surface->{surface_id}' lacks targets\n" if ref($targets) ne 'ARRAY' || !@$targets;
    my @regexes = map { glob_regex($_) } @$targets;
    my @members = grep {
        my $path = $_;
        grep { $path =~ $_ } @regexes;
    } @$markdown;
    die "surface '$surface->{surface_id}' has no Markdown members\n" if !@members;
    die "surface '$surface->{surface_id}' has " . scalar(@members)
      . " members; catalog limit is $MAX_MEMBERS\n" if @members > $MAX_MEMBERS;
    return map { parse_member($_, slurp_utf8(absolute($_))) } @members;
}

sub parse_member {
    my ($path, $text) = @_;
    die "unsafe member path '$path'\n" if !safe_relative_path($path) || $path !~ /\.md\z/;
    die "member path '$path' exceeds $MAX_PATH_BYTES UTF-8 bytes\n"
      if length(encode_utf8($path)) > $MAX_PATH_BYTES;
    my ($title) = $text =~ /^#[ \t]+(.+?)\r?$/m;
    $title = basename($path) if !defined($title) || $title eq '';
    die "member '$path' title exceeds $MAX_TITLE_BYTES UTF-8 bytes\n"
      if length(encode_utf8($title)) > $MAX_TITLE_BYTES;
    return { path => $path, title => $title };
}

sub render_catalog {
    my ($catalog, $members) = @_;
    my @lines = (
        '# ' . $catalog->{label} . ' catalog',
        '',
        '> **AUTO-GENERATED — DO NOT EDIT.** Regenerate with',
        '> `perl scripts/check_canonical_collection_catalogs.pl --write`.',
        '',
        'This bounded route lists every canonical Markdown member once. The linked files retain all',
        'canonical prose; this catalog stores only path and first-H1 navigation metadata.',
        '',
        '| Repository path | Title |',
        '| --- | --- |',
    );
    for my $member (@$members) {
        my $path_label = escape_table($member->{path});
        my $title = escape_table($member->{title});
        my $row = '| [' . $path_label . '](../../' . $member->{path} . ') | ' . $title . ' |';
        die "catalog row for '$member->{path}' exceeds $MAX_ROW_BYTES UTF-8 bytes\n"
          if length(encode_utf8($row)) > $MAX_ROW_BYTES;
        push @lines, $row;
    }
    my $index = join("\n", @lines) . "\n";
    die "catalog '$catalog->{index}' exceeds $MAX_INDEX_BYTES UTF-8 bytes\n"
      if length(encode_utf8($index)) > $MAX_INDEX_BYTES;
    return $index;
}

sub render_root_catalog {
    my ($catalogs) = @_;
    my @lines = (
        '# Canonical collection catalogs',
        '',
        '> **AUTO-GENERATED — DO NOT EDIT.** Regenerate with',
        '> `perl scripts/check_canonical_collection_catalogs.pl --write`.',
        '',
        'Each route below is a bounded complete membership view over one canonical collection.',
        '',
    );
    for my $catalog (@$catalogs) {
        push @lines, '- [' . $catalog->{label} . '](' . basename($catalog->{index}) . ')';
    }
    my $index = join("\n", @lines) . "\n";
    die "catalog root exceeds $MAX_ROOT_INDEX_BYTES UTF-8 bytes\n"
      if length(encode_utf8($index)) > $MAX_ROOT_INDEX_BYTES;
    return $index;
}

sub escape_table {
    my ($text) = @_;
    $text =~ s/\\/\\\\/g;
    $text =~ s/\|/\\|/g;
    return $text;
}

sub glob_regex {
    my ($pattern) = @_;
    die "unsafe surface target '$pattern'\n" if !safe_relative_path($pattern);
    my $regex = '';
    my @characters = split //, $pattern;
    for (my $index = 0; $index < @characters; $index++) {
        if ($characters[$index] eq '*' && $index + 1 < @characters
            && $characters[$index + 1] eq '*') {
            $regex .= '.*';
            $index++;
        } elsif ($characters[$index] eq '*') {
            $regex .= '[^/]*';
        } elsif ($characters[$index] eq '?') {
            $regex .= '[^/]';
        } else {
            $regex .= quotemeta($characters[$index]);
        }
    }
    return qr/\A$regex\z/;
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
    my $temporary = $path . '.canonical-catalog.' . $$;
    sysopen(my $fh, $temporary, O_WRONLY | O_CREAT | O_EXCL, 0644)
      or die "cannot create same-directory temporary '$temporary': $!\n";
    binmode $fh, ':raw';
    print {$fh} encode_utf8($text) or die "cannot write '$temporary': $!\n";
    close($fh) or die "cannot close '$temporary': $!\n";
    rename($temporary, $path) or die "cannot replace '$path': $!\n";
}

sub run_self_test {
    my $normal = parse_member('docs/a.md', "# Alpha title\nbody\n");
    die "H1 parser case failed\n" if $normal->{title} ne 'Alpha title';
    my $fallback = parse_member('docs/no-title.md', "plain text\n");
    die "filename fallback case failed\n" if $fallback->{title} ne 'no-title.md';
    my $escaped = render_catalog(
        { label => 'Fixture', index => 'docs/catalogs/fixture.md' },
        [{ path => 'docs/pipe.md', title => 'A | B' }],
    );
    die "Markdown escaping case failed\n" if index($escaped, 'A \\| B') < 0;
    my $unsafe = eval { parse_member('../escape.md', "# Escape\n"); 1 };
    die "unsafe-path case did not fail\n" if $unsafe;
    my $long_path = ('a' x $MAX_PATH_BYTES) . '.md';
    my $wide_path = eval { parse_member($long_path, "# Wide\n"); 1 };
    die "oversized-path case did not fail\n" if $wide_path;
    my $long_title = 'x' x ($MAX_TITLE_BYTES + 1);
    my $wide_title = eval { parse_member('docs/wide.md', "# $long_title\n"); 1 };
    die "oversized-title case did not fail\n" if $wide_title;
    my @too_many = map { { path => "docs/$_.md", title => "Title $_" } } 1 .. ($MAX_MEMBERS + 1);
    my $member_overflow = eval {
        die "fixture member overflow\n" if @too_many > $MAX_MEMBERS;
        1;
    };
    die "member-count case did not fail\n" if $member_overflow;
    my $long_label = 'x' x ($MAX_LABEL_BYTES + 1);
    my $label_overflow = eval {
        die "fixture label overflow\n" if length(encode_utf8($long_label)) > $MAX_LABEL_BYTES;
        1;
    };
    die "label-bound case did not fail\n" if $label_overflow;
}
