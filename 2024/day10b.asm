.code64

.section .rodata
filename: .asciz "input10"
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
sub $32, %rsp

# Open and mmap null-terminated input file to address (%r14)
call open_input_file
movq %rax, %rdi
call mmap_input_file
movq %rax, (%rbp) # Pointer to input

call mmap_memory
movq %rax, 8(%rbp)  # Pointer to height map
movq $0, 16(%rbp)   # Side length of map
movq $0, 24(%rbp)   # Trailhead score sum

# Parse input
movq (%rbp), %r11   # Pointer into input
movq 8(%rbp), %r12  # Pointer to height map
xor %r9, %r9        # Row count
xor %r10, %r10      # Index into height map
parse_char:
  xor %rcx, %rcx
  movb (%r11), %cl
  inc %r11
  cmp $0, %cl
  je end_of_input
  cmp $10, %cl # '\n'
  je end_of_line
  cmp $46, %cl # '.'
  je dot
  sub $48, %cl # Convert ASCII digit to integer
  movb %cl, (%r12,%r10,1)
  inc %r10
  jmp parse_char
dot:
  movb $-1, (%r12,%r10,1)
  inc %r10
  jmp parse_char
end_of_line:
  inc %r9 # Increment row count
  jmp parse_char

end_of_input:
  movq %r9, 16(%rbp) # Save map size
  imul %r9, %r9

  xor %r10, %r10 # Current trailhead index
trailhead_loop:
  # Iterate through map finding trailheads (0 height locations)
  xor %rcx, %rcx
  movb (%r12,%r10,1), %cl
  cmp $0, %cl
  jne next_trailhead

  call find_trailhead_score
  add %rax, 24(%rbp)

next_trailhead:
  inc %r10
  cmp %r9, %r10
  jne trailhead_loop

  movq 24(%rbp), %rax
  call write_result_to_output
  lea output, %rdi
  call print

  jmp exit

# Determine the score of the trailhead at index %r10, returned in %rax
find_trailhead_score:
  push %r9
  push %r10

  # We need to do a depth-first search from index %r10, finding how many distinct 9s we can reach,
  # only following paths that increase by a single height each time.

  movq 8(%rbp), %r12 # Pointer to height map
  movq 16(%rbp), %r9 # Side length of height map
  movq %r9, %rdi
  imul %rdi, %rdi    # Total size of map

  movq %r10, %r11 # Current vertex index
  xor %rax, %rax  # Count of reachable trail ends
  call dfs

  pop %r10
  pop %r9
  ret

dfs:
  xor %r15, %r15
  movb (%r12,%r11,1), %r15b # Current height
  cmp $9, %r15b
  jne neighbors
  # We've reached the end of a trail, so add to count and exit
  inc %rax
  ret

# Find potential neighbors of %r11
neighbors:
  inc %r15b # Acceptable next height

north:
  movq %r11, %r8
  sub %r9, %r8
  cmp $0, %r8 # Bounds check
  jl south
  xor %r14, %r14
  movb (%r12,%r8,1), %r14b
  cmp %r15, %r14
  jne south
  push %r11
  push %r15
  movq %r8, %r11
  call dfs
  pop %r15
  pop %r11

south:
  movq %r11, %r8
  add %r9, %r8
  cmp %rdi, %r8 # Bounds check
  jge east
  xor %r14, %r14
  movb (%r12,%r8,1), %r14b
  cmp %r15, %r14
  jne east
  push %r11
  push %r15
  movq %r8, %r11
  call dfs
  pop %r15
  pop %r11

east:
  # Bounds check: get current column, and only continue if it's < %r9-1
  push %rax
  xor %rdx, %rdx
  movq %r11, %rax
  divq %r9
  pop %rax
  inc %rdx
  cmp %r9, %rdx
  jge west
  movq %r11, %r8
  inc %r8
  xor %r14, %r14
  movb (%r12,%r8,1), %r14b
  cmp %r15, %r14
  jne west
  push %r11
  push %r15
  movq %r8, %r11
  call dfs
  pop %r15
  pop %r11

west:
  # Bounds check: get current column, and only continue if it's > 0
  push %rax
  xor %rdx, %rdx
  movq %r11, %rax
  divq %r9
  pop %rax
  cmp $0, %rdx
  jle done
  movq %r11, %r8
  dec %r8
  xor %r14, %r14
  movb (%r12,%r8,1), %r14b
  cmp %r15, %r14
  jne done
  push %r11
  push %r15
  movq %r8, %r11
  call dfs
  pop %r15
  pop %r11

done:
  ret

.include "common.asm"
.include "syscalls.asm"
