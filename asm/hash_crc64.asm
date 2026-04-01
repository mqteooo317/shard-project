; hash_crc64.asm
; Calcula CRC-64 (ECMA-182) usando instrucción crc32.
; Argumentos: 
;   rdi: puntero al buffer
;   rsi: longitud (bytes)
;   rdx: semilla inicial (usualmente 0)
; Retorna: CRC-64 en rax

section .text
global crc64_ecma
crc64_ecma:
    xor rax, rax        ; resultado = 0
    mov rax, rdx        ; cargar semilla

    ; Alinear a 8 bytes (opcional, para simplificar procesamos por bytes)
    ; En este ejemplo procesamos byte a byte por simplicidad
    ; (pero se puede optimizar con QWORD)
    test rsi, rsi
    jz .done

    mov rcx, rsi        ; contador
    mov rsi, rdi        ; puntero

.loop:
    movzx r8, byte [rsi]
    crc32 rax, r8       ; actualizar CRC
    inc rsi
    loop .loop

.done:
    ret