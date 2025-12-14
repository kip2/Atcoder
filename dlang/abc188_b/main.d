import std.stdio;
import std.string;
import std.algorithm;
import std.array;
import std.conv;
import std.range;

string solve(int[] a, int[] b) {
    return zip(a, b)
        .map!(ab => ab[0] * ab[1])
        .sum == 0 ? "Yes" : "No";
}

string getInput() {
    return readln().strip();
}

int[] getInputOfIntArray() {
    return readln().split().map!(to!int).array;
}

void main() {
    getInput();
    int[] a = getInputOfIntArray();
    int[] b = getInputOfIntArray();

    string result = solve(a, b);
    writeln(result);
}

