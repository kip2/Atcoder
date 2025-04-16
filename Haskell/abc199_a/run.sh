#!/bin/bash

ghc -o a.out Main.hs

oj test -c "./a.out"

rm ./Main.hi ./Main.o ./a.out
