.code64

.section .rodata
filename: .asciz "input25"
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

  call mmap_memory
  movq %rax, 8(%rbp)  # Array of locks
  movq $0, 16(%rbp)   # Lock count
  call mmap_memory
  movq %rax, 24(%rbp) # Array of keys
  movq $0, 32(%rbp)   # Key count

  # Current key/lock heights
  movb $0, 40(%rbp)
  movb $0, 41(%rbp)
  movb $0, 42(%rbp)
  movb $0, 43(%rbp)
  movb $0, 44(%rbp)

  movq $0, 48(%rbp)  # Lock/key combo count (answer)

  # Parse input
  movq (%rbp), %r11   # Pointer into input
  movq 8(%rbp), %r12  # Pointer to lock array
  movq 24(%rbp), %r13 # Pointer to key array
  xor %r9, %r9        # Lock array index
  xor %r10, %r10      # Key array index
  xor %rcx, %rcx
parse:
  movb (%r11), %cl
  cmp $0, %cl
  je end_of_input
  cmp $35, %cl # '#'
  je parse_lock
  cmp $46, %cl # '.'
  je parse_key
  jmp exit

parse_lock:
  movb $0, 40(%rbp)
  movb $0, 41(%rbp)
  movb $0, 42(%rbp)
  movb $0, 43(%rbp)
  movb $0, 44(%rbp)
  add $6, %r11 # Skip top line
  xor %rdi, %rdi
lock_loop:
  movb (%r11), %cl
  inc %r11
  cmp $35, %cl # '#'
  jne lock_pin_2
  incb 40(%rbp)
lock_pin_2:
  movb (%r11), %cl
  inc %r11
  cmp $35, %cl # '#'
  jne lock_pin_3
  incb 41(%rbp)
lock_pin_3:
  movb (%r11), %cl
  inc %r11
  cmp $35, %cl # '#'
  jne lock_pin_4
  incb 42(%rbp)
lock_pin_4:
  movb (%r11), %cl
  inc %r11
  cmp $35, %cl # '#'
  jne lock_pin_5
  incb 43(%rbp)
lock_pin_5:
  movb (%r11), %cl
  inc %r11
  cmp $35, %cl # '#'
  jne lock_next
  incb 44(%rbp)
lock_next:
  inc %r11 # Skip newline
  inc %rdi
  cmp $5, %rdi
  jne lock_loop
  add $7, %r11 # Skip bottom line + empty line

  # Store lock
  xor %rax, %rax
  movb 40(%rbp), %al
  movb %al, (%r12,%r9,1)
  inc %r9
  movb 41(%rbp), %al
  movb %al, (%r12,%r9,1)
  inc %r9
  movb 42(%rbp), %al
  movb %al, (%r12,%r9,1)
  inc %r9
  movb 43(%rbp), %al
  movb %al, (%r12,%r9,1)
  inc %r9
  movb 44(%rbp), %al
  movb %al, (%r12,%r9,1)
  inc %r9
  jmp parse

parse_key:
  movb $5, 40(%rbp)
  movb $5, 41(%rbp)
  movb $5, 42(%rbp)
  movb $5, 43(%rbp)
  movb $5, 44(%rbp)
  add $6, %r11 # Skip top line
  xor %rdi, %rdi
key_loop:
  movb (%r11), %cl
  inc %r11
  cmp $35, %cl # '#'
  je key_pin_2
  decb 40(%rbp)
key_pin_2:
  movb (%r11), %cl
  inc %r11
  cmp $35, %cl # '#'
  je key_pin_3
  decb 41(%rbp)
key_pin_3:
  movb (%r11), %cl
  inc %r11
  cmp $35, %cl # '#'
  je key_pin_4
  decb 42(%rbp)
key_pin_4:
  movb (%r11), %cl
  inc %r11
  cmp $35, %cl # '#'
  je key_pin_5
  decb 43(%rbp)
key_pin_5:
  movb (%r11), %cl
  inc %r11
  cmp $35, %cl # '#'
  je key_next
  decb 44(%rbp)
key_next:
  inc %r11 # Skip newline
  inc %rdi
  cmp $5, %rdi
  jne key_loop
  add $7, %r11 # Skip bottom line + empty line

  # Store key
  xor %rax, %rax
  movb 40(%rbp), %al
  movb %al, (%r13,%r10,1)
  inc %r10
  movb 41(%rbp), %al
  movb %al, (%r13,%r10,1)
  inc %r10
  movb 42(%rbp), %al
  movb %al, (%r13,%r10,1)
  inc %r10
  movb 43(%rbp), %al
  movb %al, (%r13,%r10,1)
  inc %r10
  movb 44(%rbp), %al
  movb %al, (%r13,%r10,1)
  inc %r10
  jmp parse

end_of_input:
  movq %r9, 16(%rbp)  # Save lock count
  movq %r10, 32(%rbp) # Save key count

  xor %r9, %r9
check_lock_loop:
  xor %r10, %r10
check_key_loop:
  xor %rbx, %rbx
  xor %rdi, %rdi
check_pin_loop:
  xor %rax, %rax
  movb (%r12,%r9,1), %al
  add (%r13,%r10,1), %al
  inc %r9
  inc %r10

  cmp $6, %al
  jl next_pin
  or $1, %rbx

next_pin:
  inc %rdi
  cmp $5, %rdi
  jne check_pin_loop

  cmp $1, %rbx
  je next_key
  inc 48(%rbp)

next_key:
  sub $5, %r9
  cmp %r10, 32(%rbp)
  jne check_key_loop

  add $5, %r9
  cmp %r9, 16(%rbp)
  jne check_lock_loop

  movq 48(%rbp), %rax
  call write_result_to_output
  lea output, %rdi
  call print

  jmp exit

.include "common.asm"
.include "syscalls.asm"
