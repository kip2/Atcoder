import std.stdio;
import std.string;
import std.algorithm;
import std.array;
import std.conv;

string solve(string input) {
    return input;
}

void main() {
    string result = solve(getInput());

    writeln(result);
}

// --- getInt

int getInt() {
    return parseInt(readln());
}

int parseInt(string s) {
    return s.strip.to!int;
}

unittest {
    int expected = 3;
    int actual = parseInt("3");

    assert(actual == expected);
}

// ---

string getInput() {
    return readln().strip();
}

int[] getInputOfIntArray() {
    return readln().split().map!(to!int).array;
}

void printIntArr(int[] arr) {
    writeln(arr.map!(to!string).join(" "));
}
