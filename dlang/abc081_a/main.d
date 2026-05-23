module abc081_a.main;

import std.stdio;
import std.string;
import std.algorithm;
import std.array;
import std.conv;

void main() {
    string s = readln.chomp;

    writeln(s.count('1'));
}
