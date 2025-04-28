#!/bin/bash

luac -o luac.out main.lua
oj test -c "lua luac.out"

rm ./luac.out

