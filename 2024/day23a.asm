.code64

.section .rodata
filename: .asciz "input23"
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
  movq %rax, 8(%rbp)   # Pointer to list of edges
  movq $0, 16(%rbp)    # Number of edges
  call mmap_memory
  movq %rax, 24(%rbp)  # Pointer to adjacency matrix (size is 26^4)
  call mmap_memory
  movq %rax, 32(%rbp)  # List of triangles
  movq $0, 40(%rbp)    # Triangle count

  # Parse input
  movq (%rbp), %r11   # Pointer into input
  movq 8(%rbp), %r12  # Pointer to list of edges
  movq 24(%rbp), %r13 # Pointer to adjacency matrix
  xor %r10, %r10      # Edges list size
parse_edge:
  xor %rax, %rax
  xor %rbx, %rbx
  xor %rcx, %rcx
  xor %rdx, %rdx

  movb (%r11), %al
  inc %r11
  cmp $0, %al
  je end_of_input
  sub $97, %al # 'a' -> 0, etc
  imul $26, %ax
  movb (%r11), %bl
  inc %r11
  sub $97, %bl # 'a' -> 0, etc
  add %bx, %ax

  inc %r11 # Skip "-"
  movb (%r11), %cl
  inc %r11
  sub $97, %cl # 'a' -> 0, etc
  imul $26, %cx
  movb (%r11), %dl
  inc %r11
  sub $97, %dl # 'a' -> 0, etc
  add %dx, %cx

  inc %r11 # Skip newline

  # Add edge to list
  movw %ax, (%r12,%r10,2)
  inc %r10
  movw %cx, (%r12,%r10,2)
  inc %r10

  # Add edge to adjacency matrix (both directions)
  push %rax
  imul $676, %rax # 676 = 26^2
  add %rcx, %rax
  orq $1, (%r13,%rax,1)
  pop %rax
  imul $676, %rcx # 676 = 26^2
  add %rax, %rcx
  orq $1, (%r13,%rcx,1)

  jmp parse_edge

end_of_input:
  movq %r10, 16(%rbp) # Save (2x)edge count

  # For each edge, for each vertex, check if that vertex connects to each side of the edge
  # If so, we've found a triangle
  xor %r10, %r10 # Edge list index
edge_loop:
  xor %rax, %rax
  xor %rbx, %rbx
  movw (%r12,%r10,2), %ax
  inc %r10
  movw (%r12,%r10,2), %bx
  inc %r10

  xor %r9, %r9   # Current vertex
vertex_loop:
  # If none of %ax, %bx, %r9w start with a 't', then skip to next vertex
  xor %rdi, %rdi
  movq $26, %r8
  push %rax
  xor %rdx, %rdx
  divq %r8
  cmp $19, %rax
  pop %rax
  jne check_bx
  orq $1, %rdi
check_bx:
  push %rax
  movq %rbx, %rax
  xor %rdx, %rdx
  divq %r8
  cmp $19, %rax
  pop %rax
  jne check_r9w
  orq $1, %rdi
check_r9w:
  push %rax
  movq %r9, %rax
  xor %rdx, %rdx
  divq %r8
  cmp $19, %rax
  pop %rax
  jne check_done
  orq $1, %rdi
check_done:
  cmp $1, %rdi
  jne next_vertex_no_pop

  # Check if %ax-%r9w and %bx-%r9w are edges
  xor %r15, %r15
  movw %ax, %r15w
  imul $676, %r15
  add %r9, %r15
  cmpb $1, (%r13,%r15,1)
  jne next_vertex_no_pop

  xor %r15, %r15
  movw %bx, %r15w
  imul $676, %r15
  add %r9, %r15
  cmpb $1, (%r13,%r15,1)
  jne next_vertex_no_pop

  push %rax
  push %rbx
  push %r9
  # Now we know %ax, %bx, %r9w is a valid triangle
  # Sort vertices to get a unique identifier for the triangle
  cmp %r9w, %ax
  jle cmp_2
  xchg %rax, %r9
cmp_2:
  cmp %bx, %ax
  jle cmp_3
  xchg %rax, %rbx
cmp_3:
  cmp %r9w, %bx
  jle cmp_done
  xchg %rbx, %r9
cmp_done:
  # Add triangle to list if it isn't already in there
  call is_in_list
  cmp $1, %rdx
  je next_vertex

  movq 32(%rbp), %r14
  movq 40(%rbp), %rdi
  movw %ax, (%r14,%rdi,2)
  inc %rdi
  movw %bx, (%r14,%rdi,2)
  inc %rdi
  movw %r9w, (%r14,%rdi,2)
  addq $3, 40(%rbp)

next_vertex:
  pop %r9
  pop %rbx
  pop %rax
next_vertex_no_pop:
  inc %r9
  cmp $676, %r9 # 676 = 26^2
  jne vertex_loop

  cmp %r10, 16(%rbp)
  jne edge_loop

  movq 40(%rbp), %rax
  xor %rdx, %rdx
  movq $3, %r8
  divq %r8
  call write_result_to_output
  lea output, %rdi
  call print

  jmp exit

# Check if the sorted triangle %ax-%bx-%r9w is in the list. Return 0/1 in %rdx.
is_in_list:
  push %rcx
  push %r10
  push %r14
  movq 32(%rbp), %r14
  xor %r10, %r10
  xor %rcx, %rcx
  xor %rdx, %rdx
find_loop:
  cmp %r10, 40(%rbp)
  je find_loop_done
  movw (%r14,%r10,2), %cx
  inc %r10
  cmp %ax, %cx
  jne find_loop_next_1
  movw (%r14,%r10,2), %cx
  inc %r10
  cmp %bx, %cx
  jne find_loop_next_2
  movw (%r14,%r10,2), %cx
  inc %r10
  cmp %r9w, %cx
  jne find_loop_next_3
  inc %rdx
  jmp find_loop_done
find_loop_next_1:
  inc %r10
find_loop_next_2:
  inc %r10
find_loop_next_3:
  jne find_loop
find_loop_done:
  pop %r14
  pop %r10
  pop %rcx
  ret

.include "common.asm"
.include "syscalls.asm"
