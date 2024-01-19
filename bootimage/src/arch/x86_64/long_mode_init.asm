global long_mode_start

section .text
bits 64
long_mode_start:
    ; load 0 into all data segment registers
    mov ax, 0
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    ; print 'OKAY' to screen
    mov rax, 0x2f592f412f4b2f4f ; 'OKAY'
    mov qword [0xb8000], rax
    ; halt
    hlt