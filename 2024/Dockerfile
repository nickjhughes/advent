FROM alpine:latest AS builder

ARG DAY="01"
ENV DAY=${DAY}

RUN apk add --no-cache gcc gdb

COPY . .

RUN as -gstabs -o day${DAY}.o -s day${DAY}.asm
RUN ld -o day${DAY} day${DAY}.o

# Debug:
# ENTRYPOINT gdb ./day${DAY}
# Run:
ENTRYPOINT ./day${DAY}
