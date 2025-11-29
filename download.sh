#!/usr/bin/env bash
set -e

if [[ "$1" == "-n" ]]; then
  readonly DRYRUN="echo "
  shift
else
  readonly DRYRUN=""
fi

path="${PWD}"
if [[ -z "${1}" ]]; then
  readonly day="$(basename ${PWD})"
  readonly day_num="${day#day}"
  path="${path%/*}"
else
  readonly day_num=${1}
  readonly day="day${day_num}"
fi
readonly year="$(basename ${path})"

if (( $day_num < 32 )) \
  && [[ "$day" =~ ^day[0-9]+$ ]] \
  && [[ "$day_num" =~ ^[0-9]+$ ]]  \
  &&  [[ "$year" =~ ^[0-9]+$  ]]; then
  echo "Generating stuff for day=$day, day_num=$day_num, year=$year..."
else
  echo "Running from the wrong directory: pwd=$PWD, day=$day, day_num=$day_num, year=$year"
  exit 1
fi

readonly base_path="$(realpath $(dirname $0))"
readonly project_path="$base_path/$year/$day"

if [[ ! -d ${project_path} ]]; then
  ${DRYRUN} cargo generate \
    --path $base_path/template \
    --name ${day} \
    --define aoc_year=${year} \
    --define aoc_day=${day_num}
fi

${DRYRUN}aoc download \
  --year ${year} \
  --day ${day_num} \
  --input-file $project_path/src/data/input \
  --puzzle-file $project_path/puzzle.md \
  --overwrite
