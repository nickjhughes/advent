.code64

.section .rodata
filename: .asciz "input22"
input_file_open_error: .ascii "Failed to open input file\n"
.set input_file_open_error_len, (. - input_file_open_error)

.section .data
output: .ascii "                                "
.set output_len, (. - output)
.section .text

.global _start
_start:
  # Make space on stack for local variables
  movq %rsp, %rbp
  sub $16, %rsp

  # Open and mmap null-terminated input file to address
  call open_input_file
  movq %rax, %rdi
  call mmap_input_file
  movq %rax, (%rbp)   # Pointer to input

  # Parse input
  movq (%rbp), %r11   # Pointer into input
  xor %r14, %r14      # Answer
parse:
  movb (%r11), %cl
  cmp $0, %cl
  je end_of_input
  call parse_integer
  inc %r11 # Skip newline

  xor %r10, %r10
gen_count:
  call generate_secret_number
  inc %r10
  cmp $2000, %r10
  jne gen_count

  add %rax, %r14
  jmp parse

end_of_input:
  movq %r14, %rax
  call write_result_to_output
  lea output, %rdi
  call print
  jmp exit

# Apply the secret number generation process to %rax
generate_secret_number:
  movq %rax, %rbx
  shl $6, %rbx        # Multiply by 64
  xor %rbx, %rax      # Mix
  and $16777215, %rax # Prune
  movq %rax, %rbx
  shr $5, %rbx        # Divide by 32
  xor %rbx, %rax      # Mix
  and $16777215, %rax # Prune
  movq %rax, %rbx
  shl $11, %rbx       # Multiply by 2048
  xor %rbx, %rax      # Mix
  and $16777215, %rax # Prune
  ret

# Parse an integer at %r11 into %rax
parse_integer:
  xor %rax, %rax
  xor %rcx, %rcx
parse_digit:
  movb (%r11), %cl
  cmp $48, %cl # '0'
  jl parse_integer_done
  cmp $57, %cl # '9'
  jg parse_integer_done
  inc %r11
  sub $48, %cl
  imul $10, %rax
  add %rcx, %rax
  jmp parse_digit
parse_integer_done:
  ret

.include "common.asm"
.include "syscalls.asm"
