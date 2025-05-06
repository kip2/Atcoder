#!/bin/bash -e

gcc main.c -o main

./main < ./test/sample-1.in

rm ./main
