string output "../../L2/main.asm"
string code
"segment .data
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
"

public fn _start() (
    handler: usize = create_file$(output);
    write_file$(handler, code, 171);
    close_file$(handler);
    exit$()
)

struct Some(x: usize, y: usize)

private fn test(x: Some) ()

private fn exit() ( 
    syscall$(0x3c)
)

private fn create_file(path: usize:ptr) (
    syscall$(0x02, path, 02 | 0100, 0400 | 0200)
)

private fn close_file(handler: usize) (
    syscall$(0x03, handler)
)

private fn write_file(handler: usize, buffer: usize:ptr, length: usize) (
    syscall$(0x01, handler, buffer, length)
)

private fn print(message: usize:ptr, length: usize) (
    syscall$(0x01, 0, message, length)
)