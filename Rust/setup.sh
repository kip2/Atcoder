#!/bin/bash

set -e  # エラーが出たら即終了

if [ $# -ne 1 ]; then
    echo "Usage: ./setup.sh ABC123_a"
    exit 1
fi

TASK_ID=$1
CONTEST_ID=${TASK_ID%_*}
PROBLEM_SUFFIX=${TASK_ID##*_}

CONTEST_ID_LOWER=$(echo "$CONTEST_ID" | tr '[:upper:]' '[:lower:]')
PROBLEM_SUFFIX_LOWER=$(echo "$PROBLEM_SUFFIX" | tr '[:upper:]' '[:lower:]')

VSCODE_SETTINGS="../.vscode/settings.json"

if [ -d "$TASK_ID" ]; then
    echo "ディレクトリが既に存在しています。"
    exit 1
fi

# settings.json がなければ初期化
if [ ! -f "$VSCODE_SETTINGS" ]; then
    mkdir -p "$(dirname "$VSCODE_SETTINGS")"
    echo '{ "rust-analyzer.linkedProjects": [], "files.watcherExclude": { "**/target": true } }' > "$VSCODE_SETTINGS"
fi

# json形式をチェックする
error_output=$(mktemp)

if ! jq empty "$VSCODE_SETTINGS" 2> "$error_output"; then
    echo "❌ settings.json に構文エラーがあります"
    cat "$error_output"
    rm "$error_output"
    exit 1
fi

oj d "https://atcoder.jp/contests/$CONTEST_ID_LOWER/tasks/${CONTEST_ID_LOWER}_${PROBLEM_SUFFIX_LOWER}"

mkdir -p "$TASK_ID/src"

# Create Cargo.toml
cat > "$TASK_ID/Cargo.toml" <<EOF
[package]
name = "$TASK_ID"
version = "0.1.0"
edition = "2021"

[dependencies]
EOF

# Copy Rust template (assumes template/main.rs exists)
cp ./template/main.rs "$TASK_ID/src/main.rs"

# Copy run.sh (test runner)
cp ./template/run.sh "$TASK_ID/run.sh"
chmod +x "$TASK_ID/run.sh"

# Move test case directory
mv ./test "$TASK_ID/"

# === VSCode Rust-analyzer settings 追記処理 ===

VSCODE_SETTINGS="../.vscode/settings.json"  # Rust-oj の一つ上のルートを想定
PROJECT_PATH="Rust/$TASK_ID/Cargo.toml"


# jq を使って追記（重複チェックあり）
if ! grep -q "$PROJECT_PATH" "$VSCODE_SETTINGS"; then
    tmpfile=$(mktemp)
    jq --arg path "$PROJECT_PATH" '
        .["rust-analyzer.linkedProjects"] += [$path]
    ' "$VSCODE_SETTINGS" > "$tmpfile" && mv "$tmpfile" "$VSCODE_SETTINGS"
    echo "✅ $PROJECT_PATH を .vscode/settings.json に追加しました"
else
    echo "ℹ️ $PROJECT_PATH はすでに .vscode/settings.json に含まれています"
fi
