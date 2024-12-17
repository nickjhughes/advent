.code64

.section .rodata
filename: .asciz "input17"
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
  movq %rax, 8(%rbp)  # Pointer to instruction array
  movq $0, 16(%rbp)   # Instruction count

  # Parse input
  movq (%rbp), %r11   # Pointer into input
  xor %rcx, %rcx

  add $12, %r11 # Skip "Register A: "
  call parse_integer
  movq %rax, %r13
  add $13, %r11 # Skip "\nRegister B: "
  call parse_integer
  movq %rax, %r14
  add $13, %r11 # Skip "\nRegister C: "
  call parse_integer
  movq %rax, %r15
  add $11, %r11 # Skip "\n\nProgram: "
  
  movq 8(%rbp), %r12 # Pointer to instruction array
  xor %r8, %r8       # Index into instruction array
parse_operation:
  movb (%r11), %cl
  add $2, %r11 # Skip comma or newline
  cmp $0, %cl
  je end_of_input
  sub $48, %cl
  movb %cl, (%r12,%r8,1)
  inc %r8
  jmp parse_operation

end_of_input:
  movq %r8, 16(%rbp) # Instruction count
  xor %rdi, %rdi     # Instruction pointer
  xor %rdx, %rdx
execute_instruction:
  cmp %rdi, %r8
  je halt
  xor %rcx, %rcx
  movb (%r12,%rdi,1), %cl
  cmp $0, %cl
  je execute_adv
  cmp $1, %cl
  je execute_bxl
  cmp $2, %cl
  je execute_bst
  cmp $3, %cl
  je execute_jnz
  cmp $4, %cl
  je execute_bxc
  cmp $5, %cl
  je execute_out
  cmp $6, %cl
  je execute_bdv
  cmp $7, %cl
  je execute_cdv
execute_adv:
  # The adv instruction (opcode 0) performs division. The numerator is the value in the A register.
  # The denominator is found by raising 2 to the power of the instruction's combo operand. (So, an
  # operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.) The result of
  # the division operation is truncated to an integer and then written to the A register.
  movq %r13, %rax
  inc %rdi
  call calc_combo_operand
  movq %rdx, %rcx
  movq $1, %rdx
  shl %cl, %rdx
  movq %rdx, %rcx
  xor %rdx, %rdx
  divq %rcx
  movq %rax, %r13
  jmp execute_instruction
execute_bxl:
  # The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's
  # literal operand, then stores the result in register B.
  inc %rdi
  movb (%r12,%rdi,1), %dl
  inc %rdi
  xorq %rdx, %r14
  jmp execute_instruction
execute_bst:
  # The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby
  # keeping only its lowest 3 bits), then writes that value to the B register.
  inc %rdi
  call calc_combo_operand
  and $0b111, %rdx
  movq %rdx, %r14
  jmp execute_instruction
execute_jnz:
  # The jnz instruction (opcode 3) does nothing if the A register is 0. However, if the A register
  # is not zero, it jumps by setting the instruction pointer to the value of its literal operand;
  # if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
  inc %rdi
  movb (%r12,%rdi,1), %dl
  inc %rdi
  cmp $0, %r13
  je execute_instruction
  movq %rdx, %rdi
  jmp execute_instruction
execute_bxc:
  # The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then
  # stores the result in register B. (For legacy reasons, this instruction reads an operand but
  # ignores it.)
  add $2, %rdi
  xorq %r15, %r14
  jmp execute_instruction
execute_out:
  # The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs
  # that value. (If a program outputs multiple values, they are separated by commas.)
  inc %rdi
  call calc_combo_operand
  and $0b111, %rdx
  push %rdi
  lea output, %rdi
  movq $2, %rsi
  add $48, %dl
  movb %dl, (%rdi)
  inc %rdi
  movb $44, (%rdi)
  dec %rdi
  call print
  pop %rdi
  jmp execute_instruction
execute_bdv:
  # The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is
  # stored in the B register. (The numerator is still read from the A register.)
  movq %r13, %rax
  inc %rdi
  call calc_combo_operand
  movq %rdx, %rcx
  movq $1, %rdx
  shl %cl, %rdx
  movq %rdx, %rcx
  xor %rdx, %rdx
  divq %rcx
  movq %rax, %r14
  jmp execute_instruction
execute_cdv:
  # The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is
  # stored in the C register. (The numerator is still read from the A register.)
  movq %r13, %rax
  inc %rdi
  call calc_combo_operand
  movq %rdx, %rcx
  movq $1, %rdx
  shl %cl, %rdx
  movq %rdx, %rcx
  xor %rdx, %rdx
  divq %rcx
  movq %rax, %r15
  jmp execute_instruction

halt:
  jmp exit

# Calculate the combo operand at (%r12,%rdi,1), and store in %rdx.
# - Combo operands 0 through 3 represent literal values 0 through 3.
# - Combo operand 4 represents the value of register A.
# - Combo operand 5 represents the value of register B.
# - Combo operand 6 represents the value of register C.
# - Combo operand 7 is reserved and will not appear in valid programs.
calc_combo_operand:
  xor %rdx, %rdx
  movb (%r12,%rdi,1), %dl
  inc %rdi
  cmp $3, %dl
  jbe literal_combo_operand
  cmp $4, %dl
  je reg_a_combo_operand
  cmp $5, %dl
  je reg_b_combo_operand
  cmp $6, %dl
  je reg_c_combo_operand
reg_a_combo_operand:
  movq %r13, %rdx
  ret
reg_b_combo_operand:
  movq %r14, %rdx
  ret
reg_c_combo_operand:
  movq %r15, %rdx
  ret
literal_combo_operand:
  ret

# Parse an integer at %r11 into %r13
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

.include "common.asm"
.include "syscalls.asm"
