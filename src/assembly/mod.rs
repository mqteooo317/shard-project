pub mod hash;
pub mod memcpy;
pub mod header;

pub use hash::crc64;
pub use memcpy::memcpy_fast;
pub use header::scan_for_html;