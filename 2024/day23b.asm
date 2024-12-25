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
  movq %rax, 32(%rbp)  # Largest clique
  movq $0, 40(%rbp)    # Largest clique size
  call mmap_memory
  movq %rax, 48(%rbp)  # Current clique
  movq $0, 56(%rbp)    # Current clique size

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

  # For each vertex, add it to the current clique, then try to add every other vertex,
  # but only if they connect to each vertex already in the clique.
  # Once done, compare size to the current largest clique. If larger, replace.

  xor %r9, %r9         # Current clique starter vertex
  movq 48(%rbp), %r12  # Pointer to current clique
clique_loop:
  xor %r10, %r10       # Current clique size
  movw %r9w, (%r12,%r10,2)
  inc %r10
  movq %r10, 56(%rbp)  # Clear current clique

  # Loop over all other vertices
  movq $0, %r8
vertex_loop:
  # Check if %r8 is connected to all vertices currently in the clique
  xor %rax, %rax
  xor %r11, %r11 # Index into clique
clique_check_loop:
  cmp %r11, 56(%rbp)
  je clique_check_done
  xor %rax, %rax
  movw (%r12,%r11,2), %ax

  # Check if there is an edge between %r8w and %ax
  imul $676, %rax
  add %r8, %rax

  cmpb $1, (%r13,%rax,1)
  jne clique_check_dont_add

  inc %r11
  jmp clique_check_loop

clique_check_done:
  # Add vertex to clique
  movw %r8w, (%r12,%r10,2)
  inc %r10
  inc 56(%rbp)

clique_check_dont_add:
  inc %r8
  cmp $676, %r8
  jne vertex_loop

next_clique:
  # Compare size of current clique to size of current largest clique,
  # and replace if larger.

  cmp %r10, 40(%rbp)
  jge next
  movq 32(%rbp), %r14 # Pointer to largest clique
  movq %r10, 40(%rbp)
  xor %r11, %r11
  xor %r15, %r15
copy_clique_loop:
  movw (%r12,%r11,2), %r15w
  movw %r15w, (%r14,%r11,2)
  inc %r11
  cmp %r11, %r10
  jne copy_clique_loop

next:
  inc %r9
  cmp $676, %r9 # 676 = 26^2
  jne clique_loop

  call bubble_sort

  lea output, %rdi
  movq 32(%rbp), %r14 # Pointer to largest clique
  xor %r11, %r11
  xor %rax, %rax
  movq $26, %r8
  movq $1, %rsi
print_clique_loop:
  movw (%r14,%r11,2), %ax
  xor %rdx, %rdx
  divq %r8
  # First char is in %ax, second in %dx
  add $97, %rax
  movb %al, (%rdi)
  call print
  add $97, %rdx
  movb %dl, (%rdi)
  call print
  inc %r11
  cmp %r11, 40(%rbp)
  je print_newline
  movb $44, (%rdi)
  call print
  jmp print_clique_loop

print_newline:
  movb $10, (%rdi)
  call print
  jmp exit

# Sort the 40(%rbp) length u16 array at address 32(%rbp)
bubble_sort:
  push %r9
  push %r8
  push %rdi
  push %rcx
  push %rdx
  push %rax

  movq 32(%rbp), %rax
  movq 40(%rbp), %r9
  dec %r9

bubble_sort_repeat:
  xor %r8, %r8   # Number of swaps this loop
  xor %rdi, %rdi # Loop index

bubble_sort_loop:
  # Load element %rdi and %rdi+1 into registers
  movw (%rax,%rdi,2), %cx
  inc %rdi
  movw (%rax,%rdi,2), %dx

  # Compare and swap if necessary
  cmp %dx, %cx
  jle bubble_sort_next
  movw %cx, (%rax,%rdi,2)
  dec %rdi
  movw %dx, (%rax,%rdi,2)
  inc %rdi
  inc %r8

bubble_sort_next:
  cmp %rdi, %r9
  jne bubble_sort_loop

  # If any swaps occurred, repeat the loop
  cmp $0, %r8
  jne bubble_sort_repeat

  pop %rax
  pop %rdx
  pop %rcx
  pop %rdi
  pop %r8
  pop %r9
  ret

.include "common.asm"
.include "syscalls.asm"
