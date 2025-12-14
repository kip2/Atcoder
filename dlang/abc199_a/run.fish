#!/usr/bin/env fish

dmd -of=main main.d

oj test -c "./main"

rm ./main.o
rm ./main

