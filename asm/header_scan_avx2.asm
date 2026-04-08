section .text
global scan_header_for_html
scan_header_for_html:

    vmovdqu ymm0, [rdi]

    mov eax, 0x74786574
    mov ecx, eax
    
    movdqu xmm0, [rdi]
    movdqu xmm1, [rdi+16]

    movd xmm2, ecx
    pshufd xmm2, xmm2, 0
    pcmpeqd xmm0, xmm2
    pcmpeqd xmm1, xmm2
    
    pmovmskb eax, xmm0
    pmovmskb ecx, xmm1
    or eax, ecx
    test eax, eax
    setnz al
    movzx rax, al
    ret
