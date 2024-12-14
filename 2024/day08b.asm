.code64

.section .rodata
filename: .asciz "input08"
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

# Memory for two copies of the map, one containing the antennas and one containing the antinodes
call mmap_memory
movq %rax, 8(%rbp)  # Pointer to antenna map
call mmap_memory
movq %rax, 16(%rbp) # Pointer to antinode map
movq $0, 24(%rbp)   # Side length of maps

# Parse input
movq (%rbp), %r11   # Pointer into input
movq 8(%rbp), %r12  # Pointer to antenna map
movq 16(%rbp), %r13 # Pointer to antinode map
xor %r9, %r9        # Row count
xor %r10, %r10      # Index into maps
parse_char:
  xor %rcx, %rcx
  movb (%r11), %cl
  inc %r11
  cmp $0, %cl
  je end_of_input
  cmp $10, %cl # '\n'
  je end_of_line
  cmp $46, %cl # '.'
  je blank
  # Antenna here, so store char in antenna map and 0x00 in antimode map
  movb %cl, (%r12,%r10,1)
  movb $0, (%r13,%r10,1)
  inc %r10
  jmp parse_char
blank:
  # Nothing here, so store 0x00 in antenna and antinode map
  movb $0, (%r12,%r10,1)
  movb $0, (%r13,%r10,1)
  inc %r10
  jmp parse_char
end_of_line:
  inc %r9 # Increment row count
  jmp parse_char

end_of_input:
  # Save map size
  movq %r9, 24(%rbp)

  movq 8(%rbp), %r12  # Pointer to antenna map
  xor %r10, %r10      # Index into antenna map
  movq 24(%rbp), %r9
  imul %r9, %r9       # Total size of maps

check_antenna:
  movb (%r12,%r10,1), %cl
  cmp $0, %cl
  jz no_antenna
  
  # Find all paired antennas forwards in the map
  movq %r10, %r11
  inc %r11 # Search index
  xor %rdx, %rdx
search_loop:
  movb (%r12,%r11,1), %dl
  cmp %cl, %dl
  jne search_loop_next
  call calc_antinodes
search_loop_next:
  inc %r11
  cmp %r11, %r9
  jne search_loop
  
no_antenna:
  inc %r10
  cmp %r10, %r9
  jne check_antenna

calc_answer:
  # Answer is the total count of 1s in the antinode map
  movq 16(%rbp), %r13 # Pointer to antinode map
  xor %r10, %r10      # Index into antinode map
  movq 24(%rbp), %r9
  imul %r9, %r9       # Total size of maps
  xor %rax, %rax
  xor %rbx, %rbx
count_loop:
  movb (%r13,%r10,1), %bl
  inc %r10
  cmp $0, %bl
  je no_antinode
  inc %rax
no_antinode:
  cmp %r10, %r9
  jne count_loop

  call write_result_to_output
  lea output, %rdi
  call print
  jmp exit

# Find all the antinodes for the antennas at indices %r10 and %r11 in the map. If they're inside
# the bounds of the map, add to the antinode map.
calc_antinodes:
  push %r9
  push %rdx
  push %rcx

  movq 16(%rbp), %r13 # Pointer to antinode map
  movq 24(%rbp), %r9  # Row/col count

  # Find row and column delta between the two antennas
  # First index row/column in %rbx, %rcx
  movq %r10, %rax
  xor %rdx, %rdx
  divq %r9
  movq %rax, %rbx # row (quotient of division)
  movq %rdx, %rcx # col (remainder of division)
  # Second index row/column in %rdi, %rsi
  movq %r11, %rax
  xor %rdx, %rdx
  divq %r9
  movq %rax, %rdi # row (quotient of division)
  movq %rdx, %rsi # col (remainder of division)
  # Row diff in %rax, col diff in %rdx
  movq %rdi, %rax
  sub %rbx, %rax
  movq %rsi, %rdx
  sub %rcx, %rdx

  # Find all antinodes moving towards the second antenna from the first
first_dir_loop:
  add %rax, %rbx
  add %rdx, %rcx
  # Bounds check
  cmp $0, %rbx
  jl second_dir_loop
  cmp %r9, %rbx
  jge second_dir_loop
  cmp $0, %rcx
  jl second_dir_loop
  cmp %r9, %rcx
  jge second_dir_loop
  # Antinode is in bounds, so calculate its index and mark it in the antinode array
  movq %rbx, %r15
  imul %r9, %r15
  add %rcx, %r15
  or $1, (%r13,%r15,1)
  jmp first_dir_loop

second_dir_loop:
  sub %rax, %rdi
  sub %rdx, %rsi
  # Bounds check
  cmp $0, %rdi
  jl antinodes_done
  cmp %r9, %rdi
  jge antinodes_done
  cmp $0, %rsi
  jl antinodes_done
  cmp %r9, %rsi
  jge antinodes_done
  # Antinode is in bounds, so calculate its index and mark it in the antinode array
  movq %rdi, %r15
  imul %r9, %r15
  add %rsi, %r15
  or $1, (%r13,%r15,1)
  jmp second_dir_loop

antinodes_done:
  pop %rcx
  pop %rdx
  pop %r9
  ret

.include "common.asm"
.include "syscalls.asm"
