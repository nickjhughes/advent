# Exit the program with exit code 0
exit:
  mov $60, %rax # exit(2) = 60
  mov $0, %rdi  # status = 0
  syscall

# Print the string at (%rdi) with length %rsi to stdout
print:
   push %rax
   push %rcx
   push %r11
   push %rdi
   push %rsi
   push %rdx
   movq $1, %rax   # write(2) = 1
   movq %rsi, %rdx # count = %rsi
   movq %rdi, %rsi # buf = %rdi
   movq $1, %rdi   # fd = stdout
   syscall
   pop %rdx
   pop %rsi
   pop %rdi
   pop %r11
   pop %rcx
   pop %rax
   ret

# Open the input file with null-terminated path $filename and return file descriptor in %rax
open_input_file:
  push %rcx
  push %r11
  push %rdi
  push %rsi
  movq $2, %rax       # open = 2
  leaq filename, %rdi # pathname = %rdi
  movq $0, %rsi       # flags = O_RDONLY
  syscall
  pop %rsi
  pop %rdi
  pop %r11
  pop %rcx
  # If open failed, print error and exit
  cmp $0, %rax
  jge open_input_file_success
  lea input_file_open_error, %rdi
  movq $input_file_open_error_len, %rsi
  call print
  jmp exit
open_input_file_success:
  ret

# Map the file with descriptor %rdi into memory and return the mapped address in %rax
mmap_input_file:
  push %rcx
  push %r11
  push %rdi
  push %rsi
  push %rdx
  push %r10
  push %r8
  push %r9
  movq $9, %rax      # mmap = 9
  movq %rdi, %r8     # fd = %r15
  movq $0, %rdi      # address = null
  movq $409600, %rsi # length = 100 * 4096 byte pages
  movq $1, %rdx      # prot = PROT_READ
  movq $2, %r10      # flags = MAP_PRIVATE
  movq $0, %r9       # offset = 0
  syscall
  pop %r9
  pop %r8
  pop %r10
  pop %rdx
  pop %rsi
  pop %rdi
  pop %r11
  pop %rcx
  ret

# Map some memory via a MAP_ANONYMOUS call to mmap
mmap_memory:
  push %rcx
  push %r11
  push %rdi
  push %rsi
  push %rdx
  push %r10
  push %r8
  push %r9
  movq $9, %rax     # mmap = 9
  movq $-1, %r8     # fd = -1
  movq $0, %rdi     # address = null
  movq $819200, %rsi # length = 200 * 4096 byte pages
  movq $3, %rdx     # prot = PROT_READ | PROT_WRITE
  movq $34, %r10    # flags = MAP_PRIVATE | MAP_ANONYMOUS
  movq $0, %r9      # offset = 0
  syscall
  pop %r9
  pop %r8
  pop %r10
  pop %rdx
  pop %rsi
  pop %rdi
  pop %r11
  pop %rcx
  ret
