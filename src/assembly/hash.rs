#[cfg(target_arch = "x86_64")]
extern "C" {
    fn crc64_ecma(ptr: *const u8, len: usize, seed: u64) -> u64;
}

#[cfg(not(target_arch = "x86_64"))]
fn crc64_ecma(ptr: *const u8, len: usize, seed: u64) -> u64 {
    // Fallback software implementation (simple)
    let mut crc = seed;
    for i in 0..len {
        unsafe {
            crc = crc64_byte(crc, *ptr.add(i));
        }
    }
    crc
}

#[cfg(not(target_arch = "x86_64"))]
fn crc64_byte(crc: u64, byte: u8) -> u64 {
    // Simple CRC-64 table lookup
    const CRC64_TABLE: [u64; 256] = {
        let mut table = [0u64; 256];
        let mut i = 0;
        while i < 256 {
            let mut crc = i as u64;
            let mut j = 0;
            while j < 8 {
                if crc & 1 != 0 {
                    crc = (crc >> 1) ^ 0xC96C5795D7870F42;
                } else {
                    crc >>= 1;
                }
                j += 1;
            }
            table[i] = crc;
            i += 1;
        }
        table
    };
    (crc >> 8) ^ CRC64_TABLE[(crc as u8 ^ byte) as usize]
}

pub fn crc64(data: &[u8]) -> u64 {
    unsafe { crc64_ecma(data.as_ptr(), data.len(), 0) }
}