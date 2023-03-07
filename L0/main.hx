string hello "Hello World!\n"
string output "../../L2/main.asm"
string code "global _start\n_start:\nret\n"

public fn _start() (
    handler: usize = create_file$(output);
    write_file$(handler, code, 26);
    close_file$(handler);
    exit$()
)

private fn exit() (
    syscall$(0x3c)
)

public fn create_file(path: usize:ptr) (
    syscall$(0x02, path, 02 | 0100, 0400 | 0200)
)

public fn close_file(handler: usize) (
    syscall$(0x03, handler)
)

public fn write_file(handler: usize, buffer: usize:ptr, length: usize) (
    syscall$(0x01, handler, buffer, length)
)

public fn print(message: usize:ptr, length: usize) (
    syscall$(0x01, 0, message, length)
)