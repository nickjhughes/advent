.code64

.section .rodata
filename: .asciz "input05"
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

# Allocate memory (at %r13) to store array of pairs
call mmap_memory
movq %rax, %r13

# Parse each pair and store in array
parse_pair:
  xor %rcx, %rcx
  movb (%r11), %cl
  inc %r11

  # If another newline, we've reached end of pairs
  cmp $10, %cl
  je parse_updates

  sub $48, %cl
  imul $10, %rcx
  mov %rcx, %rax
  movb (%r11), %cl
  inc %r11
  sub $48, %cl
  add %rcx, %rax

  inc %r11 # Skip '|'
  
  movb (%r11), %cl
  inc %r11
  sub $48, %cl
  imul $10, %rcx
  mov %rcx, %rbx
  movb (%r11), %cl
  inc %r11
  sub $48, %cl
  add %rcx, %rbx

  # Store order for pair (a,b) at offset (a*90+b in array) (1 indicating that a < b)
  imul $90, %rax
  add %rbx, %rax
  movb $1, (%r13,%rax,1)

  inc %r11 # Skip newline
  jmp parse_pair

# We want to check if each set of numbers is sorted according to the pair orderings we parsed above
parse_updates:
  # Store result in %rax
  xor %rax, %rax

parse_update:
  xor %r15, %r15 # Count of numbers in this update
  # Address of start of array
  mov %r13, %r14
  add $9010, %r14

  movb (%r11), %cl
  inc %r11
  
  cmp $0, %cl
  je end_of_input

  sub $48, %cl
  imul $10, %rcx
  mov %rcx, %rbx
  movb (%r11), %cl
  inc %r11
  sub $48, %cl
  add %rcx, %rbx
  movb %bl, (%r14,%r15,1)
  inc %r15
  inc %r11 # Skip first comma

parse_number:
  movb (%r11), %cl
  inc %r11
  sub $48, %cl
  imul $10, %rcx
  mov %rcx, %rbx
  movb (%r11), %cl
  inc %r11
  sub $48, %cl
  add %rcx, %rbx
  movb %bl, (%r14,%r15,1)
  inc %r15

  movb (%r11), %cl
  inc %r11
  cmp $44, %cl # ','
  je parse_number
  cmp $10, %cl # Newline
  jne end_of_input

analyse_line:
  # Sort the numbers (byte array at %r14 of length %r15) using the ordering parsed above
  # If no swaps required, skip
  # Otherwise, add middle number to total

  xor %rsi, %rsi # Total number of swaps (so we know if it was already sorted or not)
  
  mov %r15, %r9
  dec %r9

sort_repeat:
  xor %r8, %r8   # Number of swaps this loop
  xor %rdi, %rdi # Loop index
  
sort_loop:
  # Load element %rdi and %rdi+1 into registers
  xor %rcx, %rcx
  xor %rdx, %rdx
  movb (%r14,%rdi,1), %cl
  inc %rdi
  movb (%r14,%rdi,1), %dl

  # Compare and swap if necessary, using parsed ordering
  # We want to see if %rcx < %rdx according to the parsed orderings
  # For that, we need the value at %rcx*90+%rdx in the array to be a 1
  mov %rcx, %r10
  imul $90, %r10
  add %rdx, %r10
  movb (%r13,%r10,1), %bl
  cmp $1, %bl
  je sort_next # Pair is already in the correct order
  movb %cl, (%r14,%rdi,1)
  dec %rdi
  movb %dl, (%r14,%rdi,1)
  inc %rdi
  inc %r8
  inc %rsi

sort_next:
  cmp %rdi, %r9
  jne sort_loop
  
  # If any swaps occurred, repeat the loop
  cmp $0, %r8
  jne sort_repeat

  # Otherwise, we're done
  cmp $0, %rsi
  je parse_update # Already sorted, so move onto next line

  # If we had to sort, get the middle number, and add to result
  push %rax
  xor %rdx, %rdx
  mov %r15, %rax
  mov $2, %r15
  divq %r15 # %rdx:%rax / %r15
  movb (%r14,%rax,1), %cl
  pop %rax
  add %rcx, %rax

  jmp parse_update

end_of_input:
  # Print result
  call write_result_to_output
  lea output, %rdi
  movq $output_len, %rsi
  call print

  jmp exit

# Sort the %r9 length byte array at address %rax
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
  movb (%rax,%rdi,1), %cl
  inc %rdi
  movb (%rax,%rdi,1), %dl

  # Compare and swap if necessary
  cmp %dl, %cl
  jle bubble_sort_next
  movb %cl, (%rax,%rdi,1)
  dec %rdi
  movb %dl, (%rax,%rdi,1)
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
