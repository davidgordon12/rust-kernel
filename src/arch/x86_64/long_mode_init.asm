global long_mode_start

section .text
bits 64
long_mode_start:
    ; call kernel main
    extern kernel_main
    call kernel_main
    hlt