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

# Allocate memory (at %r13) to store arrays of input numbers
call mmap_memory
movq %rax, %r13

# Store second array starting at %r12
add $4000, %rax
movq %rax, %r12

# Start at beginning of input
movq %r14, %r11

# The count of lines we've parsed
xor %r9, %r9

start_of_line:
  # Index of the number we're parsing (0 or 1)
  xor %r8, %r8

# Parse each number
parse_number:
  xor %r10, %r10 # Digit index

  # Store the number in %rax, so zero it out
  xor %rax, %rax

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

  # Multiply/divide by 10 as appropriate for digit position
  mov $4, %r15
  sub %r10, %r15
mult_loop:
  cmp $0, %r15
  je mult_loop_done
  imul $10, %ecx
  dec %r15
  jmp mult_loop

mult_loop_done:
  # Add to total
  add %ecx, %eax

  inc %r10
  cmp $5, %r10
  jne digit_loop

  # Store in array
  cmp $0, %r8
  jne store_second
  movl %eax, (%r13,%r9,4)
  jmp finished_number
store_second:
  movl %eax, (%r12,%r9,4)

finished_number:
  # Move to next line if this was the second number
  cmp $0, %r8
  je next_number
  inc %r11 # Skip newline char
  inc %r9
  jmp start_of_line

next_number:
  # Skip to next number on line
  add $3, %r11
  inc %r8
  jmp parse_number

end_of_input:
  # Store result in %rax
  xor %rax, %rax

  push %r10
  push %rdi
  push %rsi
  push %rcx
  push %rdx

  xor %rdi, %rdi # Index into first array

similarity_loop:
  xor %rcx, %rcx
  movl (%r13,%rdi,4), %ecx # Load first array value in %ecx

  xor %rsi, %rsi # Index into second array
  xor %r10, %r10 # Count in second array
count_loop:
  movl (%r12,%rsi,4), %edx
  cmp %edx, %ecx
  jne count_loop_continue
  inc %r10
count_loop_continue:
  inc %rsi
  cmp %r9, %rsi
  jne count_loop

  imul %r10, %rcx
  add %rcx, %rax

  inc %rdi
  cmp %r9, %rdi
  jne similarity_loop

  pop %rdx
  pop %rcx
  pop %rsi
  pop %rdi
  pop %r10

  # Print result
  call write_result_to_output
  lea output, %rdi
  movq $output_len, %rsi
  call print

  jmp exit

# Sort the %r9 length u32 array at address %rax
bubble_sort:
  push %r9
  push %r8
  push %rdi
  push %rcx
  push %rdx

  dec %r9

bubble_sort_repeat:
  xor %r8, %r8   # Number of swaps this loop
  xor %rdi, %rdi # Loop index

bubble_sort_loop:
  # Load element %rdi and %rdi+1 into registers
  movl (%rax,%rdi,4), %ecx
  inc %rdi
  movl (%rax,%rdi,4), %edx

  # Compare and swap if necessary
  cmp %edx, %ecx
  jle bubble_sort_next
  movl %ecx, (%rax,%rdi,4)
  dec %rdi
  movl %edx, (%rax,%rdi,4)
  inc %rdi
  inc %r8

bubble_sort_next:
  cmp %rdi, %r9
  jne bubble_sort_loop
  
  # If any swaps occurred, repeat the loop
  cmp $0, %r8
  jne bubble_sort_repeat

  pop %rdx
  pop %rcx
  pop %rdi
  pop %r8
  pop %r9
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
  pop %rcx
  pop %rdx
  pop %rax
  ret

.include "syscalls.asm"
