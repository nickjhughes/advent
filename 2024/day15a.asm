.code64

.section .rodata
filename: .asciz "input15"
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
movq %rax, 8(%rbp)  # Pointer to map
movq $0, 16(%rbp)   # Side length of map
movq $0, 24(%rbp)   # Robot start index

# Parse input
movq (%rbp), %r11   # Pointer into input
movq 8(%rbp), %r12  # Pointer to map
movq %r9, %r9       # Row count
xor %r10, %r10      # Index into map
xor %rcx, %rcx
parse_char:
  movb (%r11), %cl
  inc %r11
  cmp $10, %cl # '\n'
  je end_of_line
  cmp $64, %cl # '@'
  je robot_position
  movb %cl, (%r12,%r10,1)
  jmp next_char
robot_position:
  movq %r10, 24(%rbp)      # Save robot position
  movb $46, (%r12,%r10,1)  # Store '.' for blank space
next_char:
  inc %r10
  jmp parse_char
end_of_line:
  inc %r9 # Increment row count
  movb (%r11), %cl
  cmp $10, %cl # '\n'
  je execute_moves
  jmp parse_char

execute_moves:
  movq %r9, 16(%rbp)  # Save map size
  
move_loop:
  movb (%r11), %cl
  inc %r11
  cmp $0, %cl
  je end_of_input
  cmp $10, %cl # '\n'
  je move_loop

  cmp $94, %cl # '^'
  je move_north
  cmp $118, %cl # 'v'
  je move_south
  cmp $62, %cl # '>'
  je move_east
  cmp $60, %cl # '<'
  je move_west

move_north:
  movq 24(%rbp), %r15
  sub %r9, %r15 # Potential new position
  movb (%r12,%r15,1), %cl
  cmp $46, %cl # '.' - clear space
  je do_move
  cmp $35, %cl # '#' - wall
  je move_loop
  movq %r15, %r14
north_box_loop:
  sub %r9, %r14
  movb (%r12,%r14,1), %cl
  cmp $35, %cl # '#'
  je move_loop
  cmp $46, %cl # '.'
  jne north_box_loop
north_box_move_loop:
  movb $79, (%r12,%r14,1) # 'O'
  add %r9, %r14
  cmp %r14, %r15
  jne north_box_move_loop
  movb $46, (%r12,%r14,1) # '.'
  jmp do_move

move_south:
  movq 24(%rbp), %r15
  add %r9, %r15 # Potential new position
  movb (%r12,%r15,1), %cl
  cmp $46, %cl # '.' - clear space
  je do_move
  cmp $35, %cl # '#' - wall
  je move_loop
  movq %r15, %r14
south_box_loop:
  add %r9, %r14
  movb (%r12,%r14,1), %cl
  cmp $35, %cl # '#'
  je move_loop
  cmp $46, %cl # '.'
  jne south_box_loop
south_box_move_loop:
  movb $79, (%r12,%r14,1) # 'O'
  sub %r9, %r14
  cmp %r14, %r15
  jne south_box_move_loop
  movb $46, (%r12,%r14,1) # '.'
  jmp do_move

move_east:
  movq 24(%rbp), %r15
  inc %r15 # Potential new position
  movb (%r12,%r15,1), %cl
  cmp $46, %cl # '.' - clear space
  je do_move
  cmp $35, %cl # '#' - wall
  je move_loop
  movq %r15, %r14
east_box_loop:
  inc %r14
  movb (%r12,%r14,1), %cl
  cmp $35, %cl # '#'
  je move_loop
  cmp $46, %cl # '.'
  jne east_box_loop
east_box_move_loop:
  movb $79, (%r12,%r14,1) # 'O'
  dec %r14
  cmp %r14, %r15
  jne east_box_move_loop
  movb $46, (%r12,%r14,1) # '.'
  jmp do_move

move_west:
  movq 24(%rbp), %r15
  dec %r15 # Potential new position
  movb (%r12,%r15,1), %cl
  cmp $46, %cl # '.' - clear space
  je do_move
  cmp $35, %cl # '#' - wall
  je move_loop
  movq %r15, %r14
west_box_loop:
  dec %r14
  movb (%r12,%r14,1), %cl
  cmp $35, %cl # '#'
  je move_loop
  cmp $46, %cl # '.'
  jne west_box_loop
west_box_move_loop:
  movb $79, (%r12,%r14,1) # 'O'
  inc %r14
  cmp %r14, %r15
  jne west_box_move_loop
  movb $46, (%r12,%r14,1) # '.'
  jmp do_move

do_move:
  movq %r15, 24(%rbp)
  jmp move_loop

end_of_input:
  # call print_map
  movq %r9, %r8  # Total size of map
  imul %r8, %r8
  xor %rbx, %rbx # Answer
  xor %r10, %r10 # Map index
sum_loop:
  cmp %r10, %r8
  je print_result
  movb (%r12,%r10,1), %cl
  inc %r10
  cmp $79, %cl
  jne sum_loop
  movq %r10, %rax
  dec %rax
  xor %rdx, %rdx
  divq %r9
  imul $100, %rax
  add %rdx, %rax
  add %rax, %rbx
  jmp sum_loop

print_result:
  movq %rbx, %rax
  call write_result_to_output
  lea output, %rdi
  call print
  
  jmp exit

print_map:
  lea output, %rdi
  movq $1, %rsi
  movq %r9, %r8  # Total size of map
  imul %r8, %r8
  xor %r10, %r10 # Map index
print_loop:
  cmp %r10, %r8
  je print_done
  movb (%r12,%r10,1), %cl
  inc %r10
  movb %cl, (%rdi)
  call print
  # If end of row, print a newline
  movq %r10, %rax
  xor %rdx, %rdx
  divq %r9
  cmp $0, %rdx
  jne print_loop
  movb $10, (%rdi)
  call print
  jmp print_loop
print_done:
  ret

.include "common.asm"
.include "syscalls.asm"
