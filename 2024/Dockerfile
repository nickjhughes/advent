FROM alpine:3.20.3 AS builder

ARG DAY="01"
ENV DAY=${DAY}

RUN apk add --no-cache gcc gdb

COPY . .

RUN as -gstabs -g day${DAY}.asm -o day${DAY}.o
RUN ld day${DAY}.o -o day${DAY}

# Debug:
# ENTRYPOINT gdb ./day${DAY}
# Run:
ENTRYPOINT ./day${DAY}
