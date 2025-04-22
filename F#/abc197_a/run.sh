#!/bin/bash

fsharpc --out:main.exe main.fs
oj test -c "mono main.exe"

rm ./main.exe FSharp.Core.dll


