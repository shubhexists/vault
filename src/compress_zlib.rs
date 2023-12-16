use flate2::read::ZlibDecoder;
use flate2::{write::ZlibEncoder, Compression};
use std::fs::File;
use std::io::{self, Read, Write};

pub fn compress_zlib(data: &str) -> std::io::Result<Vec<u8>> {
    let mut encoder: ZlibEncoder<Vec<u8>> = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data.as_bytes())?;
    encoder.finish()
}

pub fn decompress_zlib(file_path: File) -> io::Result<()> {
    let mut decoder: ZlibDecoder<File> = ZlibDecoder::new(file_path);
    let mut decompressed_data: Vec<u8> = Vec::new();
    decoder.read_to_end(&mut decompressed_data)?;
    let decompressed_string: String = String::from_utf8_lossy(&decompressed_data).into_owned();
    println!("{decompressed_string}");
    Ok(())
}
