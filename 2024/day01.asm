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

# Loop over each character
char_loop:
  # Load char into register
  movb (%r11), %cl
  movb %cl, (output)
  
  # Check if char is a newline, and if so jump to end_of_line
  cmp $10, %cl
  je end_of_line
  
  # Otherwise, check if char is a digit
  # TODO
  
next_char:
  # Move to next char
  inc %r11
  # If we're at the end of the input, jump to end_of_input, otherwise continue
  cmp $input_end, %r11
  je end_of_input
  jmp char_loop

end_of_line:
  # TODO: Construct number from the two digits and add to running sum
  #       For now just count the number of lines
  inc %rax
  jmp next_char

end_of_input:
  # TODO: Print out sum as answer
  #       For now just print out the number of lines, assuming it's less than 10
  add $48, %al
  movb %al, (output)
  call print_output
  jmp exit

print_output:
  push %rax
  push %rdi
  push %rsi
  push %rdi
  push %r11
  push %rcx
  mov $1, %rax
  mov $1, %rdi
  lea output, %rsi
  mov $output_len, %rdx
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
