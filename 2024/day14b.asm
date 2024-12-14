.code64

.section .rodata
filename: .asciz "input14"
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

# Open and mmap null-terminated input file to address (%r14)
call open_input_file
movq %rax, %rdi
call mmap_input_file
movq %rax, (%rbp) # Pointer to input

# Allocate memory for robot array, which contains 4 x 8-byte integeres:
#   (pos_x, pos_y, vel_x, vel_y)
call mmap_memory
movq %rax, 8(%rbp)  # Pointer to robot array
movq $0, 16(%rbp)   # Robot array size
movq $0, 24(%rbp)   # Simulated seconds

# Parse input
movq 8(%rbp), %r13  # Pointer to robot array
movq (%rbp), %r11   # Pointer into input
xor %r9, %r9        # Robot array index
parse_robot:
  xor %rcx, %rcx
  movb (%r11), %cl
  cmp $0, %cl
  je end_of_input

  add $2, %r11 # Skip "p="
  call parse_integer
  movq %rax, (%r13,%r9,8)
  inc %r9
  inc %r11 # Skip comma
  call parse_integer
  movq %rax, (%r13,%r9,8)
  inc %r9
  add $3, %r11 # Skip " v="
  call parse_integer
  movq %rax, (%r13,%r9,8)
  inc %r9
  inc %r11 # Skip comma
  call parse_integer
  movq %rax, (%r13,%r9,8)
  inc %r9
  inc %r11 # Skip newline
  jmp parse_robot

end_of_input:
  movq %r9, 16(%rbp) # Save robot array size
  
simulate_second:
  xor %r9, %r9 # Robot array index
robot_loop:
  movq (%r13,%r9,8), %rax # x-pos
  inc %r9
  movq (%r13,%r9,8), %rbx # y-pos
  inc %r9
  movq (%r13,%r9,8), %rcx # x-vel
  inc %r9
  movq (%r13,%r9,8), %rdx # y-vel
  sub $3, %r9
  
  # Move robot
  add %rcx, %rax
  add %rdx, %rbx

  # Restrict to bounds of area
  # 101 tiles wide
  movq $101, %rdi
  cqto
  idivq %rdi
  cmp $0, %rdx
  jge positive_x
  add $101, %rdx
positive_x:
  movq %rdx, (%r13,%r9,8)
  inc %r9

  # 103 tiles tall
  movq $103, %rdi
  movq %rbx, %rax
  cqto
  idivq %rdi
  cmp $0, %rdx
  jge positive_y
  add $103, %rdx
positive_y:
  movq %rdx, (%r13,%r9,8)
  add $3, %r9
  
  cmp %r9, 16(%rbp)
  jne robot_loop

  movq 24(%rbp), %rax
  cmp $5000, %rax
  jl next
  inc %rax
  call write_result_to_output
  lea output, %rdi
  call print
  call print_robots
next:
  inc 24(%rbp)
  cmp $10000, 24(%rbp)
  jne simulate_second

  jmp exit

parse_integer:
  # Parse a potentially negative integer at %r11 into %rax
  xor %rax, %rax # Result
  xor %rcx, %rcx # Current digit
  xor %r15, %r15 # Is negative flag
  movb (%r11), %cl
  cmp $45, %cl # '-'
  jne parse_digit
  inc %r11
  inc %r15
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
  cmp $1, %r15
  jne parse_integer_positive
  neg %rax
parse_integer_positive:
  ret

print_robots:
  # Loop through all positions, and print a ' ' if there's no robot, or a '#' otherwise
  
  lea output, %rdi
  movq $1, %rsi

  xor %r8, %r8 # x-pos
  xor %r9, %r9 # y-pos
print_loop:
  # Search robot array to see if there's one at (%r8, %r9)
  xor %r10, %r10
robot_search:
  movq (%r13,%r10,8), %rax
  inc %r10
  movq (%r13,%r10,8), %rbx
  add $3, %r10

  cmp %rax, %r8
  jne next_robot
  cmp %rbx, %r9
  jne next_robot

  # Found a robot, so print a '#'
  movb $35, (%rdi)
  call print
  jmp next_position

next_robot:
  cmp %r10, 16(%rbp)
  jne robot_search

  # Didn't find a robot, so print a space
  movb $32, (%rdi)
  call print

next_position:
  inc %r8
  cmp $101, %r8
  jne print_loop

  movb $10, (%rdi)
  call print
  xor %r8, %r8
  inc %r9
  cmp $103, %r9
  jne print_loop
print_done:
  ret

.include "common.asm"
.include "syscalls.asm"
