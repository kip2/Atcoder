#!/bin/bash

set -euo pipefail

if [ $# -ne 1 ]; then
    echo "Usage: ./test.sh abc083_b"
    exit 1
fi

SCRIPT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
TASK_DIR="$SCRIPT_DIR/$1"

if [ ! -f "$TASK_DIR/main.d" ]; then
    echo "見つかりません: $TASK_DIR/main.d"
    exit 1
fi

cd "$TASK_DIR"
dmd -unittest -run main.d
