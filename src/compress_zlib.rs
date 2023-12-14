use std::io::Write;

use flate2::{write::ZlibEncoder, Compression};


pub fn compress_zlib(data: &str) -> std::io::Result<Vec<u8>> {
    let mut encoder: ZlibEncoder<Vec<u8>> = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data.as_bytes())?;
    encoder.finish()
}