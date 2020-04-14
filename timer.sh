#!/usr/bin/env bash

set -Eeu -o pipefail

OUTFILE=benchmarks.md
cargo build --release
mapfile -d '' days < <(find . -maxdepth 1 -type d -name 'day*' -printf '%P\0' | sort -z -V)
echo -e "# $(date)\n" > "$OUTFILE"

# create table including its headers
hyperfine --style=none --warmup 5 --export-markdown /dev/stdout target/release/day1 >> "$OUTFILE"
# pop day1 from array
days=("${days[@]:1}")

for day in "${days[@]}"; do
  echo "$day"
  start=$SECONDS
  cargo run --release --package "$day"
  end=$SECONDS
  diff_ms=$(( ("$end" - "$start") * 1000 ))
  if [ "$diff_ms" -lt 10000 ]; then
    hyperfine --style=none --warmup 3 --export-markdown /dev/stdout target/release/"$day" | tail -n 1 >> "$OUTFILE"
  else
    echo "| "'`'"target/release/$day"'`'" | $diff_ms (single run) | -- | -- | -- |" >> "$OUTFILE"
  fi
done
