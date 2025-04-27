#!/bin/bash

set -e

if [ $# -ne 1 ]; then
    echo "Usage: ./setup.sh ABC123_a"
    exit 1
fi

TASK_ID=$1
CONTEST_ID=${TASK_ID%_*}
PROBLEM_SUFFIX=${TASK_ID##*_}

CONTEST_ID_LOWER=$(echo "$CONTEST_ID" | tr '[:upper:]' '[:lower:]')
PROBLEM_SUFFIX_LOWER=$(echo "$PROBLEM_SUFFIX" | tr '[:upper:]' '[:lower:]')

oj d "https://atcoder.jp/contests/$CONTEST_ID_LOWER/tasks/${CONTEST_ID_LOWER}_${PROBLEM_SUFFIX_LOWER}"

# mkdir -p "$TASK_ID"
mix new "$TASK_ID" --app main

# move sample data
if [ -d test ]; then
    mv ./test ./sample
    mv ./sample "$TASK_ID/"
else 
    echo "Error: test directory not found. oj d failed?"
    exit 1
fi

# copy template file
if [ -f ./template/main.ex ]; then
    cp ./template/main.ex "./$TASK_ID/lib/main.ex"
else
    echo "Error: template/main.ex not found"
    exit 1
fi

# copy shell script to execute test 
if [ -f ./template/run.sh ]; then
    cp ./template/run.sh "$TASK_ID/"
else
    echo "Error: template/run.sh not found"
    exit 1
fi

# change current directory to project directory
cd "$TASK_ID"
