module abc081_b.main;

import std.stdio;
import std.string;
import std.algorithm;
import std.array;
import std.conv;

int countDivisibleBy2(int x) {
    int count = 0;
    while (x % 2 == 0) {
        x /= 2;
        count++;
    }
    return count;
}

void main() {
    int n = readln.chomp.to!int;
    int min = readln.split
        .map!(to!int)
        .map!(countDivisibleBy2)
        .minElement;

    writeln(min);
}
