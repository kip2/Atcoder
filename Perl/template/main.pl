#!/usr/bin/env perl
use strict;
use warnings;

my $line = <STDIN>;
chomp $line;
my ($a, $b, $c) = split ' ', $line;

if ($a * $a + $b * $b < $c * $c) {
    print "Yes\n";
} else {
    print "No\n";
}
