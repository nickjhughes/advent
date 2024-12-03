.code64

.section .rodata
filename: .asciz "input03"
input_file_open_error: .ascii "Failed to open input file\n"
.set input_file_open_error_len, (. - input_file_open_error)

.section .data
output: .ascii "                                \n"
.set output_len, (. - output)

.section .text

.global _start
_start:

# Open and mmap null-terminated input file to address (%r14)
call open_input_file
movq %rax, %rdi
call mmap_input_file
movq %rax, %r14

# Start at beginning of input
movq %r14, %r11

# Store result in %rax
xor %rax, %rax

# Store current char in %rcx
xor %rcx, %rcx

# Whether mul instruction are enabled or not
movq $1, %r15

parse:
  # Load char into %cl
  movb (%r11), %cl
  inc %r11

  # If null, we've reached the end of the input
  cmp $0, %cl
  je end_of_input

  # If not a d or an m, we can move to the next char and try again
  cmp $100, %cl # d
  je parse_do
  cmp $109, %cl # m
  je parse_mul
  jmp parse

parse_do:
  # Try to parse a do() instruction, knowing we've already seen a 'd'

  movb (%r11), %cl
  inc %r11

  cmp $111, %cl # o
  jne parse
  movb (%r11), %cl
  inc %r11

  cmp $40, %cl # (
  jne parse_dont
  movb (%r11), %cl
  inc %r11

  cmp $41, %cl # )
  jne parse

  # Enable mul instructions
  movq $1, %r15
  jmp parse

parse_dont:
  # Try to parse a don't() instruction, knowing we've already seen "do"

  cmp $110, %cl # n
  jne parse
  movb (%r11), %cl
  inc %r11

  cmp $39, %cl # '
  jne parse
  movb (%r11), %cl
  inc %r11

  cmp $116, %cl # t
  jne parse
  movb (%r11), %cl
  inc %r11

  cmp $40, %cl # (
  jne parse_dont
  movb (%r11), %cl
  inc %r11

  cmp $41, %cl # )
  jne parse

  # Disable mul instructions
  xor %r15, %r15
  jmp parse  

parse_mul:
  # Try to parse a mul(x,y) instruction, knowing we've already seen an 'm'

  # Store operands in %r12 and %r13
  xor %r12, %r12
  xor %r13, %r13

  # Load char into %cl
  movb (%r11), %cl
  inc %r11

  # Try to parse "ul("
  cmp $117, %cl # u
  jne parse
  movb (%r11), %cl
  inc %r11

  cmp $108, %cl # l
  jne parse
  movb (%r11), %cl
  inc %r11

  cmp $40, %cl # (
  jne parse
  movb (%r11), %cl
  inc %r11

  # Try to parse a 1-3 digit number followed by a comma
  # If < '0' or > '9', move on
  cmp $48, %cl
  jl parse
  cmp $57, %cl
  jg parse
  # Otherwise convert from ASCII and add to number
  sub $48, %cl
  mov %rcx, %r12
  movb (%r11), %cl
  inc %r11
  
  cmp $44, %cl # Comma
  je parse_number_2
  # If < '0' or > '9', move on
  cmp $48, %cl
  jl parse
  cmp $57, %cl
  jg parse
  # Otherwise convert from ASCII and add to number
  sub $48, %cl
  imul $10, %r12
  add %rcx, %r12
  movb (%r11), %cl
  inc %r11

  cmp $44, %cl # Comma
  je parse_number_2
  # If < '0' or > '9', move on
  cmp $48, %cl
  jl parse
  cmp $57, %cl
  jg parse
  # Otherwise convert from ASCII and add to number
  sub $48, %cl
  imul $10, %r12
  add %rcx, %r12
  movb (%r11), %cl
  inc %r11

  # At this point, expecting a comma for sure
  cmp $44, %cl # Comma
  jne parse

parse_number_2:
  movb (%r11), %cl
  inc %r11

  # Try to parse a 1-3 digit number followed by a ')'
  # If < '0' or > '9', move on
  cmp $48, %cl
  jl parse
  cmp $57, %cl
  jg parse
  # Otherwise convert from ASCII and add to number
  sub $48, %cl
  mov %rcx, %r13
  movb (%r11), %cl
  inc %r11
  
  cmp $41, %cl # Close parenthesis
  je finish_mul
  # If < '0' or > '9', move on
  cmp $48, %cl
  jl parse
  cmp $57, %cl
  jg parse
  # Otherwise convert from ASCII and add to number
  sub $48, %cl
  imul $10, %r13
  add %rcx, %r13
  movb (%r11), %cl
  inc %r11

  cmp $41, %cl # Close parenthesis
  je finish_mul
  # If < '0' or > '9', move on
  cmp $48, %cl
  jl parse
  cmp $57, %cl
  jg parse
  # Otherwise convert from ASCII and add to number
  sub $48, %cl
  imul $10, %r13
  add %rcx, %r13
  movb (%r11), %cl
  inc %r11

  # At this point, expecting a closing parenthesis for sure
  cmp $41, %cl # Comma
  jne parse

finish_mul:
  # Perform multiplication and add to total, if instructions are enabled
  cmp $0, %r15
  je parse
  imul %r12, %r13
  add %r13, %rax

  jmp parse

end_of_input:
  # Print result
  call write_result_to_output
  lea output, %rdi
  movq $output_len, %rsi
  call print

  jmp exit

# Write the number stored in %rax as an ASCII string into (output)
write_result_to_output:
  push %rax
  push %rdx
  push %rcx
  push %rdi
  push %r8

  push %rax

  # First determine the number of decimal digits in the number, stored in %r8
  xor %r8, %r8
  movq $10, %rcx
count_digit:
  inc %r8
  xor %rdx, %rdx
  divq %rcx
  cmp $0, %rax
  jne count_digit

  pop %rax
 
  # %rsi is passed to the print subroutine as the length of the string
  mov %r8, %rsi
  inc %rsi # Add one for newline

  # Now read out digits by dividing by 10 and taking remainder, writing digits in reverse
  # %rdi is index into output string
write_digit_start:
  lea output, %rdi
  add %r8, %rdi
write_digit:
  decq %rdi
  xor %rdx, %rdx
  divq %rcx
  add $48, %rdx
  movb %dl, (%rdi)
  cmp $output, %rdi
  jne write_digit

  movb $10, (%rdi,%r8,1) # End with a newline

  pop %r8
  pop %rdi
  pop %rcx
  pop %rdx
  pop %rax
  ret

.include "syscalls.asm"
