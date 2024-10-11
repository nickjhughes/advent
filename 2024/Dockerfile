FROM alpine:latest AS builder

ARG DAY="01"
ENV DAY=${DAY}

RUN apk add --no-cache gcc

COPY . .

RUN as -o day${DAY}.o -s day${DAY}.asm
RUN ld -o day${DAY} day${DAY}.o

ENTRYPOINT ./day${DAY}
