#[cfg(target_arch = "x86_64")]
extern "C" {
    fn scan_header_for_html(ptr: *const u8, len: usize) -> u8;
}

#[cfg(not(target_arch = "x86_64"))]
fn scan_header_for_html(ptr: *const u8, len: usize) -> u8 {
    // Simple fallback: scan manually
    let data = unsafe { std::slice::from_raw_parts(ptr, len) };
    let needle = b"text/html";
    if data.windows(needle.len()).any(|window| window == needle) {
        1
    } else {
        0
    }
}

pub fn is_html_header(headers: &[u8]) -> bool {
    // Limitamos a primeros 256 bytes para evitar escaneo completo
    let len = headers.len().min(256);
    unsafe { scan_header_for_html(headers.as_ptr(), len) == 1 }
}