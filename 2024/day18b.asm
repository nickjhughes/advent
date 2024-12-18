.code64

.section .rodata
filename: .asciz "input18"
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
  sub $80, %rsp

  # Open and mmap null-terminated input file to address
  call open_input_file
  movq %rax, %rdi
  call mmap_input_file
  movq %rax, (%rbp)   # Pointer to input

  call mmap_memory
  movq %rax, 8(%rbp)  # Pointer to map
  movq $71, 16(%rbp)  # Map side length
  movq $0, 24(%rbp)   # Number of bytes to count
  call mmap_memory
  movq %rax, 32(%rbp) # Distance values
  call mmap_memory
  movq %rax, 40(%rbp) # Best previous indices
  call mmap_memory
  movq %rax, 48(%rbp) # Queue
  movq $0, 56(%rbp)   # Queue initial size
  movq $0, 64(%rbp)   # Lower bound on bytes to count
  movq $0, 72(%rbp)   # Upper bound on bytes to count

  # Parse input
  movq (%rbp), %r11   # Pointer into input
  movq 8(%rbp), %r12  # Pointer to map
  movq 16(%rbp), %r9  # Map side length
  xor %rcx, %rcx
  xor %r8, %r8        # Byte count
parse_line:
  movb (%r11), %cl
  cmp $0, %cl
  je end_of_input
  call parse_integer
  movq %rax, %rbx
  inc %r11 # Skip comma
  call parse_integer
  inc %r11 # Skip newline

  # Mark location as blocked
  imul %r9, %rax
  add %rbx, %rax
  inc %r8
  movq %r8, (%r12,%rax,8)
  jmp parse_line

end_of_input:
  movq %r8, 72(%rbp) # Total number of bytes

  movq %r9, %rdi
  imul %rdi, %rdi # Total number of nodes

dijkstra:
  # Consider number of bytes in the middle of the current bounds
  movq 72(%rbp), %rax
  sub 64(%rbp), %rax
  shr $1, %rax
  add 64(%rbp), %rax
  movq %rax, 24(%rbp)

  # Set all distance values to infinity (-1),
  # previous nodes to undefined (-1)
  movq 32(%rbp), %r13
  movq 40(%rbp), %r14
  xor %r10, %r10
clear_loop:
  movq $-1, (%r13,%r10,8)
  movq $-1, (%r14,%r10,8)
  inc %r10
  cmp %r10, %rdi
  jne clear_loop
  
  # Set dist[start] = 0
  movq $0, %r10
  movq $0, (%r13,%r10,8)

  # Add all open spaces to the queue
  movq $0, 56(%rbp) # Clear queue
  movq 48(%rbp), %r15
  xor %r10, %r10
  xor %r8, %r8   # Queue index
add_nodes_loop:
  cmp %r10, %rdi
  je add_nodes_done
  movq (%r12,%r10,8), %rcx
  inc %r10
  cmp $0, %rcx
  je add_node
  cmp %rcx, 24(%rbp)
  jge add_nodes_loop
add_node:
  dec %r10
  movq %r10, (%r15,%r8,8)
  inc %r8
  inc %r10
  jmp add_nodes_loop
add_nodes_done:
  movq %r8, 56(%rbp) # Save queue size

dijkstra_loop:
  movq 56(%rbp), %r8
  cmp $0, %r8
  je queue_empty
  
  call find_min_dist_node
  movq %rax, %rcx # Current node

# Can move north, south, east, or west, as long as they're in bounds and not blocked
north:
  movq %rcx, %rdx
  sub %r9, %rdx
  cmp $0, %rdx
  jl south # Out of bounds
  movq (%r12,%rdx,8), %rax
  cmp $0, %rax
  je north_clear
  cmp %rax, 24(%rbp)
  jg south # Blocked
north_clear:
  call is_in_queue
  cmp $1, %rax
  jne south # Not in queue
  # Calc alt distance
  movq (%r13,%rcx,8), %rbx
  inc %rbx
  movq (%r13,%rdx,8), %rax
  cmp %rax, %rbx
  jae south # Alt dist not smaller
  movq %rbx, (%r13,%rdx,8)
  movq %rcx, (%r14,%rdx,8)

south:
  movq %rcx, %rdx
  add %r9, %rdx
  cmp %rdi, %rdx
  jge east # Out of bounds
  movq (%r12,%rdx,8), %rax
  cmp $0, %rax
  je south_clear
  cmp %rax, 24(%rbp)
  jg east # Blocked
south_clear:
  call is_in_queue
  cmp $1, %rax
  jne east # Not in queue
  # Calc alt distance
  movq (%r13,%rcx,8), %rbx
  inc %rbx
  movq (%r13,%rdx,8), %rax
  cmp %rax, %rbx
  jae east # Alt dist not smaller
  movq %rbx, (%r13,%rdx,8)
  movq %rcx, (%r14,%rdx,8)

east:
  movq %rcx, %rax
  xor %rdx, %rdx
  divq %r9
  movq %r9, %r15
  dec %r15
  cmp %r15, %rdx
  je west # Out of bounds
  movq %rcx, %rdx
  inc %rdx
  movq (%r12,%rdx,8), %rax
  cmp $0, %rax
  je east_clear
  cmp %rax, 24(%rbp)
  jg west # Blocked
east_clear:
  call is_in_queue
  cmp $1, %rax
  jne west # Not in queue
  # Calc alt distance
  movq (%r13,%rcx,8), %rbx
  inc %rbx
  movq (%r13,%rdx,8), %rax
  cmp %rax, %rbx
  jae west # Alt dist not smaller
  movq %rbx, (%r13,%rdx,8)
  movq %rcx, (%r14,%rdx,8)

