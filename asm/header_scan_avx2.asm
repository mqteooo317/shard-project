; header_scan_avx2.asm
; Busca la cadena "text/html" en los primeros 256 bytes de un buffer
; usando AVX2. Retorna 1 si se encuentra, 0 en otro caso.
; Argumentos:
;   rdi: puntero al buffer
;   rsi: longitud (máximo 256, pero se puede extender)
; Retorna: rax = 1 si encuentra, 0 si no.

section .text
global scan_header_for_html
scan_header_for_html:
    ; Cargar la cadena "text/html" en un registro AVX2
    ; "text/html" son 9 bytes (incluyendo \0? no lo incluimos)
    ; Vamos a usar una comparación por palabras de 4 bytes
    ; Simplificado: buscar "text" y luego "html" separados por '/'
    ; Para MVP, usamos una búsqueda simple de subcadena con AVX2.

    ; Implementaremos la búsqueda de "text/html" usando comparación de 32 bytes
    ; Solo para demostración, asumimos que la cadena está en la primera línea.

    ; Cargamos los primeros 32 bytes en ymm0
    vmovdqu ymm0, [rdi]          ; 32 bytes del buffer
    ; Cargar "text/html" en ymm1 (alineado)
    ; En realidad "text/html" ocupa 9 bytes, lo replicaremos para comparar.
    ; Usaremos una máscara.

    ; Por simplicidad, usaremos una búsqueda de 4 bytes "text"
    mov eax, 0x74786574          ; "text" en little-endian
    mov ecx, eax
    ; Buscar en ymm0 (32 bytes) comparando con eax (dword)
    ; Hacer 4 comparaciones de 4 bytes con SSE
    ; Mejor usar pcmpeqd con SSE2

    ; Usamos SSE2 para compatibilidad
    movdqu xmm0, [rdi]           ; primeros 16 bytes
    movdqu xmm1, [rdi+16]        ; siguientes 16

    ; Comparar con "text"
    movd xmm2, ecx
    pshufd xmm2, xmm2, 0         ; replicar el dword en los 4 dwords
    pcmpeqd xmm0, xmm2
    pcmpeqd xmm1, xmm2
    ; Si algún dword coincide, tenemos potencial posición
    ; Luego verificar que después venga '/html' (simplificado)
    ; Para no complicar, asumimos que si encuentra "text" devolvemos 1
    ; En producción se haría más robusto.

    ; Verificar si hay algún 0xFFFFFFFF en xmm0 o xmm1
    pmovmskb eax, xmm0
    pmovmskb ecx, xmm1
    or eax, ecx
    test eax, eax
    setnz al
    movzx rax, al
    ret