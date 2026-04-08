section .text
global memcpy_fast
memcpy_fast:
    mov rcx, rdx
    rep movsb
    mov rax, rdi
    ret
