module dlang.abc191_b.main;

import std.stdio;
import std.string;
import std.algorithm;
import std.array;
import std.conv;

int[] solve(int[] arr, int x) {
    return arr.filter!(a => a != x).array;
}

int[] getInputOfIntArray() {
    return readln().split().map!(to!int).array;
}

void main() {
    int[] line1 = getInputOfIntArray();
    int[] line2 = getInputOfIntArray();

    int n = line1[0];
    int x = line1[1];

    int[] result = solve(line2, x);
    printIntArr(result);
}

void printIntArr(int[] arr) {
    writeln(arr.map!(to!string).join(" "));
}

unittest {
    int[] arr = [1, 2, 3];
    int x = 3;

    int[] expected = [1, 2];
    int[] actual = solve(arr, 3);

    assert(
        actual == expected,
        "expected=" ~ expected.to!string ~
            ", actual=" ~ actual.to!string
    );
}
