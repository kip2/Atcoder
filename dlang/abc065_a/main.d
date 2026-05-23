module abc065_a.main;

import std.stdio;
import std.string;
import std.algorithm;
import std.array;
import std.conv;

string solve(int x, int a, int b) {
    int expirationDays = -a + b;
    if (expirationDays <= 0)
        return "delicious";
    else if (expirationDays <= x)
        return "safe";
    else
        return "dangerous";
}

void main() {
    auto ln = readln.split.map!(to!int).array;
    int x = ln[0];
    int a = ln[1];
    int b = ln[2];
    string result = solve(x, a, b);

    writeln(result);
}
