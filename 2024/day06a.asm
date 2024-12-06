.code64

.section .rodata
filename: .asciz "input06"
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

# Allocate memory (at %r13) to store map as array
call mmap_memory
movq %rax, %r13

# Load the map into an array at %r13, and store the size of the square array in %r9
xor %r9, %r9 # Size
xor %r10, %r10 # Array index

parse_char:
  # Load char into %cl
  xor %rcx, %rcx
  movb (%r11), %cl
  inc %r11

  cmp $0, %cl # Null
  je end_of_input

  cmp $10, %cl # Newline
  je end_of_line

  cmp $94, %cl # '^'
  je parse_pos

  # Copy char to array and increment index
  movb %cl, (%r13,%r10,1)
  inc %r10
  jmp parse_char

parse_pos:
  mov %r10, %r15 # Guard position index

  movb $88, (%r13,%r10,1) # 'X'
  inc %r10
  jmp parse_char

end_of_line:
  # Increment row count
  inc %r9
  jmp parse_char

end_of_input:
  mov %r15, %r10 # Index of guard
  # Directions: North = 0, East = 1, South = 2, West = 3
  mov $0, %r14 # Direction of guard

# Move the guard through the map, leaving 'X' where they've been
move_guard:
  cmp $0, %r14
  je move_north
  cmp $1, %r14
  je move_east
  cmp $2, %r14
  je move_south
  cmp $3, %r14
  je move_west

move_north:
  # Store current (row,col) in (%r11,%r12)
  # Calculate row/col from index %r10 by dividing by %r9
  # idiv divides %rdx:%rax, so clear %rdx and move %r10 to %rax
  push %rax
  xor %rdx, %rdx
  mov %r10, %rax
  idiv %r9
  mov %rax, %r11 # row (quotient of division)
  mov %rdx, %r12 # col (remainder of division)
  pop %rax

  # Move north
  dec %r11
  mov %r10, %r15 # Potential new index
  sub %r9, %r15

  # Bound check
  cmp %r9, %r11
  jge left_map
  cmp $0, %r11
  jl left_map

  # Check for obstacle
  xor %rcx, %rcx
  movb (%r13,%r15,1), %cl
  cmp $35, %cl # '#'
  jne do_move_north
  # Rotate clockwise
  inc %r14
  jmp move_guard

do_move_north:
  sub %r9, %r10 # Actually do move
  movb $88, (%r13,%r10,1) # Leave an 'X' everywhere we go
  jmp move_guard

move_east:
  # Store current (row,col) in (%r11,%r12)
  push %rax
  xor %rdx, %rdx
  mov %r10, %rax
  idiv %r9
  mov %rax, %r11 # row (quotient of division)
  mov %rdx, %r12 # col (remainder of division)
  pop %rax

  # Move east
  inc %r12
  mov %r10, %r15 # Potential new index
  inc %r15

  # Bound check
  cmp %r9, %r12
  jge left_map
  cmp $0, %r12
  jl left_map

  # Check for obstacle
  xor %rcx, %rcx
  movb (%r13,%r15,1), %cl
  cmp $35, %cl # '#'
  jne do_move_east
  # Rotate clockwise
  inc %r14
  jmp move_guard

do_move_east:
  inc %r10 # Actually do move
  movb $88, (%r13,%r10,1) # Leave an 'X' everywhere we go
  jmp move_guard

move_south:
  # Store current (row,col) in (%r11,%r12)
  push %rax
  xor %rdx, %rdx
  mov %r10, %rax
  idiv %r9
  mov %rax, %r11 # row (quotient of division)
  mov %rdx, %r12 # col (remainder of division)
  pop %rax

  # Move south
  inc %r11
  mov %r10, %r15 # Potential new index
  add %r9, %r15

  # Bound check
  cmp %r9, %r11
  jge left_map
  cmp $0, %r11
  jl left_map

  # Check for obstacle
  xor %rcx, %rcx
  movb (%r13,%r15,1), %cl
  cmp $35, %cl # '#'
  jne do_move_south
  # Rotate clockwise
  inc %r14
  jmp move_guard

do_move_south:
  add %r9, %r10 # Actually do move
  movb $88, (%r13,%r10,1) # Leave an 'X' everywhere we go
  jmp move_guard

move_west:
  # Store current (row,col) in (%r11,%r12)
  push %rax
  xor %rdx, %rdx
  mov %r10, %rax
  idiv %r9
  mov %rax, %r11 # row (quotient of division)
  mov %rdx, %r12 # col (remainder of division)
  pop %rax

  # Move east
  dec %r12
  mov %r10, %r15 # Potential new index
  dec %r15

  # Bound check
  cmp %r9, %r12
  jge left_map
  cmp $0, %r12
  jl left_map

  # Check for obstacle
  xor %rcx, %rcx
  movb (%r13,%r15,1), %cl
  cmp $35, %cl # '#'
  jne do_move_west
  # Rotate clockwise
  xor %r14, %r14
  jmp move_guard

do_move_west:
  dec %r10 # Actually do move
  movb $88, (%r13,%r10,1) # Leave an 'X' everywhere we go
  jmp move_guard

# Answer is the number of 'X's left in the map
left_map:
  imul %r9, %r9
  xor %r10, %r10
  xor %rax, %rax
  xor %rcx, %rcx
count_x:
  movb (%r13,%r10,1), %cl
  inc %r10
  cmp $88, %cl # 'X'
  jne next
  inc %rax

next:
  cmp %r10, %r9
  jne count_x

  # Print result
  call write_result_to_output
  lea output, %rdi
  call print

  jmp exit

.include "common.asm"
.include "syscalls.asm"
