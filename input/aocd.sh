#!/bin/bash

if [ $# != 2 ]; then
    echo "Usage: ./aocd.sh <session> <year>"
    exit 1
fi

base_url="https://adventofcode.com"
session=$1
year=$2

status_code=$(curl --silent --output /dev/null --write-out %{http_code} \
    --cookie "session=$session" "$base_url/$year/auth/login")
if [ $status_code == 404 ]; then
    echo "error: invalid year"
    exit 1
fi
if [ $status_code != 302 ]; then
    echo "error: invalid session cookie"
    exit 1
fi

for day in {1..25}; do
    day_padded=$(printf "%02d" $day)
    curl --silent --show-error --cookie "session=$session" \
        --output "day$day_padded.txt" "$base_url/$year/day/$day/input" &
done
wait
