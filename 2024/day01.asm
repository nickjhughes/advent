.code64

.section .rodata
input: .ascii "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet\n"
.set input_end, .
.set input_len, (input_end - input)

.section .data
output: .ascii "        \n"
.set output_len, (. - output)

.section .text

.global _start
_start:

# Initialize sum to 0
movq $0, %rax

# Start at beginning of input
lea input, %r11

start_of_line:
  # Reset variables
  movq $0, %r8  # Whether we've seen the first digit
  movq $0, %r9  # The first digit
  movq $0, %r10 # The last digit

# Loop over each character
char_loop:
  # Load char into register
  movb (%r11), %cl
  movb %cl, (output)
  
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
  # If we're at the end of the input, jump to end_of_input, otherwise continue
  cmp $input_end, %r11
  je end_of_input
  jmp char_loop

end_of_line:
  # Construct number from the two digits and add to running sum
  imul $10, %r9
  add %r10, %r9
  add %r9, %rax

  inc %r11
  cmp $input_end, %r11
  je end_of_input
  jmp start_of_line

end_of_input:
  # TODO: Print out sum as answer
  #       For just print the number of lines
  call write_result_to_output
  call print_output
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
  movq $0, %r8
  movq $10, %rcx
count_digit:
  inc %r8
  movq $0, %rdx
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
  movq $0, %rdx
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

# Print the string at (output) to stdout
print_output:
  push %rax
  push %rdi
  push %rsi
  push %rdi
  push %r11
  push %rcx
  movq $1, %rax
  movq $1, %rdi
  leaq output, %rsi
  movq $output_len, %rdx
  syscall
  pop %rcx
  pop %r11
  pop %rdi
  pop %rsi
  pop %rdi
  pop %rax
  ret

exit:
  mov $60, %rax
  mov $0, %rdi 
  syscall
