#!/bin/bash

# set -e

if [ $# -ne 1 ]; then
    echo "Usage: ./setup.sh ABC123_a"
    exit 1
fi

TASK_ID=$1
CONTEST_ID=${TASK_ID%_*}
PROBLEM_SUFFIX=${TASK_ID##*_}

CONTEST_ID_LOWER=$(echo "$CONTEST_ID" | tr '[:upper:]' '[:lower:]')
PROBLEM_SUFFIX_LOWER=$(echo "$PROBLEM_SUFFIX" | tr '[:upper:]' '[:lower:]')

if [ -d "$TASK_ID" ]; then
    echo "ディレクトリが既に存在しています。"
    exit 1
fi


oj d "https://atcoder.jp/contests/$CONTEST_ID_LOWER/tasks/${CONTEST_ID_LOWER}_${PROBLEM_SUFFIX_LOWER}"

mkdir -p "$TASK_ID"

# copy clojure template file
cp ./template/main.zig "$TASK_ID/main.zig"

# copy shell script to execute test 
cp ./template/run.sh "$TASK_ID/"

# move sample data
mv ./test "$TASK_ID/"