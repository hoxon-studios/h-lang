segment .data
hello: db `Hello World!\n`, 0

segment .data
output: db `../../L2/main.asm`, 0

segment .data
code: db `global _start\n_start:\nret\n`, 0

global _start
segment .text
_start:
push rbp
mov rbp, rsp
sub rsp, 8
mov rdi, output
call create_file
mov QWORD[rbp - 8], rax
mov rdx, 26
mov rsi, code
mov rdi, QWORD[rbp - 8]
call write_file
mov rdi, QWORD[rbp - 8]
call close_file
call exit
add rsp, 8
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

global create_file
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

global close_file
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

global write_file
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

global print
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