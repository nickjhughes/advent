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
  mov %r10, %rbx # Guard position index

  movb $46, (%r13,%r10,1) # '.'
  inc %r10
  jmp parse_char

end_of_line:
  # Increment row count
  inc %r9
  jmp parse_char

end_of_input:
  # Store history map after main map (at %r13 + %r9 * %r9), store address at %rdi
  # Store %r9 * %r9 in %rsi
  push %r9
  imul %r9, %r9
  mov %r9, %rsi
  add %r13, %r9
  mov %r9, %rdi
  pop %r9

  # Current new obstacle location
  xor %rbp, %rbp

  # Store overall result in %rax
  xor %rax, %rax

reset_map:
  xor %r10, %r10
reset_loop:
  movb $0, (%rdi,%r10,1)
  inc %r10
  cmp %r10, %rsi
  jne reset_loop

  mov %rbx, %r10 # Index of guard
  # Directions: North = 1, East = 2, South = 4, West = 8
  mov $1, %r14 # Direction of guard

  movb $1, (%rdi,%r10,1) # Store initial position in history map

# Move the guard through the map, updating the history map where they've been
move_guard:
  cmp $1, %r14
  je move_north
  cmp $2, %r14
  je move_east
  cmp $4, %r14
  je move_south
  cmp $8, %r14
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
  cmp %r15, %rbp
  je rotate_clockwise
  xor %rcx, %rcx
  movb (%r13,%r15,1), %cl
  cmp $35, %cl # '#'
  je rotate_clockwise

  # Actually do move
  sub %r9, %r10
  # Update history map
  movb (%rdi,%r10,1), %cl
  and %r14b, %cl
  cmp %r14b, %cl
  je found_loop # We've been in this place and direction before
  xor %r14b, %cl
  movb %cl, (%rdi,%r10,1)
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
  cmp %r15, %rbp
  je rotate_clockwise
  xor %rcx, %rcx
  movb (%r13,%r15,1), %cl
  cmp $35, %cl # '#'
  je rotate_clockwise

  # Actually do move
  inc %r10
  # Update history map
  movb (%rdi,%r10,1), %cl
  and %r14b, %cl
  cmp %r14b, %cl
  je found_loop # We've been in this place and direction before
  xor %r14b, %cl
  movb %cl, (%rdi,%r10,1)
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
  cmp %r15, %rbp
  je rotate_clockwise
  xor %rcx, %rcx
  movb (%r13,%r15,1), %cl
  cmp $35, %cl # '#'
  je rotate_clockwise

  # Actually do move
  add %r9, %r10
  # Update history map
  movb (%rdi,%r10,1), %cl
  and %r14b, %cl
  cmp %r14b, %cl
  je found_loop # We've been in this place and direction before
  xor %r14b, %cl
  movb %cl, (%rdi,%r10,1)
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
  cmp %r15, %rbp
  je rotate_clockwise
  xor %rcx, %rcx
  movb (%r13,%r15,1), %cl
  cmp $35, %cl # '#'
  je rotate_clockwise

  # Actually do move
  dec %r10
  # Update history map
  movb (%rdi,%r10,1), %cl
  and %r14b, %cl
  cmp %r14b, %cl
  je found_loop # We've been in this place and direction before
  xor %r14b, %cl
  movb %cl, (%rdi,%r10,1)
  jmp move_guard

rotate_clockwise:
  shl $1, %r14
  cmp $16, %r14
  jne move_guard
  mov $1, %r14
  jmp move_guard

found_loop:
  # Increment loop count and move onto next obstacle
  inc %rax
left_map:
  # Move onto next obstacle
  inc %rbp
  cmp %rbp, %rsi
  je print_result # Checked all obstacle positions
  cmp %rbp, %rbx
  je left_map # Can't place obstacle at guard starting position
  xor %rcx, %rcx
  movb (%r13,%rbp,1), %cl
  cmp $35, %cl # '#'
  je left_map # Already an obstacle here, so skip to next
  jmp reset_map

print_result:
  call write_result_to_output
  lea output, %rdi
  call print

  jmp exit

.include "common.asm"
.include "syscalls.asm"
