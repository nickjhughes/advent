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
  # TODO: Store the current index for later, as the position of the guard

  movb $88, (%r13,%r10,1) # 'X'
  inc %r10
  jmp parse_char

end_of_line:
  # Increment row count
  inc %r9
  jmp parse_char

end_of_input:
  # TODO: Guard starts facing north, store direction in a register

  # TODO: Move the guard through the map, leaving 'X' where they've been

  # TODO: When they leave the map, count the 'X's in the map as the answer

  jmp exit

.include "common.asm"
.include "syscalls.asm"
