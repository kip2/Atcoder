module dlang.sandbox.main;

import std.stdio;
import std.array;
import std.algorithm;

void main() {
    int[] list = [3, 2, 5, 1, 4];

    list.sort!"a < b".array;
    writeln(list);
}

unittest {
    int[] list = [3, 2, 5, 1, 4];

    // ASC
    list.sort!"a < b".array;

    int[] expected = [1, 2, 3, 4, 5];

    assert(list == expected);

    // DESC
    list.sort!"a > b".array;

    expected = [5, 4, 3, 2, 1];

    assert(list == expected);
}
