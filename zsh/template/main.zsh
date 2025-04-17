#!/bin/zsh
read a b c
sum=$((a * a + b * b))
sq=$((c * c))

if (( sum < sq )); then
  echo Yes
else
  echo No
fi
