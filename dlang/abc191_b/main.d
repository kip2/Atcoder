module dlang.abc191_b.main;

import std.stdio;
import std.string;
import std.algorithm;
import std.array;
import std.conv;

string solve(string input) {
    return input;
}

string getInput() {
    return readln().strip();
}

int[] getInputOfIntArray() {
    return readln().split().map!(to!int).array;
}

void main() {
    string result = solve(getInput());

    writeln(result);
}
