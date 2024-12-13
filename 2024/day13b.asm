.code64

.section .rodata
filename: .asciz "input13"
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
movq %rax, %r14

# Start at beginning of input
movq %r14, %r11

# We're solving the following matrix equation for each machine,
# for non-negative, integer values of A and B less or equal to 100.
# (x) = (a b) (A)
# (y)   (c d) (B)
movq $0, (%rbp)  # a
movq $0, 8(%rbp) # b
movq $0, 16(%rbp) # c
movq $0, 24(%rbp) # d
movq $0, 32(%rbp) # x
movq $0, 40(%rbp) # y

movq $0, 48(%rbp) # Puzzle answer

parse_machine:
  # Button A: X+a, Y+c
  # Button B: X+b, Y+d
  # Prize: X=x, Y=y

  xor %rcx, %rcx
  movb (%r11), %cl
  cmp $0, %cl
  je end_of_input
  cmp $10, %cl # Newline
  jne no_newline
  inc %r11

no_newline:
  add $12, %r11 # Skip "Button A: X+"
  call parse_integer
  movq %rax, (%rbp) # Parse into a
  add $4, %r11 # Skip ", Y+"
  call parse_integer
  movq %rax, 16(%rbp) # Parse into c
  add $13, %r11 # Skip "\nButton B: X+"
  call parse_integer
  movq %rax, 8(%rbp) # Parse into b
  add $4, %r11 # Skip ", Y+"
  call parse_integer
  movq %rax, 24(%rbp) # Parse into d
  add $10, %r11 # Skip "\nPrize: X="
  call parse_integer
  mov $10000000000000, %rbx
  add %rbx, %rax
  movq %rax, 32(%rbp) # Parse into x
  add $4, %r11 # Skip ", Y="
  call parse_integer
  mov $10000000000000, %rbx
  add %rbx, %rax
  movq %rax, 40(%rbp) # Parse into y
  inc %r11 # Skip "\n"

solve:
  # Calculate determinant of matrix [[a b], [c d]] = 1 / (a * d - b * c)
  movq (%rbp), %rax
  movq 24(%rbp), %rdx
  imul %rdx, %rax
  movq 8(%rbp), %rbx
  movq 16(%rbp), %rcx
  imul %rbx, %rcx
  sub %rcx, %rax
  # If determinant is zero, we have no valid solution
  jz parse_machine
  movq %rax, %r15 # Save determinant for later
  
  # Perform matrix multiplication of adjoint matrix [[d, -b], [-c, a]] with result matrix [[x], [y]]
  # A = d * x - b * y
  movq 32(%rbp), %rax
  imul %rdx, %rax
  neg %rbx
  movq 40(%rbp), %rdx
  imul %rbx, %rdx
  add %rax, %rdx
  mov %rdx, %rax
  cqto
  # If the result doesn't divide evenly by the determinant, we have no integer solution
  idivq %r15
  cmp $0, %rdx
  jne parse_machine
  # Otherwise, %rax contains our solution for A
  movq %rax, %r13

  # B = -c * x + a * y
  movq 16(%rbp), %rcx
  neg %rcx
  movq 32(%rbp), %rax
  imul %rcx, %rax
  movq (%rbp), %rcx
  movq 40(%rbp), %rdx
  imul %rcx, %rdx
  add %rdx, %rax
  cqto
  idivq %r15
  cmp $0, %rdx
  jne parse_machine
  movq %rax, %r14

  # Answer is (A, B) in (%r13, %r14)
  # Solution is A * 3 + B * 1 tokens
  imul $3, %r13
  add %r14, %r13
  add %r13, 48(%rbp)

  jmp parse_machine

parse_integer:
  # Parse an integer at %r11 into %rax
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

end_of_input:
  movq 48(%rbp), %rax
  call write_result_to_output
  lea output, %rdi
  call print

  jmp exit

.include "common.asm"
.include "syscalls.asm"
