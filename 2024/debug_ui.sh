#!/bin/bash

echo "Debugging day $1"
docker build -t aoc-2024 --platform linux/amd64 --build-arg DAY="$1" -f Dockerfile-gdbfrontend . && docker run -p 5550:5550 -it aoc-2024
