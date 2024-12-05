.code64

.section .rodata
filename: .asciz "input04"
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

# Allocate memory (at %r13) to store array of chars
call mmap_memory
movq %rax, %r13

# Start at beginning of input
movq %r14, %r11

# Store result in %rax
xor %rax, %rax

# Load the input into an array at %r13, and store the size of the square array in %r9
xor %r9, %r9 # Size
xor %r10, %r10 # Array index

parse_char:
  # Load char into %cl
  xor %rcx, %rcx
  movb (%r11), %cl
  inc %r11

  cmp $0, %cl # Null
  je end_of_input

  cmp $10, %cl # Newline
  je end_of_line

  # Copy char to array and increment index
  movb %cl, (%r13,%r10,1)
  inc %r10
  jmp parse_char

end_of_line:
  # Increment row count
  inc %r9
  jmp parse_char

end_of_input:
  # For each char that is an 'X', check for XMAS in each of the 8 directions
  xor %r10, %r10

check_char:
  movb (%r13,%r10,1), %cl
  inc %r10

  cmp $0, %cl # Null
  je finished

  cmp $88, %cl # 'X'
  jne check_char

  dec %r10

  # left-to-right
  mov $0, %r14
  mov $1, %r15
  call find_xmas

  # right-to-left
  mov $0, %r14
  mov $-1, %r15
  call find_xmas
   
  # top-to-bottom
  mov $1, %r14
  mov $0, %r15
  call find_xmas
   
  # bottom-to-top
  mov $-1, %r14
  mov $0, %r15
  call find_xmas

  # top-left-to-bottom-right
  mov $1, %r14
  mov $1, %r15
  call find_xmas

  # top-right-to-bottom-left
  mov $1, %r14
  mov $-1, %r15
  call find_xmas
   
  # bottom-left-to-top-right
  mov $-1, %r14
  mov $1, %r15
  call find_xmas
   
  # bottom-right-to-to-left
  mov $-1, %r14
  mov $-1, %r15
  call find_xmas

  inc %r10

  jmp check_char

finished:
  # Print result
  call write_result_to_output
  lea output, %rdi
  movq $output_len, %rsi
  call print

  jmp exit

# Attempt to find XMAS in the square array at %r13 of size %r9, starting at index %r10,
# with a difference between (row,col) of (%r14,%r15). Increments %rax if found.
find_xmas:
  push %r10

  # Store current (row,col) in (%r11,%r12)
  # Calculate starting row/col from index %r10 by dividing by %r9
  # idiv divides %rdx:%rax, so clear %rdx and move %r10 to %rax
  push %rax
  xor %rdx, %rdx
  mov %r10, %rax
  idiv %r9
  mov %rax, %r11 # row (quotient of division)
  mov %rdx, %r12 # col (remainder of division)
  pop %rax

  # Calculate index, and check if char is as expected
  mov %r11, %r10
  imul %r9, %r10
  add %r12, %r10
  movb (%r13,%r10,1), %cl
  cmp $88, %cl # X
  jne no_xmas
  # Increment row/col as appropriate, and then bounds check
  add %r14, %r11
  cmp %r9, %r11
  jge no_xmas
  cmp $0, %r11
  jl no_xmas
  add %r15, %r12
  cmp %r9, %r12
  jge no_xmas
  cmp $0, %r12
  jl no_xmas

  # Calculate index, and check if char is as expected
  mov %r11, %r10
  imul %r9, %r10
  add %r12, %r10
  movb (%r13,%r10,1), %cl
  cmp $77, %cl # M
  jne no_xmas
  # Increment row/col as appropriate, and then bounds check
  add %r14, %r11
  cmp %r9, %r11
  jge no_xmas
  cmp $0, %r11
  jl no_xmas
  add %r15, %r12
  cmp %r9, %r12
  jge no_xmas
  cmp $0, %r12
  jl no_xmas

  # Calculate index, and check if char is as expected
  mov %r11, %r10
  imul %r9, %r10
  add %r12, %r10
  movb (%r13,%r10,1), %cl
  cmp $65, %cl # A
  jne no_xmas
  # Increment row/col as appropriate, and then bounds check
  add %r14, %r11
  cmp %r9, %r11
  jge no_xmas
  cmp $0, %r11
  jl no_xmas
  add %r15, %r12
  cmp %r9, %r12
  jge no_xmas
  cmp $0, %r12
  jl no_xmas

  # Calculate index, and check if char is as expected
  mov %r11, %r10
  imul %r9, %r10
  add %r12, %r10
  movb (%r13,%r10,1), %cl
  cmp $83, %cl # S
  jne no_xmas
  
  # If we get here, we found XMAS
  inc %rax
  
no_xmas:
  pop %r10
  ret

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
