module abc086_a.main;

import std.stdio;
import std.string;
import std.algorithm;
import std.array;
import std.conv;

string solve(int a, int b) {
    bool isEven = (a * b) % 2 == 0;
    return isEven ? "Even" : "Odd";
}

void main() {
    auto ab = readln.split.map!(to!int).array;
    int a = ab[0];
    int b = ab[1];

    writeln(solve(a, b));
}
