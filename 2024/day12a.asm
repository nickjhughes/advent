.code64

.section .rodata
filename: .asciz "input12"
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

call mmap_memory
movq %rax, 8(%rbp)  # Pointer to map
movq $0, 16(%rbp)   # Side length of map
movq $0, 24(%rbp)   # Answer

# Parse input
movq (%rbp), %r11   # Pointer into input
movq 8(%rbp), %r12  # Pointer to map
xor %r9, %r9        # Row count
xor %r10, %r10      # Index into map
parse_char:
  xor %rcx, %rcx
  movb (%r11), %cl
  inc %r11
  cmp $0, %cl
  je end_of_input
  cmp $10, %cl # '\n'
  je end_of_line
  movb %cl, (%r12,%r10,1)
  inc %r10
  jmp parse_char
end_of_line:
  inc %r9 # Increment row count
  jmp parse_char

end_of_input:
  # Save map size
  movq %r9, 16(%rbp)

  movq %r9, %rdi
  imul %rdi, %rdi    # Total size of map

  xor %r10, %r10
area_loop:
  xor %rcx, %rcx
  xor %r15, %r15
  movb (%r12,%r10,1), %cl
  movb %cl, %r15b
  neg %r15b
  cmp $0, %cl # A negative number indicates we've already counted this tile in a previous area
  jl next_tile

  xor %rax, %rax # Tile count
  xor %rbx, %rbx # Perimeter
  call dfs

  imul %rbx, %rax
  add %rax, 24(%rbp)

next_tile:
  inc %r10
  cmp %r10, %rdi
  jnz area_loop

  movq 24(%rbp), %rax
  call write_result_to_output
  lea output, %rdi
  call print

  jmp exit

dfs:
  movb %r15b, (%r12,%r10,1) # Label tile as visited
  inc %rax # Increment tile count

# Find potential neighbors of %r10
north:
  movq %r10, %r8
  sub %r9, %r8
  cmp $0, %r8 # Bounds check
  jl north_perimeter
  xor %r14, %r14
  movb (%r12,%r8,1), %r14b
  cmp %r15b, %r14b
  je south # Tile was already counted as part of this region, so don't count as perimeter
  cmp %rcx, %r14
  jne north_perimeter
  push %r10
  movq %r8, %r10
  call dfs
  pop %r10
  jmp south
north_perimeter:
  inc %rbx

south:
  movq %r10, %r8
  add %r9, %r8
  cmp %rdi, %r8 # Bounds check
  jge south_perimeter
  xor %r14, %r14
  movb (%r12,%r8,1), %r14b
  cmp %r15b, %r14b
  je east # Tile was already counted as part of this region, so don't count as perimeter
  cmp %rcx, %r14
  jne south_perimeter
  push %r10
  movq %r8, %r10
  call dfs
  pop %r10
  jmp east
south_perimeter:
  inc %rbx

east:
  # Bounds check: get current column, and only continue if it's < %r9-1
  push %rax
  xor %rdx, %rdx
  movq %r10, %rax
  divq %r9
  pop %rax
  inc %rdx
  cmp %r9, %rdx
  jge east_perimeter
  movq %r10, %r8
  inc %r8
  xor %r14, %r14
  movb (%r12,%r8,1), %r14b
  cmp %r15b, %r14b
  je west # Tile was already counted as part of this region, so don't count as perimeter
  cmp %rcx, %r14
  jne east_perimeter
  push %r10
  movq %r8, %r10
  call dfs
  pop %r10
  jmp west
east_perimeter:
  inc %rbx

west:
  # Bounds check: get current column, and only continue if it's > 0
  push %rax
  xor %rdx, %rdx
  movq %r10, %rax
  divq %r9
  pop %rax
  cmp $0, %rdx
  jle west_perimeter
  movq %r10, %r8
  dec %r8
  xor %r14, %r14
  movb (%r12,%r8,1), %r14b
  cmp %r15b, %r14b
  je done # Tile was already counted as part of this region, so don't count as perimeter
  cmp %rcx, %r14
  jne west_perimeter
  push %r10
  movq %r8, %r10
  call dfs
  pop %r10
  jmp done
west_perimeter:
  inc %rbx

done:
  ret

.include "common.asm"
.include "syscalls.asm"
