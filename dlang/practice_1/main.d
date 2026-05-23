import std.stdio;
import std.string;
import std.algorithm;
import std.array;
import std.conv;

void main() {
    int a = readln.chomp.to!int;
    auto ln = readln.split.map!(to!int).array;
    int b = ln[0];
    int c = ln[1];
    string s = readln.chomp;

    writeln(a + b + c, " ", s);
}
