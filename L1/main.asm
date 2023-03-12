segment .data
output: db `../../L2/main.asm`, 0

segment .data
code: db `segment .data\nhello: db 'Hello World!', 10, 0\n\nsegment .text\nglobal _start\n_start:\n\nmov rdx, 13\nmov rsi, hello\nmov rdi, 0\nmov rax, 0x01\nsyscall\n\nmov rax, 0x3c\nsyscall\nret\n`, 0

global _start
segment .text
_start:
push rbp
mov rbp, rsp
sub rsp, 32
mov QWORD[rbp - 32], 0
mov rax, output
mov QWORD[rbp - 32 + 8], rax
mov rax, code
mov QWORD[rbp - 32 + 16], rax
mov QWORD[rbp - 32 + 24], 171
mov rdi, QWORD[rbp - 32 + 8]
call create_file
mov QWORD[rbp - 32], rax
mov rdx, QWORD[rbp - 32 + 24]
mov rsi, QWORD[rbp - 32 + 16]
mov rdi, QWORD[rbp - 32]
call write_file
mov rdi, QWORD[rbp - 32]
call close_file
call exit
add rsp, 32
pop rbp
ret

segment .text
exit:
push rbp
mov rbp, rsp
mov rax, 0x3c
syscall
pop rbp
ret

segment .text
create_file:
push rbp
mov rbp, rsp
sub rsp, 8
mov QWORD[rbp - 8], rdi
mov rax, 0100 | 02
push rax
mov rax, 0200 | 0400
push rax
pop rdx
pop rsi
mov rdi, QWORD[rbp - 8]
mov rax, 0x02
syscall
add rsp, 8
pop rbp
ret

segment .text
close_file:
push rbp
mov rbp, rsp
sub rsp, 8
mov QWORD[rbp - 8], rdi
mov rdi, QWORD[rbp - 8]
mov rax, 0x03
syscall
add rsp, 8
pop rbp
ret

segment .text
write_file:
push rbp
mov rbp, rsp
sub rsp, 24
mov QWORD[rbp - 8], rdi
mov QWORD[rbp - 16], rsi
mov QWORD[rbp - 24], rdx
mov rdx, QWORD[rbp - 24]
mov rsi, QWORD[rbp - 16]
mov rdi, QWORD[rbp - 8]
mov rax, 0x01
syscall
add rsp, 24
pop rbp
ret

segment .text
print:
push rbp
mov rbp, rsp
sub rsp, 16
mov QWORD[rbp - 8], rdi
mov QWORD[rbp - 16], rsi
mov rdx, QWORD[rbp - 16]
mov rsi, QWORD[rbp - 8]
mov rdi, 0
mov rax, 0x01
syscall
add rsp, 16
pop rbp
ret