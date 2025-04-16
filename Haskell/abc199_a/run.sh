#!/bin/bash

ghc -o a.out Main.hs

oj test -c "./a.out"
