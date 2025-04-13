import std.stdio;
import std.algorithm;
import std.array;
import std.conv;

string solve(int a, int b, int c) {
    if (a * a + b * b < c * c) {
        return "Yes";
    } else {
        return "No";
    }
}

void main() {
    auto nums = readln().split().map!(to!int).array;
    writeln(solve(nums[0], nums[1], nums[2]));
}
