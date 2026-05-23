#!/bin/bash

cd "$(dirname "$0")" || exit 1

dmd -of=main main.d && oj test -c "./main"

rm -f ./main.o ./main
