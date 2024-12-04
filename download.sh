#!/usr/bin/env bash
set -e

readonly day_num=${1}
readonly year="$(basename ${PWD})"
readonly day="day${day_num}"

if [[ ! -d ${day} ]]; then
  cargo generate \
    --path ../template \
    --name ${day} \
    --define aoc_year=${year} \
    --define aoc_day=${day_num}
fi

aoc download \
  --year ${year} \
  --day ${day_num} \
  --input-file ${day}/src/data/input \
  --puzzle-file ${day}/puzzle.md \
  --overwrite