west:
  movq %rcx, %rax
  xor %rdx, %rdx
  divq %r9
  cmp $0, %rdx
  je dijkstra_loop # Out of bounds
  movq %rcx, %rdx
  dec %rdx
  movq (%r12,%rdx,8), %rax
  cmp $0, %rax
  je west_clear
  cmp %rax, 24(%rbp)
  jg dijkstra_loop # Blocked
west_clear:
  call is_in_queue
  cmp $1, %rax
  jne dijkstra_loop # Not in queue
  # Calc alt distance
  movq (%r13,%rcx,8), %rbx
  inc %rbx
  movq (%r13,%rdx,8), %rax
  cmp %rax, %rbx
  jae dijkstra_loop # Alt dist not smaller
  movq %rbx, (%r13,%rdx,8)
  movq %rcx, (%r14,%rdx,8)
  jmp dijkstra_loop

queue_empty:
  dec %rdi
  movq (%r13,%rdi,8), %rax
  inc %rdi
  cmp $-1, %rax
  je no_path
  movq 24(%rbp), %r8
  movq %r8, 64(%rbp) # Change lower bound
  jmp next_dijkstra
no_path:
  movq 24(%rbp), %r8
  movq %r8, 72(%rbp) # Change upper bound
next_dijkstra:
  movq 64(%rbp), %rax
  cmp %rax, 72(%rbp)
  je print_result
  inc %rax
  cmp %rax, 72(%rbp)
  jne dijkstra

print_result:
  xor %r10, %r10
byte_loop:
  movq (%r12,%r10,8), %rax
  cmp %rax, 72(%rbp)
  je byte_found
  inc %r10
  cmp %r10, %rdi
  jne byte_loop

byte_found:
  movq %r10, %rax
  xor %rdx, %rdx
  div %r9, %rax
  xchg %rax, %rdx
  # x-coord
  call write_result_to_output
  dec %rsi # Don't print newline
  lea output, %rdi
  call print
  # Comma
  movq $1, %rsi
  movq $44, (%rdi)
  call print
  # y-coord
  movq %rdx, %rax
  call write_result_to_output
  lea output, %rdi
  call print

  jmp exit

# Parse an integer at %r11 into %rax
parse_integer:
  xor %rax, %rax
  xor %rcx, %rcx
parse_digit:
  movb (%r11), %cl
  cmp $48, %cl # '0'
  jl parse_integer_done
  cmp $57, %cl # '9'
  jg parse_integer_done
  inc %r11
  sub $48, %cl
  imul $10, %rax
  add %rcx, %rax
  jmp parse_digit
parse_integer_done:
  ret

# Determine whether the node in %rdx is in the queue. If so, %rax = 1, else %rax = 0.
is_in_queue:
  push %r8
  push %r10
  push %r15
  xor %rax, %rax
  movq 48(%rbp), %r15 # Pointer to queue
  movq 56(%rbp), %r8 # Queue size
  cmp $0, %r8
  je find_loop_done
  xor %r10, %r10
find_loop:
  cmp %rdx, (%r15,%r10,8)
  jne find_loop_next
  inc %rax
  jmp find_loop_done
find_loop_next:
  inc %r10
  cmp %r10, %r8
  jne find_loop
find_loop_done:
  pop %r15
  pop %r10
  pop %r8
  ret

# Find the node in the queue with minimum distance, remove it, and return in %rax.
find_min_dist_node:
  push %rbx
  push %rcx
  push %rdx
  push %r8
  push %r9
  push %r10
  push %r13
  push %r15

  movq 48(%rbp), %r15 # Pointer to queue
  movq 32(%rbp), %r13 # Pointer to dist array
  movq 56(%rbp), %r8 # Queue size
  movq $-1, %rbx # Current min dist
  movq $-1, %rax # Current min dist node
  movq $-1, %rcx # Index of min in queue
  xor %r9, %r9 # Queue index
min_dist_loop:
  movq (%r15,%r9,8), %r10
  movq (%r13,%r10,8), %rdx
  cmp %rdx, %rbx
  jb not_less
  # New minimum
  movq %rdx, %rbx
  movq %r10, %rax
  movq %r9, %rcx
not_less:
  inc %r9
  cmp %r9, %r8
  jne min_dist_loop

  cmp $-1, %rcx
  je not_found

  # Remove the minimum from the queue and resize it
  movq %rcx, %r9
remove_loop:
  inc %r9
  cmp %r9, %r8
  je remove_loop_done
  movq (%r15,%r9,8), %rdx
  movq %rdx, (%r15,%rcx,8)
  inc %rcx
  jmp remove_loop
remove_loop_done:
  decq 56(%rbp)
not_found:
  pop %r15
  pop %r13
  pop %r10
  pop %r9
  pop %r8
  pop %rdx
  pop %rcx
  pop %rbx
  ret

print_map:
  push %rax
  push %rcx
  push %rdx
  push %rdi
  push %rsi
  push %r9
  push %r8
  push %r10
  push %r12
  push %r15
  movq 8(%rbp), %r12
  movq 16(%rbp), %r9
  lea output, %rdi
  movq $1, %rsi
  movq %r9, %r8  # Total size of map
  imul %r8, %r8
  xor %r10, %r10 # Map index
print_loop:
  cmp %r10, %r8
  je print_done
  movq (%r12,%r10,8), %rcx
  inc %r10
  cmp $0, %cl
  je print_blank
  movb $35, (%rdi) # '#'
  jmp do_print
print_blank:
  movb $46, (%rdi) # '.'
do_print:
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
  pop %r15
  pop %r12
  pop %r10
  pop %r8
  pop %r9
  pop %rsi
  pop %rdi
  pop %rdx
  pop %rcx
  pop %rax
  ret

.include "common.asm"
.include "syscalls.asm"
