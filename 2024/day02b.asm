.code64

.section .rodata
filename: .asciz "input02"
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

# Allocate memory (at %r13) to store array of input numbers
call mmap_memory
movq %rax, %r13

# Start at beginning of input
movq %r14, %r11

# The number of safe reports
xor %rax, %rax

start_of_line:
  # Index of number in line
  xor %r9, %r9

# Parse each number
parse_number:
  # Store the number in %dl, so zero it out
  xor %rdx, %rdx

digit_loop:
  # Load char into register
  xor %rcx, %rcx
  movb (%r11), %cl
  inc %r11
  
  # Check if char is null, meaning we've reached end of file
  cmp $0, %cl
  je end_of_input

  # Convert from ASCII by subtracting $48
  sub $48, %cl

  # Multiply existing number by 10, and add in new digit
  imul $10, %edx
  add %cl, %dl

  # If next char is a space or newline, we're done with this number
  movb (%r11), %cl
  cmp $32, %cl # Space
  je finished_number
  cmp $10, %cl # Newline
  jne digit_loop

finished_number:
  # Store in array
  movb %dl, (%r13,%r9,1)
  inc %r9

  # Move to next line if next char is a newline
  movb (%r11), %cl
  inc %r11
  cmp $10, %cl # Newline
  jne parse_number

end_of_line:
  # Analyse differences between numbers in line
  dec %r9 # So we run to n-1 rather than n
  
  # Index of the number we're checking
  xor %r8, %r8

  # Store sign bit in %r15
  xor %r15, %r15

compare_pair:
  xor %rcx, %rcx
  movb (%r13,%r8,1), %cl
  inc %r8
  xor %rdx, %rdx
  movb (%r13,%r8,1), %dl

  # Difference between numbers
  xor %r10, %r10
  sub %cl, %dl # dl -= cl
  mov %dl, %r10b

  # Check size of difference
  # Take absolute value of difference abs(%r10b)
  xor %r12, %r12
  mov %r10b, %r12b
  neg %r10b
  cmovl %r12d, %r10d
  # If difference is 0 or >3, skip to the next line, as this one is invalid
  cmp $0, %r10b
  je skip_line
  cmp $3, %r10b
  jg skip_line

  # Check sign of difference
  # Store sign bit
  shr $7, %rdx
  # Skip check if we're on the first pair
  cmp $1, %r8
  je finished_compare

  cmp %rdx, %r15
  jne skip_line

finished_compare:
  mov %rdx, %r15

  cmp %r8, %r9
  jne compare_pair

  # If we've gotten to this point, the line must be valid/safe
  inc %rax
  jmp start_of_line

skip_line:
  # Remove each level, one at a time, and repeat the check

  # Level index to skip
  xor %r14, %r14
  sub $1, %r14

removal_loop:
  cmp %r9, %r14
  je start_of_line
  inc %r14

  # Index of the number we're checking
  xor %r8, %r8

  # Store sign bit in %r15
  xor %r15, %r15

compare_pair2:
  xor %rcx, %rcx

  # Skip level %r8 = %r14
  cmp %r14, %r8
  jne load_a
  inc %r8
load_a:
  movb (%r13,%r8,1), %cl
  inc %r8
  # Skip level %r8 = %r14
  cmp %r14, %r8
  jne load_b
  # We're done if we're skipping the last number here
  cmp %r14, %r9
  je finished_compare2
  inc %r8
load_b:
  xor %rdx, %rdx
  movb (%r13,%r8,1), %dl

  # Difference between numbers
  xor %r10, %r10
  sub %cl, %dl # dl -= cl
  mov %dl, %r10b

  # Check size of difference
  # Take absolute value of difference abs(%r10b)
  xor %r12, %r12
  mov %r10b, %r12b
  neg %r10b
  cmovl %r12d, %r10d
  # If difference is 0 or >3, skip to the next line, as this one is invalid
  cmp $0, %r10b
  je removal_loop
  cmp $3, %r10b
  jg removal_loop

  # Check sign of difference
  # Store sign bit
  shr $7, %rdx
  # Skip check if we're on the first pair
  cmp $1, %r8
  je finished_compare2
  # Also skip check if we're on the second pair after skipping the first number
  # In which case, %r8 = 2, and %r14 = 0
  cmp $0, %r14
  jne skip_skip
  cmp $2, %r8
  je finished_compare2

skip_skip:
  cmp %rdx, %r15
  jne removal_loop

finished_compare2:
  mov %rdx, %r15

  cmp %r8, %r9
  jne compare_pair2

  # If we've gotten to this point, the line must be valid/safe
  inc %rax
  jmp start_of_line

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

  pop %r8
  pop %rdi
  pop %rcx
  pop %rdx
  pop %rax
  ret

.include "syscalls.asm"
