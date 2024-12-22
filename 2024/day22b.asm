.code64

.section .rodata
filename: .asciz "input22"
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
  movq %rax, (%rbp)   # Pointer to input

  movq $64, %rdi
  call mmap_pages
  movq %rax, 8(%rbp)   # Global result for each sequence
  movq $32, %rdi
  call mmap_pages
  movq %rax, 16(%rbp)  # Result for the current sequence

  # Current diff sequence
  movq $0, 32(%rbp)
  movq $0, 33(%rbp)
  movq $0, 34(%rbp)
  movq $0, 35(%rbp)

  # Parse input
  movq (%rbp), %r11    # Pointer into input
  movq 8(%rbp), %r12   # Pointer to global array
  movq 16(%rbp), %r13  # Pointer to current array
  xor %r9, %r9
parse:
  movb (%r11), %cl
  cmp $0, %cl
  je end_of_input
  call parse_integer
  inc %r11 # Skip newline

  # Clear current array
  movq $130321, %rdi
  xor %r10, %r10
clear_current:
  movb $-1, (%r13,%r10,1)
  inc %r10
  cmp %r10, %rdi
  jne clear_current

  # Store current sequence number in %rax, and last decimal digit in %rdx
  call mod10

  xor %r10, %r10
gen_count:
  call generate_secret_number
  movq %rdx, %rbx
  call mod10
  # Calculate difference between current and previous %rdx
  sub %rdx, %rbx
  neg %rbx

  # Add new difference to the running last 4
  xor %rcx, %rcx
  movb 34(%rbp), %cl
  movb %cl, 35(%rbp)
  movb 33(%rbp), %cl
  movb %cl, 34(%rbp)
  movb 32(%rbp), %cl
  movb %cl, 33(%rbp)
  movb %bl, 32(%rbp)

  # If we have at least 4 differences, add to current array
  cmp $3, %r10
  push %rbx
  jl next_gen
  # Index will be 32(%rbp) * 19^3 + 33(%rbp) * 19^2 + 34(%rbp) * 19 + 35(%rbp)
  xor %rcx, %rcx
  movb 32(%rbp), %cl
  add $9, %cl
  and $0xFF, %cl
  imul $6859, %rcx
  movq %rcx, %rbx
  xor %rcx, %rcx
  movb 33(%rbp), %cl
  add $9, %cl
  and $0xFF, %cl
  imul $361, %rcx
  add %rcx, %rbx
  xor %rcx, %rcx
  movb 34(%rbp), %cl
  add $9, %cl
  and $0xFF, %cl
  imul $19, %rcx
  add %rcx, %rbx
  xor %rcx, %rcx
  movb 35(%rbp), %cl
  add $9, %cl
  and $0xFF, %cl
  add %rcx, %rbx

  # If we've already seen this sequence before, skip it
  xor %rcx, %rcx
  movb (%r13,%rbx,1), %cl
  cmp $-1, %cl
  jne next_gen
  # Otherwise, store the result in the local array (so we know we've seen it)
  movb %dl, (%r13,%rbx,1)
  # And add it to the total for the global array
  add %dx, (%r12,%rbx,2)

next_gen:
  pop %rbx

  inc %r10
  cmp $2000, %r10
  jne gen_count

  jmp parse

end_of_input:
  # Find max in global array
  movq $130321, %rdi
  xor %r10, %r10
  xor %rax, %rax
  xor %rbx, %rbx
max_loop:
  movw (%r12,%r10,2), %dx
  cmp %dx, %ax
  jg next_max_loop
  movw %dx, %ax
next_max_loop:
  inc %r10
  cmp %r10, %rdi
  jne max_loop

  call write_result_to_output
  lea output, %rdi
  call print

  jmp exit

# Apply mod 10 to %rax and store in %rdx (without clobbering %rax)
mod10:
  push %rax
  imul $1717986919, %rax, %rcx
  movq %rcx, %rdx
  shr $63, %rdx
  sar $34, %rcx
  add %edx, %ecx
  add %ecx, %ecx
  lea (%rcx,%rcx,4), %ecx
  sub %ecx, %eax
  movq %rax, %rdx
  pop %rax
  ret

# Apply the secret number generation process to %rax
generate_secret_number:
  movq %rax, %rbx
  shl $6, %rbx        # Multiply by 64
  xor %rbx, %rax      # Mix
  and $16777215, %rax # Prune
  movq %rax, %rbx
  shr $5, %rbx        # Divide by 32
  xor %rbx, %rax      # Mix
  and $16777215, %rax # Prune
  movq %rax, %rbx
  shl $11, %rbx       # Multiply by 2048
  xor %rbx, %rax      # Mix
  and $16777215, %rax # Prune
  ret

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

.include "common.asm"
.include "syscalls.asm"
