#[link(name = "example")]
#[link(name = "tier1")]
extern "C" {
    /// This function generate a checksum with a buffer
    #[link_name = "_Z8checksumPKvi"]
    pub fn crc32_process_single_buffer(buffer: *const u8, length: i32) -> u32;
}

fn main() {
    unsafe {
        let buffer = std::ptr::null();
        let len = 0;
        crc32_process_single_buffer(buffer, len);
    }
}
