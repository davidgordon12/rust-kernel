global start
extern long_mode_start

section .text
bits 32
start:
    mov esp, stack_top

    call check_multiboot
    call check_cpuid
    call check_long_mode

    call setup_page_tables
    call enable_paging

    ; load the 64-bit GDT
    lgdt [gdt64.pointer]

    ; finally enter 64bit mode
    jmp gdt64.code:long_mode_start

    ; print `OK` to the screen
    mov dword [0xb8000], 0x2f4b2f4f
    hlt

check_multiboot:            
    cmp eax, 0x36d76289     ; magic value (multiboot 1.6 spec)
    jne .no_multiboot
    ret
.no_multiboot:              ; not multiboot compliant
    mov al, "0"             
    jmp error

check_cpuid:                
    ; OSDev Wiki `Detection of CPUID` from `Setting up Long Mode`
    ; Check if CPUID is supported by attempting to flip the ID bit (bit 21) in
    ; the FLAGS register. If we can flip it, CPUID is available.

    ; Copy FLAGS in to EAX via stack
    pushfd
    pop eax

    ; Copy to ECX as well for comparing later on
    mov ecx, eax

    ; Flip the ID bit
    xor eax, 1 << 21

    ; Copy EAX to FLAGS via the stack
    push eax
    popfd

    ; Copy FLAGS back to EAX (with the flipped bit if CPUID is supported)
    pushfd
    pop eax

    ; Restore FLAGS from the old version stored in ECX (i.e. flipping the ID bit
    ; back if it was ever flipped).
    push ecx
    popfd

    ; Compare EAX and ECX. If they are equal then that means the bit wasn't
    ; flipped, and CPUID isn't supported.
    cmp eax, ecx
    je .no_cpuid
    ret
.no_cpuid:
    mov al, "1"
    jmp error

check_long_mode:
    mov eax, 0x80000000    ; Set the A-register to 0x80000000.
    cpuid                  ; CPU identification.
    cmp eax, 0x80000001    ; Compare the A-register with 0x80000001.
    jb .no_long_mode       ; It is less, there is no long mode.

    mov eax, 0x80000001    ; Set the A-register to 0x80000001.
    cpuid                  ; CPU identification.
    test edx, 1 << 29      ; Test if the LM-bit, which is bit 29, is set in the D-register.
    jz .no_long_mode       ; They aren't, there is no long mode.
    ret
.no_long_mode:
    mov al, "2"
    jmp error

setup_page_tables:
    ; map first PML-4 entry to PDP
    mov eax, pdp_table
    or  eax, 0b11        ; present, writeable
    mov [pml4_table], eax

    ; map first PDP Table entry to PD
    mov eax, pd_table
    or  eax, 0b11        ; present, writeable
    mov [pdp_table], eax

    ; map each PD entry to a huge 2MiB page
    mov ecx, 0          ; counter variable

.map_pd_table:
    ; map ecx-th PD entry to a huge page that starts at addr 2MiB*ecx
    mov eax, 0x20000    ; 2MiB
    mul ecx             ; start address of ecx-th page
    or  eax, 0b10000011 ; present + writeable + huge
    mov [pd_table + ecx * 8], eax ; map ecx-th entry

    inc ecx
    cmp ecx, 512        ; if counter == 512 then we are done mapping
    jne .map_pd_table   ; otherwise map next entry

    ret

enable_paging:
    ; load PML4 to cr3 register (CPU uses this to access the PML4 table)
    mov eax, pml4_table
    mov cr3, eax

    ; enable PAE-flag in cr4 (Physical Address Extension)
    mov eax, cr4
    or  eax, 1 << 5
    mov cr4, eax

    ; set the long mode bit in the EFER MSR (model specific register)
    mov ecx, 0xC0000080
    rdmsr
    or  eax, 1 << 8
    wrmsr

    ; enable paging in the cr0 register
    mov eax, cr0
    or  eax, 1 << 31
    mov cr0, eax

    ret

section .rodata
gdt64:
    dq 0 ; zero entry
.code equ $ - gdt64
    dq (1<<43) | (1<<44) | (1<<47) | (1<<53) ; code segment
.pointer:
    dw $ - gdt64 - 1
    dq gdt64

error:
    ; print `ERR:` followed by the error code
    mov dword [0xb8000], 0x4f524f45
    mov dword [0xb8000], 0x4f3a4f52
    mov dword [0xb8000], 0x4f204f20
    mov byte  [0xb800a], al
    hlt

section .bss
align 4096
pml4_table:
    resb 4096
pdp_table:
    resb 4096
pd_table:
    resb 4096
stack_bottom:
    ; reserve 64 bytes
    resb 64 
stack_top: