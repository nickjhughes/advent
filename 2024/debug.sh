#!/bin/bash

echo "Debugging day $1"
docker build -t aoc-2024 --platform linux/amd64 --build-arg DAY="$1" . && docker run -it --entrypoint day$1 aoc-2024
