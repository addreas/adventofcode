#!/usr/bin/perl
my @lines = <>;
my $rows = @lines * 1;
my $cols = length($lines[0]);

sub count_neighbors {
    local ($i, $j, @prev) = @_;
    local $count = 0;
    foreach $a ( $i - 1 .. $i + 1) {
       foreach $b ( $j - 1 .. $j + 1) {
           if(($i != $a or $j != b) and "#" eq substr($prev[$a], $b, 1)) {
               $count += 1;
           }
       }
    }
    return $count;
}

sub generation {
    local @prev = @_;
    local @next = ();
    local $changes = 0;
    foreach $i (0 .. $rows - 1) {
        local $next_line = "";
        foreach $j (0 .. $cols - 1) {
            local $neighbors = count_neighbors($i, $j, @prev);
            local $this = substr($prev[$i], $j, 1);

            if ($this eq "L" and $neighbors == 0) {
                $this = "#";
                $changes += 1;
            } elsif ($this eq "#" and $neighbors >= 4) {
                $this = "L";
                $changes += 1;
            }

            $next_line .= $this;
        }
        push @next, $next_line;
    }
    return $changes, @next;
}


my @gen = @lines;
my $changes = 1;
while ($changes) {
    ($changes, @gen) = generation(@gen);
    print "@gen\n";
    print "$changes\n";
}

print @gen

#TODO: Find the bug and solve part two.
