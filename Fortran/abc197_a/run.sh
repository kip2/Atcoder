#!/bin/bash

gfortran -O2 -std=f2008 -o a.out main.f90
oj test -c "./a.out"

rm ./a.out