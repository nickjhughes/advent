.code64
.section .rodata
msg: .ascii "Hello, World!\n"
.set msglen, (. - msg)

.section .text
.global _start
_start:

# write(2)
mov $1, %rax
mov $1, %rdi
lea msg, %rsi
mov $msglen, %rdx
syscall

# exit(2)
mov $60, %rax
mov $0, %rdi 
syscall
