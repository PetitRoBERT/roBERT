use encoding::{all::ISO_8859_1, DecoderTrap, Encoding};
use rayon::prelude::*;
use std::fs::File;
use std::io::{
    prelude::{Read, Seek},
    BufReader, SeekFrom,
};

use crate::errors::ReaderError;
use crate::header::{Header, HeaderMOBI, HeaderPalmDoc, HeaderPalmDocForMOBI, HeaderRecord};
use crate::lz77::decompress;

const MOBI_IDENTIFIER: &str = "BOOKMOBI";
const PALMDOC_IDENTIFIER: &str = "TEXtREAd";

pub fn parse_book(file_path: &str) -> Result<String, ReaderError> {
    let f = File::open(file_path)?;
    let mut buffer: BufReader<std::fs::File> = BufReader::new(f);

    // First parse the Header
    let header = Header::from_buffer(&mut buffer)?;

    // Then parse record headers
    let record_headers = HeaderRecord::parse_all(header.number_of_records, &mut buffer)?;

    // Then parse PalmDocHeader inside the Record 0
    let identifier_header_offset = record_headers[0].record_data_offset;

    match &(header._type + &header.creator)[..] {
        MOBI_IDENTIFIER => {
            let palmdoc_header =
                HeaderPalmDocForMOBI::from_buffer(identifier_header_offset, &mut buffer);
            let mobi_header = HeaderMOBI::from_buffer(&mut buffer)?;

            let exth_header: Option<usize> = if mobi_header.has_exth_header() {
                // parse EXTH header
                Some(1)
            } else {
                None
            };
            Err(ReaderError::NotImplemented)
        }
        PALMDOC_IDENTIFIER => {
            let palmdoc_header = HeaderPalmDoc::from_buffer(identifier_header_offset, &mut buffer)?;

            let first_data_record_pos = record_headers[1].record_data_offset as usize;
            let _ = buffer.seek(SeekFrom::Start(first_data_record_pos as u64))?;

            let last_data_record_pos =
                record_headers[record_headers.len() - 2].record_data_offset as usize;

            let mut compressed_data = vec![0; last_data_record_pos - first_data_record_pos];
            buffer.read_exact(&mut compressed_data)?;

            // PalmDoc custom compression
            if palmdoc_header.compression == 2 {
                record_headers
                    .par_iter()
                    .enumerate()
                    .skip(1)
                    .take(record_headers.len() - 3)
                    .map(|(index, record_header)| {
                        let start =
                            record_header.record_data_offset as usize - first_data_record_pos;
                        let end = record_headers[index + 1].record_data_offset as usize
                            - first_data_record_pos;
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
        // others not handled
        _ => Err(ReaderError::NotImplemented),
    }
}
