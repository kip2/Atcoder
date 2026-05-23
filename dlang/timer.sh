#!/bin/bash

set -euo pipefail

usage() {
    cat <<EOF
Usage: $(basename "$0") <duration>

  300       300秒
  5m        5分
  1h        1時間
  1h40m     1時間40分 (ABC本番)
  100m      100分 (= ABC本番)
EOF
    exit 1
}

[ $# -eq 1 ] || usage

input=$1
remaining=$input
seconds=0

# Xh
if [[ $remaining =~ ^([0-9]+)h(.*)$ ]]; then
    seconds=$((seconds + BASH_REMATCH[1] * 3600))
    remaining=${BASH_REMATCH[2]}
fi
# Xm
if [[ $remaining =~ ^([0-9]+)m(.*)$ ]]; then
    seconds=$((seconds + BASH_REMATCH[1] * 60))
    remaining=${BASH_REMATCH[2]}
fi
# Xs or plain number
if [[ $remaining =~ ^([0-9]+)s?$ ]]; then
    seconds=$((seconds + BASH_REMATCH[1]))
elif [ -n "$remaining" ]; then
    echo "Invalid format: $input" >&2
    usage
fi

if [ "$seconds" -le 0 ]; then
    echo "Invalid duration: $input" >&2
    usage
fi

format_time() {
    local t=$1
    local h=$((t / 3600))
    local m=$(((t % 3600) / 60))
    local s=$((t % 60))
    if [ "$h" -gt 0 ]; then
        printf "%02d:%02d:%02d" "$h" "$m" "$s"
    else
        printf "%02d:%02d" "$m" "$s"
    fi
}

# Ctrl+C で抜けたときに改行と「中断」表示
on_interrupt() {
    printf "\r\033[K中断しました\n"
    exit 130
}
trap on_interrupt INT

end_time=$(($(date +%s) + seconds))
echo "Timer started: $(format_time "$seconds")  (until $(date -d "@$end_time" '+%H:%M:%S'))"

while true; do
    now=$(date +%s)
    remain=$((end_time - now))
    if [ "$remain" -le 0 ]; then
        break
    fi
    printf "\r\033[K残り時間: %s" "$(format_time "$remain")"
    sleep 1
done

printf "\r\033[K残り時間: 00:00\n"
echo "TIMEOUT"

notify-send -u critical "AtCoder TIMEOUT" "コンテスト時間終了 ($input)" 2>/dev/null || true
