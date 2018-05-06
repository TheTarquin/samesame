#!/usr/bin/perl

#use strict;
use utf8;
use Encode; 
use Encode::Punycode;
use open ':std', ':encoding(UTF-8)';

sub codepoint_hex {
    if (my $char = shift) {
        return sprintf '%2.2x', unpack('U0U*', $char);
    }
}
  
open my $fh, '<', '../confusables.csv' or die "Cannot open: $!";
while (my $line = <$fh>) {
   $line =~ s/\s*\z//;
   my @array = split /,/, $line;
   my $key = shift @array;
   $confusable{$key} = shift @array;
}
close $fh;
#open my $fh, '<', '../100.puny.com' or die "Cannot open: $!";
open my $fh, '<', '../2.puny.com' or die "Cannot open: $!";
while (my $line = <$fh>) {
   $line =~ s/\s*\z//;
   $line =~/xn--(.*)\.[A-Za-z]+\./;
   $dom{$line} = decode('Punycode', $1);
}
for $each (keys %dom) {
   $val = $dom{$each};
   undef($new);
   for $every (split("", $val)) {
      $cp = codepoint_hex($every);
      $cpU = uc $cp;
      while (length($cpU) < 4) {
         $cpU = '0'.$cpU;
      }
      if ($confusable{$cpU}) {
      # if the confusable val exists then we have a potential problem.
         if ($confusable{$cpU} =~ /\s/) {
            for $vine (split(/\s/, $confusable{$cpU})) {
               $new .= chr(hex($vine));
            }
         }
         $new .= chr(hex($confusable{$cpU}));
      } else {
         $new .= $every;
      }
   }
   if ($new ne $val) {
      print "ISSUES : $val != $new\n";
      #Really here we look in the top Alexa Top 1MM.  If the 'clean' is in here then problem
   } else {
      #print "CLEAN  : $val = $new\n";
   }
}

