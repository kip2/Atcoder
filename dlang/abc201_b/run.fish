#!/usr/bin/env fish

dmd -of=main main.d

if test $status -ne 0 
    echo "[ERROR] Failed compile."
    exit 1;
end

oj test -c "./main"

rm ./main.o
rm ./main


