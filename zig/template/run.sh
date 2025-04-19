#!/bin/bash

zig build-exe main.zig -O ReleaseFast 

oj test -c "./main"

rm ./main ./main.o


