module dlang.abc201_b.main;

import std.stdio;
import std.string;
import std.algorithm;
import std.algorithm.mutation : swap;
import std.array;
import std.conv;

int getInt() {
    return parseInt(readln());
}

int parseInt(string s) {
    return s.strip.to!int;
}

struct Mountain {
    string name;
    int height;

    this(string name, int height) {
        this.name = name;
        this.height = height;
    }
}

void main() {
    int n = getInt();

    Mountain[] mountains = new Mountain[n];

    for (int i = 0; i < n; i++) {
        string[] line = readln().split();
        string s = line[0];
        int t = line[1].to!int;

        mountains[i] = Mountain(s, t);
    }

    auto sortedMountains = sortMountainDesc(mountains);

    writeln(sortedMountains[1].name);

}

Mountain[] sortMountainDesc(Mountain[] list) {
    return list.sort!"a.height > b.height".array;
}
