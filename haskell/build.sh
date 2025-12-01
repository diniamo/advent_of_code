#!/bin/sh
set -e
cd "$(dirname "$0")"

year="$1"
day="$2"
part="$3"
if [ "$#" -gt 3 ]; then
  subcommand="$4"
  shift 4
else
  shift 3
fi

input="input/$year/$day"
if [ ! -f "$input" ]; then
  directory="$(dirname "$input")"
  [ ! -d "$directory" ] && mkdir --parents "$directory"
  token="$(cat token)"
  curl --silent --header "Cookie: session=$token" \
    "https://adventofcode.com/$year/day/$day/input" > "$input"
fi

solution="$year/$day.hs"
src="$(mktemp --suffix='.hs')"
cat << EOF >> "$src"
{-# LANGUAGE QuasiQuotes #-}
import Text.RawString.QQ
EOF

import_end="$(awk '/^[[:blank:]]*import/{i=NR}END{print 0+i}' "$src")"
head -"$import_end" "$solution" >> "$src"

echo -n 'input = [r|' >> "$src"
cat "$input" >> "$src"
echo '|]' >> "$src"

tail +"$((import_end + 1))" "$solution" >> "$src"

echo "main = putStrLn . show . part$part . parse \$ input" >> "$src"

set +e
out="$(mktemp)"

build() {
  ghc "$src" -o "$out" \
    -no-keep-hi-files -no-keep-o-files \
    "$@"
}
case "$subcommand" in
  time)
    build -O2 "$@"
    hyperfine -Nr1 "$out"
    ;;
  bench)
    build -O2 "$@"
    hyperfine -Nw1 "$out"
    ;;
  *)
    build "$@"
    "$out"
esac

rm "$src" "$out" 2>/dev/null
