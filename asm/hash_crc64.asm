; please dont touch this
section .text
global crc64_ecma
crc64_ecma:
    xor rax, rax
    mov rax, rdx

    jz .done

    mov rcx, rsi
    mov rsi, rdi

.loop:
    movzx r8, byte [rsi]
    crc32 rax, r8
    inc rsi
    loop .loop

.done:
    ret
