#!/usr/bin/env bash

if [[ "${#}" != 1 ]]; then
  echo "usage: ${0} DAY_NUMBER"
  exit 1
fi

day="${1}"
cp -i template.rs src/bin/day-$day.rs
touch src/inputs/input-$day
sed -i "s/DAY_NUM/$day/" src/bin/day-$day.rs