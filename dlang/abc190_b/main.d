module dlang.abc190_b.main;

import std.stdio;
import std.string;
import std.algorithm;
import std.array;
import std.conv;

int[] getInputOfIntArray() {
    return readln().split().map!(to!int).array;
}

void main() {
    int[] line = getInputOfIntArray();

    int n = line[0];
    int s = line[1];
    int d = line[2];

    string result = solve(n, s, d);

    writeln(result);
}

string solve(int n, int s, int d) {
    for (int i = 0; i < n; i++) {
        int[] line = getInputOfIntArray();
        int x = line[0];
        int y = line[1];

        if (x < s && y > d) {
            return "Yes";
        }
    }
    return "No";
}
