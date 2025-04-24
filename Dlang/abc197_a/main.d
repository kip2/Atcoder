import std.stdio;
import std.string;
import std.algorithm;
import std.array;
import std.conv;

string solve(string input) {
    string first = input[0 .. 1];
    string rest = input[1 .. $];
    return rest ~ first;
}

string getInput() {
    string input = readln().strip();
    return input;
}

int[] getInputOfIntArray() {
    return readln().split().map!(to!int).array;
}

void main() {
    string result = solve(getInput());

    writeln(result);
}
