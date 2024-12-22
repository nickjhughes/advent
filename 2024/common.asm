# Write the u64 stored in %rax as an ASCII string into (output), with a newline at the end
# Leaves the length of the string in %rsi
write_result_to_output:
  push %rax
  push %rdx
  push %rcx
  push %rdi
  push %r8

  push %rax

  # First determine the number of decimal digits in the number, stored in %r8
  xor %r8, %r8
  movq $10, %rcx
count_digit:
  inc %r8
  xor %rdx, %rdx
  divq %rcx
  cmp $0, %rax
  jne count_digit

  pop %rax

  # %rsi is passed to the print subroutine as the length of the string
  mov %r8, %rsi
  inc %rsi # Add one for newline

  # Now read out digits by dividing by 10 and taking remainder, writing digits in reverse
  # %rdi is index into output string
write_digit_start:
  lea output, %rdi
  add %r8, %rdi
write_digit:
  decq %rdi
  xor %rdx, %rdx
  divq %rcx
  add $48, %rdx
  movb %dl, (%rdi)
  cmp $output, %rdi
  jne write_digit

  movb $10, (%rdi,%r8,1) # End with a newline

  pop %r8
  pop %rdi
  pop %rcx
  pop %rdx
  pop %rax
  ret

# Write the i64 stored in %rax as an ASCII string into (output), with a newline at the end.
# Leaves the length of the string in %rsi.
write_signed_result_to_output:
  push %r15
  push %rax
  push %rdx
  push %rcx
  push %rdi
  push %r8

  xor %rsi, %rsi
  lea output, %r15

  # First determine if negative, and if so print a negative sign and negate the input
  test %rax, %rax
  jns find_length
  inc %rsi
  movb $45, (%r15)
  inc %r15
  neg %rax

find_length:
  push %rax

  # First determine the number of decimal digits in the number, stored in %r8
  xor %r8, %r8
  movq $10, %rcx
count_unsigned_digit:
  inc %r8
  xor %rdx, %rdx
  divq %rcx
  cmp $0, %rax
  jne count_unsigned_digit

  pop %rax

  # %rsi is passed to the print subroutine as the length of the string
  add %r8, %rsi
  inc %rsi # Add one for newline

  # Now read out digits by dividing by 10 and taking remainder, writing digits in reverse
  # %rdi is index into output string
write_unsigned_digit_start:
  movq %r15, %rdi
  add %r8, %rdi
write_unsigned_digit:
  decq %rdi
  xor %rdx, %rdx
  divq %rcx
  add $48, %rdx
  movb %dl, (%rdi)
  cmp %r15, %rdi
  jne write_unsigned_digit

  movb $10, (%rdi,%r8,1) # End with a newline

  pop %r8
  pop %rdi
  pop %rcx
  pop %rdx
  pop %rax
  pop %r15
  ret
