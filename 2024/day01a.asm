.code64

.section .rodata
filename: .asciz "input01"
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

# Initialize sum to 0
xor %rax, %rax

# Start at beginning of input
movq %r14, %r11

start_of_line:
  # Reset variables
  xor %r8, %r8   # Whether we've seen the first digit
  xor %r9, %r9   # The first digit
  xor %r10, %r10 # The last digit

# Loop over each character
char_loop:
  # Load char into register
  movb (%r11), %cl

  # Check if char is null, meaning we've reached end of file
  cmp $0, %cl
  je end_of_input

  # Check if char is a newline, and if so jump to end_of_line
  cmp $10, %cl
  je end_of_line

  # If < '0' or > '9', skip
  cmp $48, %cl
  jl next_char
  cmp $57, %cl
  jg next_char

  # Char is a digit, so convert from ASCII by subtracting $48
  sub $48, %cl

  # If we haven't seen the first digit on this line, this must be it
  cmp $1, %r8
  je not_first_digit
  movq $1, %r8
  movb %cl, %r9b
not_first_digit:
  # Store all digits in other slot, so value at the end of the line will be the last one
  movb %cl, %r10b

next_char:
  # Move to next char
  inc %r11
  jmp char_loop

end_of_line:
  # Construct number from the two digits and add to running sum
  imul $10, %r9
  add %r10, %r9
  add %r9, %rax

  # Move to next char
  inc %r11
  jmp start_of_line

end_of_input:
  # Print out sum as answer
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
  pop %rdx
  pop %rcx
  pop %rax
  ret

.include "syscalls.asm"
