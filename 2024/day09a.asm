.code64

.section .rodata
filename: .asciz "input09"
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

# Allocate memory (at %r13) to store the file blocks
call mmap_memory
movq %rax, %r13

xor %r10, %r10 # Block count
xor %r8, %r8 # Whether the current digit is a file (0) or free space (1)
xor %r15, %r15 # Current file index

parse_digit:
  # Load char into %cl
  xor %rcx, %rcx
  movb (%r11), %cl
  inc %r11

  cmp $0, %cl # Null
  je end_of_input
  cmp $10, %cl # Newline
  je end_of_input

  sub $48, %cl # Convert ASCII to digit
  jz next_digit
  cmp $1, %r8
  je fill_space_loop

fill_file_loop:
  movw %r15w, (%r13,%r10,2)
  inc %r10
  dec %cl
  jnz fill_file_loop
  jmp next_digit

fill_space_loop:
  movw $-1, (%r13,%r10,2)
  inc %r10
  dec %cl
  jnz fill_space_loop

next_digit:
  inc %r8
  cmp $2, %r8
  jne parse_digit
  xor %r8, %r8
  inc %r15
  jmp parse_digit

end_of_input:
  # Index of the first free block
  xor %r8, %r8
  call next_free_block

  # Index of the last file block
  xor %r9, %r9
  mov %r10, %r9
  dec %r9
  call next_file_block

move_loop:
  # Move the last file block to the first free block, until the two indices are equal/in the other order
  xor %r15, %r15
  movw (%r13,%r9,2), %r15w
  movw %r15w, (%r13,%r8,2)
  movw $-1, (%r13,%r9,2)
  call next_free_block
  call next_file_block
  cmp %r9, %r8
  jl move_loop

  # Calculate and print checksum
  xor %rax, %rax
  xor %r9, %r9
checksum_loop:
  xor %rcx, %rcx
  movw (%r13,%r9,2), %cx
  cmp $-1, %cx
  je checksum_loop_done
  imul %r9, %rcx
  add %rcx, %rax
  inc %r9
  jmp checksum_loop

checksum_loop_done:
  call write_result_to_output
  lea output, %rdi
  call print

  jmp exit

# Increment %r8 until it points to the next free block (or exceeds the array)
next_free_block:
  push %rax
  xor %rax, %rax
next_free_block_loop:
  movw (%r13,%r8,2), %ax
  cmp $-1, %ax
  je next_free_block_done
  inc %r8
  jmp next_free_block_loop
next_free_block_done:
  pop %rax
  ret

# Decrement %r9 until it points to the next file block (or exceeds the array)
next_file_block:
  push %rax
  xor %rax, %rax
next_file_block_loop:
  movw (%r13,%r9,2), %ax
  cmp $-1, %ax
  jne next_file_block_done
  dec %r9
  jmp next_file_block_loop
next_file_block_done:
  pop %rax
  ret

.include "common.asm"
.include "syscalls.asm"
