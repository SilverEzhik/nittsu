extern kmain

global start

section .text

bits 32 ; 32 bit protected mode
start:
    ; point first p4 table entry to the first entry in p3 table
    mov eax, p3_table
    or eax, 0b11 ; set present and writable bits
    mov dword [p4_table + 0], eax ; write to zeroth entry
    ; repeat the same but with the p2 table
    mov eax, p2_table
    or eax, 0b11 ; set present and writable bits
    mov dword [p3_table + 0], eax ; write to zeroth entry

    ; fill p2
    mov ecx, 0 ; counter
    .map_p2_table:
        mov eax, 0x200000 ; 2MiB
        mul ecx ; multiply eax * ecx, store in eax
        or eax, 0b10000011 ; huge page, present, writable bits
        mov [p2_table + ecx * 8], eax ; each entry is 8 bytes

        inc ecx
        cmp ecx, 512
        jne .map_p2_table
    ; .map_p2_table loop

    ; move p4 table address to cr3
    mov eax, p4_table
    mov cr3, eax
    
    ; enable PAE   
    mov eax, cr4
    or eax, 1 << 5 ; 5th bit
    mov cr4, eax

    ; set long mode bit
    mov ecx, 0xC0000080
    rdmsr ; read 'model specific register'
    or eax, 1 << 8
    wrmsr ; write 'model specific register'

    ; enable paging
    mov eax, cr0
    or eax, 1 << 31
    or eax, 1 << 16
    mov cr0, eax

    ; gdt
    lgdt [gdt64.pointer]
    mov ax, gdt64.data
    mov ss, ax ; stack segment
    mov ds, ax ;  data segment
    mov es, ax ; extra segment

    ; long mode jump
    jmp gdt64.code:kmain
    

section .bss

align 4096
p4_table:
    resb 4096
p3_table:
    resb 4096
p2_table:
    resb 4096

section .rodata

gdt64:
    dq 0 ; quad word (64 bits)
.code: equ $ - gdt64 ; reference offset
    dq (1<<44) | (1<<47) | (1<<41) | (1<<43) | (1<<53)
    ; 44 - descriptor type, 1 for code/data segments
    ; 47 - present
    ; 41 - r/w, in code segments 1 - readable, in data segments - writable
    ; 43 - executable
    ; 53 - 64 bit
.data: equ $ - gdt64
    dq (1<<44) | (1<<47) | (1<<41)
.pointer:
    dw .pointer - gdt64 - 1 ; length
    dq gdt64
