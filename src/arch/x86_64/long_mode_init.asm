global long_mode_start

section .text
bits 64
long_mode_start:
    ; call kernel main
    extern kernel_main
    call kernel_main

    ; print `OKAY` to the screen
    mov rax, 0x2f592f412f4b2f4f
    mov qword [0xb8000], rax
    hlt