use encoding::{all::ISO_8859_1, DecoderTrap, Encoding};
use rayon::prelude::*;
use std::io::{
    prelude::{Read, Seek},
    Cursor, SeekFrom,
};

use crate::errors::ReaderError;
use crate::header::{Header, HeaderMOBI, HeaderPalmDoc, HeaderPalmDocForMOBI, HeaderRecord};
use crate::lz77::decompress;

const MOBI_IDENTIFIER: &str = "BOOKMOBI";
const PALMDOC_IDENTIFIER: &str = "TEXtREAd";

pub fn parse_book(buffer: &[u8]) -> Result<String, ReaderError> {
    let mut cursor = Cursor::new(buffer);

    // First parse the Header
    let header = Header::from_cursor(&mut cursor)?;

    // Then parse record headers
    let record_headers = HeaderRecord::parse_all(header.number_of_records, &mut cursor)?;

    // Then parse PalmDocHeader inside the Record 0
    let identifier_header_offset = record_headers[0].record_data_offset;

    match &(header._type + &header.creator)[..] {
        MOBI_IDENTIFIER => {
            let palmdoc_header =
                HeaderPalmDocForMOBI::from_cursor(identifier_header_offset, &mut cursor)?;
            let mobi_header = HeaderMOBI::from_cursor(&mut cursor)?;
            if mobi_header.has_exth_header() {
                Err(ReaderError::NotImplemented)
            } else {
                extract_data_from_palmdoc(
                    &mut cursor,
                    &record_headers,
                    1,
                    mobi_header.first_non_book_index as usize - 1,
                    palmdoc_header.compression,
                    4,
                )
            }
        }
        PALMDOC_IDENTIFIER => {
            let palmdoc_header = HeaderPalmDoc::from_cursor(identifier_header_offset, &mut cursor)?;
            extract_data_from_palmdoc(
                &mut cursor,
                &record_headers,
                1,
                record_headers.len() - 2,
                palmdoc_header.compression,
                0,
            )
        }
        // others not handled
        _ => Err(ReaderError::NotImplemented),
    }
}

fn extract_data_from_palmdoc(
    cursor: &mut Cursor<&[u8]>,
    record_headers: &[HeaderRecord],
    from: usize,
    to: usize,
    compression: u16,
    extra: usize,
) -> Result<String, ReaderError> {
    let first_data_record_pos = record_headers[from].record_data_offset as usize;
    let _ = cursor.seek(SeekFrom::Start(first_data_record_pos as u64))?;

    let last_data_record_pos = record_headers[to].record_data_offset as usize;

    let mut compressed_data = vec![0; last_data_record_pos - first_data_record_pos];
    cursor.read_exact(&mut compressed_data)?;
    // PalmDoc custom compression
    if compression == 2 {
        (from..to)
            .into_par_iter()
            .map(|index| {
                let start =
                    record_headers[index].record_data_offset as usize - first_data_record_pos;
                let end = record_headers[index + 1].record_data_offset as usize
                    - first_data_record_pos
                    - extra;
                let lz77_decompressed = decompress(&compressed_data[start..end])?;

                ISO_8859_1
                    .decode(&lz77_decompressed, DecoderTrap::Strict)
                    .map_err(|err| ReaderError::DecodingError(err.to_string()))
            })
            .collect()
    } else {
        Err(ReaderError::NotImplemented)
    }
}

mod tests {
    #[test]
    fn read_example_mobi() {
        let file_path: &str = "./data/ex.mobi";

        let buffer = std::fs::read(file_path).expect("Couldn't read file");

        let _ = super::parse_book(&buffer).expect("Error");
    }
}
