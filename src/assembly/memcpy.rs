#[cfg(target_arch = "x86_64")]
extern "C" {
    fn memcpy_fast(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;
}

#[cfg(not(target_arch = "x86_64"))]
fn memcpy_fast(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    unsafe {
        std::ptr::copy_nonoverlapping(src, dest, n);
    }
    dest
}

pub fn copy_fast(dest: &mut [u8], src: &[u8]) {
    assert!(dest.len() >= src.len());
    unsafe {
        memcpy_fast(dest.as_mut_ptr(), src.as_ptr(), src.len());
    }
}