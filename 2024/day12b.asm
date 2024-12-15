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
call mmap_memory
movq %rax, 32(%rbp) # Pointer to north border map
call mmap_memory
movq %rax, 40(%rbp) # Pointer to south border map
call mmap_memory
movq %rax, 48(%rbp) # Pointer to east border map
call mmap_memory
movq %rax, 56(%rbp) # Pointer to west border map

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

  # Clear border maps
  movq 32(%rbp), %r13
  call clear_edges_map
  movq 40(%rbp), %r13
  call clear_edges_map
  movq 48(%rbp), %r13
  call clear_edges_map
  movq 56(%rbp), %r13
  call clear_edges_map

  xor %rax, %rax # Tile count
  call dfs

  xor %rbx, %rbx # Side count
  call count_sides

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
  # Border index is same as map index
  movq 32(%rbp), %r13
  orb $1, (%r13,%r10,1)

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
  # Border index is map index + map size
  movq 40(%rbp), %r13
  push %r10
  add %r9, %r10
  orb $1, (%r13,%r10,1)
  pop %r10

east:
  # Bounds check: get current column, and only continue if it's < %r9-1
  push %rax
  xor %rdx, %rdx
  movq %r10, %rax
  divq %r9
  movq %rax, %rbx
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
  # Border index is col + row * (size + 1) + 1
  # Current row is %rbx
  # Current col is %rdx - 1
  movq 48(%rbp), %r13
  inc %r9
  imul %r9, %rbx
  dec %r9
  add %rdx, %rbx
  orb $1, (%r13,%rbx,1)

west:
  # Bounds check: get current column, and only continue if it's > 0
  push %rax
  xor %rdx, %rdx
  movq %r10, %rax
  divq %r9
  movq %rax, %rbx
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
  # Border index is col + row * (size + 1)
  # Current row is %rbx
  # Current col is %rdx
  movq 56(%rbp), %r13
  inc %r9
  imul %r9, %rbx
  dec %r9
  add %rdx, %rbx
  orb $1, (%r13,%rbx,1)

done:
  ret

# Zero-fill the array of length %r9(%r9+1) at %r13
clear_edges_map:
  push %r8
  push %r10
  movq %r9, %r10
  imul %r10, %r10
  add %r9, %r10
  xor %r8, %r8
clear_loop:
  movb $0, (%r13,%r8,1)
  inc %r8
  cmp %r8, %r10
  jne clear_loop
  pop %r10
  pop %r8
  ret

# Count the number of sides based on the border maps and store in %rbx
count_sides:
  push %r10
  push %rdi
  push %rcx
  push %rax

  movq %r9, %rdi
  imul %rdi, %rdi
  add %r9, %rdi # Full size of border maps

  # Horizontal side count is the number of horizontally-connected marked areas in the horizontal border map
  movq 32(%rbp), %r13
  xor %r10, %r10
  xor %rcx, %rcx
north_loop:
  movb (%r13,%r10,1), %cl
  cmp $0, %cl
  je north_next

  # Otherwise, we've found the start of a region
  inc %rbx
  # Skip over the rest of it, so it only counts as a single side
north_area_loop:
  movb (%r13,%r10,1), %cl
  cmp $0, %cl
  je north_next
  inc %r10
  cmp %rdi, %r10
  je south_borders
  # Also stop when we hit the end of a row
  movq %r10, %rax
  xor %rdx, %rdx
  divq %r9
  cmp $0, %rdx
  je north_next
  jmp north_area_loop

north_next:
  inc %r10
  cmp %rdi, %r10
  jne north_loop

south_borders:
  # Horizontal side count is the number of horizontally-connected marked areas in the horizontal border map
  movq 40(%rbp), %r13
  xor %r10, %r10
  xor %rcx, %rcx
south_loop:
  movb (%r13,%r10,1), %cl
  cmp $0, %cl
  je south_next

  # Otherwise, we've found the start of a region
  inc %rbx
  # Skip over the rest of it, so it only counts as a single side
south_area_loop:
  movb (%r13,%r10,1), %cl
  cmp $0, %cl
  je south_next
  inc %r10
  cmp %rdi, %r10
  je east_borders
  # Also stop when we hit the end of a row
  movq %r10, %rax
  xor %rdx, %rdx
  divq %r9
  cmp $0, %rdx
  je south_next
  jmp south_area_loop

south_next:
  inc %r10
  cmp %rdi, %r10
  jne south_loop

east_borders:
  push %r9
  inc %r9
  # Vertical side count is the number of vertically-connected marked areas in the vertical border map
  movq 48(%rbp), %r13
  xor %r10, %r10
  xor %rcx, %rcx
east_loop:
  movb (%r13,%r10,1), %cl
  cmp $0, %cl
  je east_next

  # Otherwise, we've found the start of a region
  inc %rbx
  # Skip over the rest of it, so it only counts as a single side
  movq %r10, %r11
east_area_loop:
  movb (%r13,%r11,1), %cl
  cmp $0, %cl
  je east_next
  movb $0, (%r13,%r11,1) # Mark tile as not a border so we don't count it again
  add %r9, %r11
  cmp %rdi, %r11
  je east_next
  jmp east_area_loop

east_next:
  inc %r10
  cmp %rdi, %r10
  jne east_loop

west_borders:
  # Vertical side count is the number of vertically-connected marked areas in the vertical border map
  movq 56(%rbp), %r13
  xor %r10, %r10
  xor %rcx, %rcx
west_loop:
  movb (%r13,%r10,1), %cl
  cmp $0, %cl
  je west_next

  # Otherwise, we've found the start of a region
  inc %rbx
  # Skip over the rest of it, so it only counts as a single side
  movq %r10, %r11
west_area_loop:
  movb (%r13,%r11,1), %cl
  cmp $0, %cl
  je west_next
  movb $0, (%r13,%r11,1) # Mark tile as not a border so we don't count it again
  add %r9, %r11
  cmp %rdi, %r11
  je west_next
  jmp west_area_loop

west_next:
  inc %r10
  cmp %rdi, %r10
  jne west_loop

  pop %r9
finished:
  pop %rax
  pop %rcx
  pop %rdi
  pop %r10
  ret

.include "common.asm"
.include "syscalls.asm"
