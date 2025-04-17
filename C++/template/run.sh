#!/bin/bash

g++ -std=c++17 -O2 -Wall -o main main.cpp
oj test -c "./main"

rm ./main 



