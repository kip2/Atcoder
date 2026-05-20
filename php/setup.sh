#!/bin/bash

set -euo pipefail

if [ $# -ne 1 ]; then
    echo "Usage: ./setup.sh ABC123_a"
    exit 1
fi

TASK_ID=$1
CONTEST_ID=${TASK_ID%_*}
PROBLEM_SUFFIX=${TASK_ID##*_}

CONTEST_ID_LOWER=$(echo "$CONTEST_ID" | tr '[:upper:]' '[:lower:]')
PROBLEM_SUFFIX_LOWER=$(echo "$PROBLEM_SUFFIX" | tr '[:upper:]' '[:lower:]')

SCRIPT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
TASK_DIR="$SCRIPT_DIR/$TASK_ID"

if [ -e "$TASK_DIR" ]; then
    echo "ディレクトリが既に存在しています: $TASK_DIR"
    exit 1
fi

mkdir -p "$TASK_DIR"

# 失敗したら作りかけのディレクトリを片付ける
cleanup() {
    echo "セットアップに失敗しました。$TASK_DIR を削除します。"
    rm -rf "$TASK_DIR"
}
trap cleanup ERR

URL="https://atcoder.jp/contests/$CONTEST_ID_LOWER/tasks/${CONTEST_ID_LOWER}_${PROBLEM_SUFFIX_LOWER}"

oj d -d "$TASK_DIR/test" "$URL"

cp "$SCRIPT_DIR/template/main.php" "$TASK_DIR/main.php"
cp "$SCRIPT_DIR/template/run.sh" "$TASK_DIR/run.sh"

trap - ERR

echo "セットアップ完了: $TASK_DIR"
