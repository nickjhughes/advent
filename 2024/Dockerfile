FROM alpine:3.20.3 AS builder

RUN apk add --no-cache gcc gdb

ARG DAY="01"
ENV DAY=${DAY}

COPY . .

RUN as -gstabs -g day${DAY}.asm -o day${DAY}.o
RUN ld day${DAY}.o -o day${DAY}

ENTRYPOINT ./day${DAY}
