segment .data
hello: db 'Hello World!', 10, 0

segment .text
global _start
_start:

mov rdx, 13
mov rsi, hello
mov rdi, 0
mov rax, 0x01
syscall

mov rax, 0x3c
syscall
ret
