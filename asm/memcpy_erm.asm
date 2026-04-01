; memcpy_erms.asm
; Copia memoria usando REP MOVSB, que en CPUs modernas es óptimo
; Argumentos:
;   rdi: destino
;   rsi: origen
;   rdx: tamaño
; Retorna: rdi (destino) por convención

section .text
global memcpy_fast
memcpy_fast:
    ; Usar rep movsb
    mov rcx, rdx        ; tamaño
    rep movsb
    mov rax, rdi        ; retornar destino (original)
    ret