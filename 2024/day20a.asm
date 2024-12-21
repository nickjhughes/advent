.code64

.section .rodata
filename: .asciz "input20"
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
sub $96, %rsp

# Open and mmap null-terminated input file to address
call open_input_file
movq %rax, %rdi
call mmap_input_file
movq %rax, (%rbp)   # Pointer to input

call mmap_memory
movq %rax, 8(%rbp)  # Pointer to map
call mmap_memory
movq %rax, 16(%rbp) # Distance values
call mmap_memory
movq %rax, 24(%rbp) # Best previous indices
movq $0, 32(%rbp)   # Side length of map
movq $0, 40(%rbp)   # Start index
movq $0, 48(%rbp)   # End index
call mmap_memory
movq %rax, 56(%rbp) # Queue
movq $0, 64(%rbp)   # Queue initial size
movq $0, 72(%rbp)   # No-cheat shortest path length
movq $0, 80(%rbp)   # Number of >= 100 saving cheats

# Parse input
movq (%rbp), %r11   # Pointer into input
movq 8(%rbp), %r12  # Pointer to map
movq %r9, %r9       # Row count
xor %r10, %r10      # Index into map
xor %rcx, %rcx
parse_char:
  movb (%r11), %cl
  inc %r11
  cmp $0, %cl
  je end_of_input
  cmp $10, %cl # '\n'
  je end_of_line
  cmp $83, %cl # 'S'
  je start_index
  cmp $69, %cl # 'E'
  je end_index
  movb %cl, (%r12,%r10,1)
  jmp next_char
start_index:
  movq %r10, 40(%rbp)      # Save start index
  movb $46, (%r12,%r10,1)  # Store '.' for blank space
  jmp next_char
end_index:
  movq %r10, 48(%rbp)      # Save end index
  movb $46, (%r12,%r10,1)  # Store '.' for blank space
next_char:
  inc %r10
  jmp parse_char
end_of_line:
  inc %r9 # Increment row count
  jmp parse_char

end_of_input:
  movq %r9, 32(%rbp)  # Save map size

  movq %r9, %rdi
  imul %rdi, %rdi
  sub %r9, %rdi
  dec %rdi            # One past the last wall to remove

  # Path length without cheats
  call dijkstra
  movq %rax, 72(%rbp)

  # Remove one non-border wall at a time, and compare resulting path length to non-cheat one
  movq %r9, %r10
  inc %r10
cheat_loop:
  cmp %rdi, %r10
  jge done

  movb (%r12,%r10,1), %cl
  cmp $35, %cl
  jne next_cheat

  movb $46, (%r12,%r10,1) # '.'
  call dijkstra

  # Compare to no-cheat path, print difference
  movq 72(%rbp), %rdx
  sub %rax, %rdx
  cmp $100, %rdx
  jl replace_wall
  inc 80(%rbp)
replace_wall:
  movb $35, (%r12,%r10,1) # '#'
next_cheat:
  inc %r10
  # Skip border walls so we don't have to do bounds checks
  movq %r10, %rax
  xor %rdx, %rdx
  divq %r9
  cmp $0, %rdx # First column, so skip to next
  je inc_cheat
  movq %r9, %r8
  dec %r8
  cmp %r8, %rdx # Last column so skip two
  jne cheat_loop
  inc %r10
inc_cheat:
  inc %r10
  jmp cheat_loop

done:
  movq 80(%rbp), %rax
  call write_result_to_output
  lea output, %rdi
  call print
  jmp exit

dijkstra:
  push %r10
  push %rdi
  movq 8(%rbp), %r12  # Pointer to map
  movq 32(%rbp), %r9  # Map size
  movq %r9, %rdi      # Total number of nodes
  imul %rdi, %rdi

  # Set all distance values to infinity (-1),
  # previous nodes to undefined (-1)
  movq 16(%rbp), %r13
  movq 24(%rbp), %r14
  xor %r10, %r10
clear_loop:
  movq $-1, (%r13,%r10,8)
  movq $-1, (%r14,%r10,8)
  inc %r10
  cmp %r10, %rdi
  jne clear_loop

  # Set dist[start] = 0
  movq 40(%rbp), %r10
  movq $0, (%r13,%r10,8)

  # Add all valid nodes (i.e., not a wall) to the queue
  movq 56(%rbp), %r15
  xor %r10, %r10
  xor %r8, %r8   # Queue index
add_nodes_loop:
  cmp %r10, %rdi
  je add_nodes_done
  movb (%r12,%r10,1), %cl
  inc %r10
  cmp $46, %cl # '.'
  jne add_nodes_loop
  dec %r10
  movq %r10, (%r15,%r8,8)
  inc %r10
  inc %r8
  jmp add_nodes_loop
add_nodes_done:
  movq %r8, 64(%rbp) # Save queue size

dijkstra_loop:
  movq 64(%rbp), %r8
  cmp $0, %r8
  je queue_empty

  call find_min_dist_node
  movq %rax, %rcx # Current node

north:
  movq %rcx, %rdx
  sub %r9, %rdx
  xor %rax, %rax
  movb (%r12,%rdx,1), %al
  cmp $35, %al # '#'
  je south # Blocked
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
  xor %rax, %rax
  movb (%r12,%rdx,1), %al
  cmp $35, %al # '#'
  je east # Blocked
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
  movq %rcx, %rdx
  inc %rdx
  xor %rax, %rax
  movb (%r12,%rdx,1), %al
  cmp $35, %al # '#'
  je west # Blocked
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
  movq %rcx, %rdx
  dec %rdx
  xor %rax, %rax
  movb (%r12,%rdx,1), %al
  cmp $35, %al # '#'
  je dijkstra_loop # Blocked
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
  movq 48(%rbp), %rdx
  movq (%r13,%rdx,8), %rax
  pop %rdi
  pop %r10
  ret

# Determine whether the node in %rdx is in the queue. If so, %rax = 1, else %rax = 0.
is_in_queue:
  push %r8
  push %r10
  push %r15
  xor %rax, %rax
  movq 64(%rbp), %r8 # Size of queue
  cmp $0, %r8
  je find_loop_done
  movq 56(%rbp), %r15 # Pointer to queue
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

  movq 56(%rbp), %r15 # Pointer to queue
  movq 16(%rbp), %r13 # Pointer to dist array
  movq 64(%rbp), %r8 # Queue size
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
  decq 64(%rbp)
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
  movq 32(%rbp), %r9
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
