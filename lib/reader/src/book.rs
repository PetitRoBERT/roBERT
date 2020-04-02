use encoding::{all::ISO_8859_1, DecoderTrap, Encoding};
use std::fs::File;
use std::io::{
    prelude::{Read, Seek, Write},
    BufReader, SeekFrom,
};

use crate::header::{Header, HeaderMOBI, HeaderPalmDoc, HeaderPalmDocForMOBI, HeaderRecord};
use crate::lz77::decompress;

const MOBI_IDENTIFIER: &str = "BOOKMOBI";
const PALMDOC_IDENTIFIER: &str = "TEXtREAd";

pub fn parse_book(file_path: &str) {
    let f = File::open(file_path).unwrap();
    let mut buffer: BufReader<std::fs::File> = BufReader::new(f);

    // First parse the Header
    let header = Header::from_buffer(&mut buffer);

    // Then parse record headers
    let record_headers = HeaderRecord::parse_all(header.number_of_records, &mut buffer);

    // Then parse PalmDocHeader inside the Record 0
    let identifier_header_offset = record_headers[0].record_data_offset;

    match &(header._type + &header.creator)[..] {
        MOBI_IDENTIFIER => {
            let palmdoc_header =
                HeaderPalmDocForMOBI::from_buffer(identifier_header_offset, &mut buffer);

            let mobi_header = HeaderMOBI::from_buffer(&mut buffer);

            let exth_header: Option<usize> = if mobi_header.has_exth_header() {
                // parse EXTH header
                Some(1)
            } else {
                None
            };
        }
        PALMDOC_IDENTIFIER => {
            let palmdoc_header = HeaderPalmDoc::from_buffer(identifier_header_offset, &mut buffer);

            let record_1 = record_headers[1].clone();
            let record_2 = record_headers[record_headers.len() - 2].clone();
            let size = record_2.record_data_offset - record_1.record_data_offset;
            let _ = buffer.seek(SeekFrom::Start(record_1.record_data_offset as u64));

            // PalmDoc custom compression
            if palmdoc_header.compression == 2 {
                let mut data_to_decompress = vec![0; size as usize];
                let _ = buffer
                    .read(&mut data_to_decompress)
                    .expect("Error while reading data");

                let lz77_decompressed = decompress(&data_to_decompress).unwrap();
                let text: String = ISO_8859_1
                    .decode(&lz77_decompressed, DecoderTrap::Strict)
                    .expect("Error while decoding the decompressed text");
                let mut file = File::create("foo.txt").unwrap();
                file.write_all(text.as_bytes()).unwrap();
            }
        }
        // others not handled
        _ => {}
    }
}
