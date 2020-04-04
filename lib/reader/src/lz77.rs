use crate::errors::ReaderError;

// Source: https://wiki.mobileread.com/wiki/PalmDOC
pub fn decompress(data: &[u8]) -> Result<Vec<u8>, ReaderError> {
    let length = data.len();
    let mut text = Vec::<u8>::new();
    let mut offset = 0;
    while offset < length {
        let byte = data[offset];
        offset += 1;
        match byte {
            // 0x00: "1 literal" copy that byte unmodified to the decompressed stream.
            // 0x09 to 0x7f: "1 literal" copy that byte unmodified to the decompressed stream.
            // Literal chars
            0x00 | 0x09..=0x7f => text.push(byte),
            // 0x01 to 0x08: "literals": the byte is interpreted as a count from 1 to 8,
            // and that many literals are copied
            // unmodified from the compressed stream to the decompressed stream.
            0x01..=0x08 => {
                if offset + byte as usize <= length {
                    text.extend(&data[offset..(offset + (byte as usize))]);
                    offset += byte as usize;
                }
            }
            // 0x80 to 0xbf: "length, distance" pair: the 2 leftmost bits of this byte ('10')
            // are discarded, and the following 6 bits are combined with the 8 bits of the next
            // byte to make a 14 bit "distance, length" item. Those 14 bits are broken into 11
            // bits of distance backwards from the current location in the uncompressed text,
            // and 3 bits of length to copy from that point (copying n+3 bytes, 3 to 10 bytes).
            0x80..=0xbf => {
                offset += 1;
                if offset > data.len() {
                    println!(
                        "Offset to LZ77 bits is outside of the
                        data: offset: {}, data_length: {}",
                        offset,
                        data.len()
                    );
                    return Ok(text);
                }
                let mut lz77 = u16::from_be_bytes([data[offset - 2], data[offset - 1]]);

                lz77 &= 0x3fff; // Leftmost two bits are ID bits and need to be dropped
                let lz77_length = (lz77 & 0x0007) + 3; // Length is rightmost 3 bits + 3
                let lz77_offset = lz77 >> 3; // Remaining 11 bits are offset

                if lz77_offset < 1 {
                    println!("WARNING: LZ77 decompression offset is invalid!");
                    return Ok(text);
                }

                let mut current_text_length = text.len();
                for _ in 0..lz77_length {
                    let text_pos = current_text_length as i32 - lz77_offset as i32;
                    if text_pos < 0 {
                        return Err(ReaderError::DecodingError(format!(
                            "WARNING: LZ77 decompression reference is before
                            beginning of text! {}",
                            lz77,
                        )));
                    }
                    text.push(text[text_pos as usize]);
                    current_text_length += 1;
                }
            }
            // 0xc0 to 0xff: "byte pair": this byte is decoded into 2 characters:
            // a space character, and a letter formed from this byte XORed with 0x80.
            0xc0..=0xff => text.extend(&[b' ', byte ^ 0x80]),
        }
    }
    Ok(text)
}
