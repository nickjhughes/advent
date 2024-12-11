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

# Allocate memory for arrays and set up local variables
call mmap_memory
movq %rax, (%rbp) # Actual file blocks
movq $0, 8(%rbp)  # Block count
call mmap_memory
movq %rax, 16(%rbp) # List of free spaces
movq $0, 24(%rbp)  # Free space count
call mmap_memory
movq %rax, 32(%rbp) # List of file spaces
movq $0, 40(%rbp) # File space count

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
  je new_space

new_file:
  # Store (index, length) of file space
  movq 32(%rbp), %r13
  movq 40(%rbp), %r10
  movq 8(%rbp), %rax
  shl $32, %rax
  or %rcx, %rax
  movq %rax, (%r13,%r10,8)
  incq 40(%rbp)
fill_file_loop:
  movq (%rbp), %r13
  movq 8(%rbp), %r10
  movw %r15w, (%r13,%r10,2)
  incq 8(%rbp)
  dec %cl
  jnz fill_file_loop
  jmp next_digit

new_space:
  # Store (index, length) of free space
  movq 16(%rbp), %r13
  movq 24(%rbp), %r10
  movq 8(%rbp), %rax
  shl $32, %rax
  or %rcx, %rax
  movq %rax, (%r13,%r10,8)
  incq 24(%rbp)
fill_space_loop:
  movq (%rbp), %r13
  movq 8(%rbp), %r10
  movw $-1, (%r13,%r10,2)
  incq 8(%rbp)
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
  # Current file index
  movq 40(%rbp), %r8
file_loop:
  dec %r8
  
  movq 32(%rbp), %r13
  movq (%r13,%r8,8), %rbx
  shr $32, %rbx
  movq %rbx, %rdi # Current file position
  movq (%r13,%r8,8), %rbx
  and $0xffffffff, %ebx # Current file size

  movq 16(%rbp), %r13
  xor %r9, %r9 # Free space index
free_space_loop:
  movq (%r13,%r9,8), %rax
  shr $32, %rax # Current free space position
  cmp %rax, %rdi
  jl next_file # Free space is to the right of the file, so move to next file
  movq (%r13,%r9,8), %rax
  and $0xffffffff, %eax # Current free space size
  cmp %eax, %ebx
  jle move_file # File fits in free space, so move it
  
  inc %r9
  movq 24(%rbp), %r10
  cmp %r9, %r10
  jne free_space_loop
  jmp next_file

move_file:
  # Load file index
  movq 32(%rbp), %r13
  movq (%r13,%r8,8), %rcx
  shr $32, %rcx # File index in %ecx, size in %ebx

  # Load free space index
  movq 16(%rbp), %r13
  movq (%r13,%r9,8), %rdi
  shr $32, %rdi # Free space index in %edi, size in %eax

  # Move file blocks to free space
  xor %rsi, %rsi # Move loop counter
move_loop:
  movq (%rbp), %r13
  xor %r15, %r15
  movw (%r13,%rcx,2), %r15w
  movw %r15w, (%r13,%rdi,2)
  movw $-1, (%r13,%rcx,2)
  inc %ecx
  inc %edi
  inc %rsi
  cmp %rsi, %rbx
  jne move_loop

  # Update the free space list
  cmp %eax, %ebx
  je delete_free_space

  # Decrease the free space size by the size of the file
  # and increase its index by the size of the file
  movq 16(%rbp), %r13
  movq (%r13,%r9,8), %rax
  sub %rbx, %rax # Decrease length by size of file
  movq %rbx, %rcx
  shl $32, %rcx
  add %rcx, %rax # Increase index by size of file
  movq %rax, (%r13,%r9,8)

  jmp next_file

delete_free_space:
  # Set free space length (and index) to 0
  movq 16(%rbp), %r13
  movq $0, (%r13,%r9,8)

next_file:
  cmp $0, %r8
  jne file_loop

  # Calculate and print checksum
  xor %rax, %rax # Checksum
  xor %r9, %r9 # File block index
  movq (%rbp), %r13 # File block array pointer
  movq 8(%rbp), %r10 # Block count
checksum_loop:
  xor %rcx, %rcx
  movw (%r13,%r9,2), %cx
  inc %r9
  cmp $-1, %cx
  je checksum_loop
  dec %r9
  cmp %r10, %r9
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

.include "common.asm"
.include "syscalls.asm"
