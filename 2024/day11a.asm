.code64

.section .rodata
filename: .asciz "input11"
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
sub $64, %rsp

# Open and mmap null-terminated input file to address
call open_input_file
movq %rax, %rdi
call mmap_input_file
movq %rax, (%rbp) # Pointer to input

# Allocate memory for two arrays of numbers and counts
# (We need two so that we can effectively double-buffer them for each update)
call mmap_memory
movq %rax, 8(%rbp) # Pointer to first array
movq $0, 16(%rbp)  # First array length
call mmap_memory
movq %rax, 24(%rbp) # Pointer to second array
movq $0, 32(%rbp)   # Second array length

movq (%rbp), %r11  # Pointer into input
movq 8(%rbp), %r12 # Pointer to first array
xor %r13, %r13     # Index into first array
parse_stone:
  xor %rcx, %rcx
  movb (%r11), %cl
  cmp $0, %cl
  je end_of_input
  call parse_integer

  movq %rax, (%r12,%r13,8)
  inc %r13
  movq $1, (%r12,%r13,8)
  inc %r13

  inc %r11 # Skip ' ' or '\n'
  jmp parse_stone

end_of_input:
  movq %r13, 16(%rbp) # Save array length
  xor %r15, %r15      # Blink count

  movq 24(%rbp), %r10  # Pointer to source array
  movq 8(%rbp), %r11   # Pointer to target array
  movq 16(%rbp), %r12  # Source array length
blink_loop:
  xchg %r10, %r11  # Swap array pointers to double-buffer
  xor %r13, %r13   # Target array length
  xor %r14, %r14   # Source array index
stone_loop:
  movq (%r10,%r14,8), %rax # Stone number
  inc %r14
  movq (%r10,%r14,8), %rbx # Stone count
  inc %r14

  cmp $0, %rax
  je stone_zero
  movq $10, %r8
  xor %rcx, %rcx # Decimal digit count
  push %rax
div_loop:
  xor %rdx, %rdx
  divq %r8
  inc %rcx
  cmp $0, %rax
  jne div_loop
  pop %rax
  mov %rcx, %r8
  and $1, %rcx
  jz stone_even

stone_mul_2024:
  # Multiply by 2024
  imul $2024, %rax
  call store_in_target_array
  jmp stone_done

stone_zero:
  # Turn 0 into 1
  inc %rax
  call store_in_target_array
  jmp stone_done

stone_even:
  # Split number in half to create two new numbers
  mov %r8, %rcx
  # %rcx contains number of digits
  shr $1, %rcx
  # Now %rcx is number of times to divide the number by 10 to split it in half
  movq %rcx, %rdi
  xor %r9, %r9 # Right half of number, left half will end up in %rax
  movq $10, %r8
split_loop:
  xor %rdx, %rdx
  divq %r8

  # Multiply %rdx by 10 as appropriate before adding to %r9
  movq %rdi, %rsi
  sub %rcx, %rsi
mul_loop:
  cmp $0, %rsi
  je mul_loop_done
  imul $10, %rdx
  dec %rsi
  jmp mul_loop
mul_loop_done:
  add %rdx, %r9

  dec %rcx
  cmp $0, %rcx
  jne split_loop
  mov %r9, %rcx
  call store_in_target_array
  mov %rcx, %rax
  call store_in_target_array

stone_done:
  cmp %r14, %r12
  jne stone_loop

  movq %r13, %r12 # Update source array length
  inc %r15
  cmp $25, %r15
  jne blink_loop

print_result:
  # Answer is the sum of the counts in the current target array
  xor %rax, %rax  # Sum
  xor %r9, %r9    # Index into array
sum_loop:
  inc %r9
  movq (%r11,%r9,8), %rbx
  add %rbx, %rax
  inc %r9
  cmp %r9, %r13
  jne sum_loop

  call write_result_to_output
  lea output, %rdi
  call print
  jmp exit

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

# Store %rax with count %rbx into the target array of length %r13 at address %r11
# If already in the array, add to its count. If not, add to the end of the array
store_in_target_array:
  # Search for %rax in target array
  cmp $0, %r13
  je not_found
  xor %rdi, %rdi # Target array search index
search_loop:
  movq (%r11,%rdi,8), %r8
  cmp %r8, %rax
  je found
  add $2, %rdi
  cmp %rdi, %r13
  jne search_loop
not_found:
  # Number not in target array, so add it to the end
  movq %rax, (%r11,%r13,8)
  inc %r13
  movq %rbx, (%r11,%r13,8)
  inc %r13
  ret
found:
  # Number in target array already, so just add to its count
  inc %rdi
  add %rbx, (%r11,%rdi,8)
  ret

.include "common.asm"
.include "syscalls.asm"
