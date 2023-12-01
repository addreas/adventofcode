#!/bin/bash

YEAR=2023
DAY=$1

mkdir -p $YEAR/day/$DAY/

curl -sSL -v -H "Cookie: session=$SESSION" https://adventofcode.com/$YEAR/day/$DAY/input > $YEAR/day/$DAY/input
