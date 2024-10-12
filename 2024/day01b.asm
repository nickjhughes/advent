.code64

.section .rodata
filename: .asciz "input01"
input_file_open_error: .ascii "Failed to open input file\n"
.set input_file_open_error_len, (. - input_file_open_error)

newline_debug: .ascii "Newline\n";
.set newline_debug_len, (. - newline_debug)

one: .asciz "one\0\0"
two: .asciz "two\0\0"
three: .asciz "three"
four: .asciz "four\0"
five: .asciz "five\0"
six: .asciz "six\0\0"
seven: .asciz "seven"
eight: .asciz "eight"
nine: .asciz "nine\0"

.section .data
output: .ascii "                                \n"
.set output_len, (. - output)
.set output_end, (output + output_len - 1)

.section .text

.global _start
_start:

call open_input_file
movq %rax, %r15
# %r15 now contains file descriptor
call mmap_input_file
movq %rax, %r14
# %r14 now contains addr of the memory-mapped input file
# we detect the end of the file by looking for a null byte

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
  movb %cl, (output)

  # Check if char is null, meaning we've reached end of file
  cmp $0, %cl
  je end_of_input

  # Check if char is a newline, and if so jump to end_of_line
  cmp $10, %cl
  je end_of_line

  # Check if this is the start of a spelled-out number
  # If so, this will store ethe number as an ASCII digit in %cl
  # (so the following code can be the same as if it were an ASCII digit)
  call check_number

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

  inc %r11
  jmp start_of_line

end_of_input:
  # Print out sum as answer
  call write_result_to_output
  lea output, %rsi
  movq $output_len, %rdx
  call print
  jmp exit

check_number:
  lea one, %rdi
  call match_string
  cmp $1, %r15
  jne check_two
  movb $49, %cl
  ret
check_two:
  lea two, %rdi
  call match_string
  cmp $1, %r15
  jne check_three
  movb $50, %cl
  ret
check_three:
  lea three, %rdi
  call match_string
  cmp $1, %r15
  jne check_four
  movb $51, %cl
  ret
check_four:
  lea four, %rdi
  call match_string
  cmp $1, %r15
  jne check_five
  movb $52, %cl
  ret
check_five:
  lea five, %rdi
  call match_string
  cmp $1, %r15
  jne check_six
  movb $53, %cl
  ret
check_six:
  lea six, %rdi
  call match_string
  cmp $1, %r15
  jne check_seven
  movb $54, %cl
  ret
check_seven:
  lea seven, %rdi
  call match_string
  cmp $1, %r15
  jne check_eight
  movb $55, %cl
  ret
check_eight:
  lea eight, %rdi
  call match_string
  cmp $1, %r15
  jne check_nine
  movb $56, %cl
  ret
check_nine:
  lea nine, %rdi
  call match_string
  cmp $1, %r15
  jne check_done
  movb $57, %cl
check_done:
  ret

# Compare the bytes starting at %r11 with the null-terminated string pointed to by %rdi
# If true, set %r15 to 1, otherwise 0
match_string:
  push %rdx
  push %rcx
  push %r11
  push %rdi

  xor %r15, %r15

match_string_loop:
  movb (%r11), %cl
  movb (%rdi), %dl
  # If we've reached the end of the string, then we've matched
  cmp $0, %dl
  je match_string_match
  # If we've reacthed the end of the input, then we're done
  cmp $0, %cl
  je match_string_end
  # Otherwise, compare and end if not equal, or continue if equal
  cmp %dl, %cl
  jne match_string_end
  inc %rdi
  inc %r11
  jmp match_string_loop

match_string_match:
  movq $1, %r15

match_string_end:
  pop %rdi
  pop %r11
  pop %rcx
  pop %rdx
  ret

clear_output:
  push %rdi
  push %rax
  lea output, %rdi
  movb $32, %al
clear_output_loop:
  movb %al, (%rdi)
  inc %rdi
  cmp $output_end, %rdi
  jne clear_output_loop
  pop %rax
  pop %rdi
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

# Print the string at (%rsi) with length %rdx to stdout
print:
  push %rax
  push %rdi
  push %r11
  push %rcx
  movq $1, %rax
  movq $1, %rdi
  syscall
  pop %rcx
  pop %r11
  pop %rdi
  pop %rax
  ret

open_input_file:
  push %rdi
  push %rsi
  push %rdx
  push %r11
  push %rcx
  movq $2, %rax
  leaq filename, %rdi
  movq $0, %rsi # flags = O_RDONLY
  syscall
  pop %rcx
  pop %r11
  pop %rdx
  pop %rsi
  pop %rdi

  # If open failed, print error and exit
  cmp $0, %rax
  jge open_input_file_success
  lea input_file_open_error, %rsi
  movq $input_file_open_error_len, %rdx
  call print
  jmp exit

open_input_file_success:
  ret

mmap_input_file:
  push %rdi
  push %rsi
  push %rdx
  push %r10
  push %r8
  push %r9
  push %r11
  push %rcx
  movq $9, %rax
  movq $0, %rdi # address = null
  movq $409600, %rsi # map 100 pages (assuming 4096 byte pages)
  # movq $22039, %rsi # length = 22039
  movq $1, %rdx # prot = PROT_READ
  movq $2, %r10 # flags = MAP_PRIVATE
  movq %r15, %r8 # fd = result from open syscall
  movq $0, %r9 # offset = 0
  syscall
  pop %rcx
  pop %r11
  pop %r9
  pop %r8
  pop %r10
  pop %rdx
  pop %rsi
  pop %rdi
  ret

exit:
  mov $60, %rax
  mov $0, %rdi
  syscall
