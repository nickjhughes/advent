.code64

.section .rodata
filename: .asciz "input07"
input_file_open_error: .ascii "Failed to open input file\n"
.set input_file_open_error_len, (. - input_file_open_error)

.section .data
output: .ascii "                                "
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

# Allocate memory (at %r13) to store each line as array
call mmap_memory
movq %rax, %r13

# Overall answer
xor %rbx, %rbx

parse_answer:
  xor %rcx, %rcx
  movb (%r11), %cl
  cmp $0, %cl
  je end_of_input
  call parse_ascii_number
  mov %rax, %r15 # Store answer in %r15
  inc %r11

  # Load the numbers into an array at %r13, and with the count of numbers in %r9
  xor %r9, %r9 # Array size
parse_number:
  xor %rcx, %rcx
  movb (%r11), %cl
  cmp $10, %cl # '\n'
  je analyse_numbers
  inc %r11 # Skip ' '
  call parse_ascii_number
  movq %rax, (%r13,%r9,8)
  inc %r9
  jmp parse_number

analyse_numbers:
  inc %r11 # Skip '\n'

  # %r8 = number to stop at
  mov %r9, %rcx
  dec %rcx
  mov $1, %r8
  shl %cl, %r8

  # We'll store the operations for each gap as bits in a binary number,
  # where 0 = +, 1 = *. Incrementing the number gives a new combination
  # of operations to try, and we stop when we hit %r8.
  xor %r10, %r10 # Current operations combo (start as all +'s)
combo_loop:
  xor %rbx, %rbx # Array index
  xor %rax, %rax # Running result
  mov (%r13,%rbx,8), %rax
  inc %rbx
number_loop:
  mov (%r13,%rbx,8), %r12
  inc %rbx

  push %r10
  mov %rbx, %rcx
  sub $2, %rcx
  shr %cl, %r10
  and $1, %r10
  cmp $0, %r10
  jne do_mul
  add %r12, %rax
  jmp pop
do_mul:
  imul %r12, %rax
pop:
  pop %r10

  cmp %rbx, %r9
  jne number_loop

  # Check if result is equal to the required answer
  cmp %r15, %rax
  je success

  cmp %r10, %r8
  je parse_answer # We failed to find a solution, so move on to next line
  inc %r10 # Next combination of operations
  jmp combo_loop

success:
  add %r15, %rdx
  jmp parse_answer

end_of_input:
  # Print result
  mov %rdx, %rax
  call write_result_to_output
  lea output, %rdi
  call print

  jmp exit

# Parse the ASCII number at %r11 into %rax. Will move %r11 to the first non-ASCII digit.
parse_ascii_number:
  xor %rax, %rax
parse_ascii_digit_loop:
  xor %rcx, %rcx
  movb (%r11), %cl
  inc %r11
  cmp $48, %cl # '0'
  jl end_of_ascii_number
  cmp $57, %cl # '9'
  jg end_of_ascii_number
  sub $48, %cl
  imul $10, %rax
  add %rcx, %rax
  jmp parse_ascii_digit_loop
end_of_ascii_number:
  dec %r11
  ret

.include "common.asm"
.include "syscalls.asm"
