section .multiboot_header
header_start:
    dd 0xE85250D6 ; define double word
    dd 0          ; protected mode
    dd header_end - header_start

    ; checksum
    dd 0x100000000 - (0xE85250D6 + 0 + (header_end - header_start))
    ; https://intermezzos.github.io/book/first-edition/multiboot-headers.html

    ; end tag
    dw 0 ; type
    dw 0 ; flags
    dd 8 ; size
header_end:

