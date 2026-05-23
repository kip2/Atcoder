module abc083_b.main;

import std.stdio;
import std.string;
import std.algorithm;
import std.array;
import std.conv;

int solve(int n, int a, int b) {
    int total = 0;
    foreach (i; 1 .. n + 1) {
        int ds = digitSum(i);
        if (a <= ds && ds <= b)
            total += i;
    }
    return total;
}

int digitSum(int x) {
    int sum = 0;
    while (0 < x) {
        sum += x % 10;
        x /= 10;
    }

    return sum;
}

void main() {
    auto nab = readln.split.map!(to!int).array;
    int n = nab[0];
    int a = nab[1];
    int b = nab[2];

    int total = solve(n, a, b);
    writeln(total);
}
