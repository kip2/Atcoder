#!/bin/bash

gcc main.c -o main

oj test -c "./main"

rm ./main