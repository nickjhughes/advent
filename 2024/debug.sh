#!/bin/bash

echo "Debugging day $1"
docker build -t aoc-2024 --build-arg DAY="$1" --platform linux/amd64 . && docker run --cap-add=SYS_PTRACE --security-opt seccomp=unconfined -it --platform linux/amd64 --entrypoint sh aoc-2024
