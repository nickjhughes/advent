#!/bin/bash

echo "Running day $1"
docker build -t aoc-2024 --build-arg DAY="$1" --platform linux/amd64 . && docker run -it --platform linux/amd64 aoc-2024
