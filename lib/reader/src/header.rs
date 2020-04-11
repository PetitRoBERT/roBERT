use std::io::prelude::{Read, Seek};
use std::io::{Cursor, IoSliceMut, SeekFrom};

use crate::errors::ReaderError;

// From mobi-python:
//
// HEADER is '>32shhIIIIII4s4sIIH'
// Meaning:
//
// >  : Big-Endian
// Xs : X characters
// h  : 2B - short
// I  : 4B - Unsigned Int
// H  : 2B - Unsigned short
#[derive(Debug)]
pub struct Header {
    pub title: String,            // 32 bytes: Title of the book
    pub attributes: i16,          // h: 2 bytes
    pub version: i16,             // h: 2 bytes
    pub created: u32,             // I: 4 bytes - unsigned int
    pub modified: u32,            // I: 4 bytes - unsigned int
    pub backup: u32,              // I: 4 bytes - unsigned int
    pub modnum: u32,              // I: 4 bytes - unsigned int
    pub app_info_id: u32,         // I: 4 bytes - unsigned int
    pub sort_info_id: u32,        // I: 4 bytes - unsigned int
    pub _type: String,            // 4s
    pub creator: String,          // 4s
    pub unique_id_seed: u32,      // I: 4 bytes - unsigned int
    pub next_record_list_id: u32, // I: 4 bytes - unsigned int
    pub number_of_records: u16,   // H: 2 bytes - unsigned short
}

impl Header {
    pub fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Header, ReaderError> {
        let mut title = [0; 32];
        let mut attributes = [0; 2];
        let mut version = [0; 2];
        let mut created = [0; 4];
        let mut modified = [0; 4];
        let mut backup = [0; 4];
        let mut modnum = [0; 4];
        let mut app_info_id = [0; 4];
        let mut sort_info_id = [0; 4];
        let mut _type = [0; 4];
        let mut creator = [0; 4];
        let mut unique_id_seed = [0; 4];
        let mut next_record_list_id = [0; 4];
        let mut number_of_records = [0; 2];

        let bufs = &mut [
            IoSliceMut::new(&mut title),
            IoSliceMut::new(&mut attributes),
            IoSliceMut::new(&mut version),
            IoSliceMut::new(&mut created),
            IoSliceMut::new(&mut modified),
            IoSliceMut::new(&mut backup),
            IoSliceMut::new(&mut modnum),
            IoSliceMut::new(&mut app_info_id),
            IoSliceMut::new(&mut sort_info_id),
            IoSliceMut::new(&mut _type),
            IoSliceMut::new(&mut creator),
            IoSliceMut::new(&mut unique_id_seed),
            IoSliceMut::new(&mut next_record_list_id),
            IoSliceMut::new(&mut number_of_records),
        ];

        let _ = cursor.read_vectored(bufs)?;
        Ok(Header {
            title: String::from_utf8_lossy(&title)
                .to_owned()
                .trim_matches(char::from(0))
                .to_string(),
            attributes: i16::from_be_bytes(attributes),
            version: i16::from_be_bytes(version),
            created: u32::from_be_bytes(created),
            modified: u32::from_be_bytes(modified),
            backup: u32::from_be_bytes(backup),
            modnum: u32::from_be_bytes(modnum),
            app_info_id: u32::from_be_bytes(app_info_id),
            sort_info_id: u32::from_be_bytes(sort_info_id),
            _type: String::from_utf8_lossy(&_type).to_owned().to_string(),
            creator: String::from_utf8_lossy(&creator).to_owned().to_string(),
            unique_id_seed: u32::from_be_bytes(unique_id_seed),
            next_record_list_id: u32::from_be_bytes(next_record_list_id),
            number_of_records: u16::from_be_bytes(number_of_records),
        })
    }
}

// From Mobi-Python
//
// HEADER: >II
#[derive(Debug, Clone)]
pub struct HeaderRecord {
    pub record_data_offset: u32,
    unique_id: u32,
}

impl HeaderRecord {
    fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<HeaderRecord, ReaderError> {
        let mut record_data_offset = [0; 4];
        let mut unique_id = [0; 4];

        let bufs = &mut [
            IoSliceMut::new(&mut record_data_offset),
            IoSliceMut::new(&mut unique_id),
        ];

        let _ = cursor.read_vectored(bufs)?;
        Ok(HeaderRecord {
            record_data_offset: u32::from_be_bytes(record_data_offset),
            unique_id: u32::from_be_bytes(unique_id),
        })
    }

    pub fn parse_all(
        nb_of_records: u16,
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<Vec<HeaderRecord>, ReaderError> {
        (0..nb_of_records)
            .map(|_| Self::from_cursor(cursor))
            .collect()
    }
}

// From Mobi-Python
//
// HEADER: >HHIHHHH
#[derive(Debug)]
pub struct HeaderPalmDocForMOBI {
    pub compression: u16,
    blank_1: u16,
    text_length: u32,
    record_count: u16,
    record_size: u16,
    encryption_type: u16,
    blank_2: u16,
}

impl HeaderPalmDocForMOBI {
    pub fn from_cursor(
        offset: u32,
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<HeaderPalmDocForMOBI, ReaderError> {
        let mut compression = [0; 2];
        let mut blank_1 = [0; 2];
        let mut text_length = [0; 4];
        let mut record_count = [0; 2];
        let mut record_size = [0; 2];
        let mut encryption_type = [0; 2];
        let mut blank_2 = [0; 2];

        let bufs = &mut [
            IoSliceMut::new(&mut compression),
            IoSliceMut::new(&mut blank_1),
            IoSliceMut::new(&mut text_length),
            IoSliceMut::new(&mut record_count),
            IoSliceMut::new(&mut record_size),
            IoSliceMut::new(&mut encryption_type),
            IoSliceMut::new(&mut blank_2),
        ];

        let _ = cursor.seek(SeekFrom::Start(u64::from(offset)))?;

        let _ = cursor.read_vectored(bufs)?;
        Ok(HeaderPalmDocForMOBI {
            compression: u16::from_be_bytes(compression),
            blank_1: u16::from_be_bytes(blank_1),
            text_length: u32::from_be_bytes(text_length),
            record_count: u16::from_be_bytes(record_count),
            record_size: u16::from_be_bytes(record_size),
            encryption_type: u16::from_be_bytes(encryption_type),
            blank_2: u16::from_be_bytes(blank_2),
        })
    }
}

#[derive(Debug)]
pub struct HeaderPalmDoc {
    pub compression: u16,
    blank_1: u16,
    pub text_length: u32,
    pub record_count: u16,
    pub record_size: u16,
    pub text_offset: u32,
}

impl HeaderPalmDoc {
    pub fn from_cursor(
        offset: u32,
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<HeaderPalmDoc, ReaderError> {
        let mut compression = [0; 2];
        let mut blank_1 = [0; 2];
        let mut text_length = [0; 4];
        let mut record_count = [0; 2];
        let mut record_size = [0; 2];
        let mut text_offset = [0; 4];

        let bufs = &mut [
            IoSliceMut::new(&mut compression),
            IoSliceMut::new(&mut blank_1),
            IoSliceMut::new(&mut text_length),
            IoSliceMut::new(&mut record_count),
            IoSliceMut::new(&mut record_size),
            IoSliceMut::new(&mut text_offset),
        ];

        let _ = cursor.seek(SeekFrom::Start(offset as u64))?;

        let _ = cursor.read_vectored(bufs)?;
        Ok(HeaderPalmDoc {
            compression: u16::from_be_bytes(compression),
            blank_1: u16::from_be_bytes(blank_1),
            text_length: u32::from_be_bytes(text_length),
            record_count: u16::from_be_bytes(record_count),
            record_size: u16::from_be_bytes(record_size),
            text_offset: u32::from_be_bytes(text_offset),
        })
    }
}

// From Mobi-Python
//
// HEADER:
// '> IIII II 40s III IIIII IIII I 36s IIII 8s HHIIIII'
#[allow(dead_code)]
pub struct HeaderMOBI {
    pub identifier: String,
    header_length: u32,
    mobi_type: u32,
    text_encoding: u32,

    unique_id: u32,

    reserved: [u8; 44],

    pub first_non_book_index: u32,
    pub full_name_offset: u32,
    pub full_name_length: u32,

    language: u32,
    input_language: u32,
    output_language: u32,
    format_version: u32,

    first_image_index: u32,

    first_huff_record: u32,
    huff_record_count: u32,
    first_datp_record: u32,
    datp_record_count: u32,

    exth_flags: u32,

    blank_1: [u8; 36],

    drm_offset: u32,
    drm_count: u32,
    drm_size: u32,
    drm_flags: u32,

    blank_2: [u8; 8],

    blank_3: [u8; 2],
    last_image_record: u16,
    blank_4: [u8; 4],
    fcis_record: u32,
    blank_5: [u8; 4],
    flis_record: u32,
    blank_6: [u8; 2],
    useful_blank_6: [u8; 2],

    pub extra_bytes: u32,
}

#[allow(dead_code)]
impl HeaderMOBI {
    pub fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<HeaderMOBI, ReaderError> {
        let mut identifier = [0; 4];
        let mut header_length = [0; 4];
        let mut mobi_type = [0; 4];
        let mut text_encoding = [0; 4];

        let mut unique_id = [0; 4];

        let mut reserved = [0; 44];

        let mut first_non_book_index = [0; 4];
        let mut full_name_offset = [0; 4];
        let mut full_name_length = [0; 4];

        let mut language = [0; 4];
        let mut input_language = [0; 4];
        let mut output_language = [0; 4];
        let mut format_version = [0; 4];

        let mut first_image_index = [0; 4];

        let mut first_huff_record = [0; 4];
        let mut huff_record_count = [0; 4];
        let mut first_datp_record = [0; 4];
        let mut datp_record_count = [0; 4];

        let mut exth_flags = [0; 4];

        let mut blank_1 = [0; 36];

        let mut drm_offset = [0; 4];
        let mut drm_count = [0; 4];
        let mut drm_size = [0; 4];
        let mut drm_flags = [0; 4];

        let mut blank_2 = [0; 8];

        let mut blank_3 = [0; 2];
        let mut last_image_record = [0; 2];
        let mut blank_4 = [0; 4];
        let mut fcis_record = [0; 4];
        let mut blank_5 = [0; 4];
        let mut flis_record = [0; 4];
        let mut blank_6 = [0; 2];
        let mut useful_blank_6 = [0; 2];

        let bufs = &mut [
            IoSliceMut::new(&mut identifier),
            IoSliceMut::new(&mut header_length),
            IoSliceMut::new(&mut mobi_type),
            IoSliceMut::new(&mut text_encoding),
            IoSliceMut::new(&mut unique_id),
            IoSliceMut::new(&mut reserved),
            IoSliceMut::new(&mut first_non_book_index),
            IoSliceMut::new(&mut full_name_offset),
            IoSliceMut::new(&mut full_name_length),
            IoSliceMut::new(&mut language),
            IoSliceMut::new(&mut input_language),
            IoSliceMut::new(&mut output_language),
            IoSliceMut::new(&mut format_version),
            IoSliceMut::new(&mut first_image_index),
            IoSliceMut::new(&mut first_huff_record),
            IoSliceMut::new(&mut huff_record_count),
            IoSliceMut::new(&mut first_datp_record),
            IoSliceMut::new(&mut datp_record_count),
            IoSliceMut::new(&mut exth_flags),
            IoSliceMut::new(&mut blank_1),
            IoSliceMut::new(&mut drm_offset),
            IoSliceMut::new(&mut drm_count),
            IoSliceMut::new(&mut drm_size),
            IoSliceMut::new(&mut drm_flags),
            IoSliceMut::new(&mut blank_2),
            IoSliceMut::new(&mut blank_3),
            IoSliceMut::new(&mut last_image_record),
            IoSliceMut::new(&mut blank_4),
            IoSliceMut::new(&mut fcis_record),
            IoSliceMut::new(&mut blank_5),
            IoSliceMut::new(&mut flis_record),
            IoSliceMut::new(&mut blank_6),
            IoSliceMut::new(&mut useful_blank_6),
        ];

        let _ = cursor.read_vectored(bufs)?;

        let useful_blank_6_1 = u16::from_be_bytes(useful_blank_6);

        Ok(HeaderMOBI {
            identifier: String::from_utf8_lossy(&identifier)
                .to_owned()
                .trim_matches(char::from(0))
                .to_string(),
            header_length: u32::from_be_bytes(header_length),
            mobi_type: u32::from_be_bytes(mobi_type),
            text_encoding: u32::from_be_bytes(text_encoding),

            unique_id: u32::from_be_bytes(unique_id),

            reserved,

            first_non_book_index: u32::from_be_bytes(first_non_book_index),
            full_name_offset: u32::from_be_bytes(full_name_offset),
            full_name_length: u32::from_be_bytes(full_name_length),

            language: u32::from_be_bytes(language),
            input_language: u32::from_be_bytes(input_language),
            output_language: u32::from_be_bytes(output_language),
            format_version: u32::from_be_bytes(format_version),

            first_image_index: u32::from_be_bytes(first_image_index),

            first_huff_record: u32::from_be_bytes(first_huff_record),
            huff_record_count: u32::from_be_bytes(huff_record_count),
            first_datp_record: u32::from_be_bytes(first_datp_record),
            datp_record_count: u32::from_be_bytes(datp_record_count),

            exth_flags: u32::from_be_bytes(exth_flags),

            blank_1,

            drm_offset: u32::from_be_bytes(drm_offset),
            drm_count: u32::from_be_bytes(drm_count),
            drm_size: u32::from_be_bytes(drm_size),
            drm_flags: u32::from_be_bytes(drm_flags),

            blank_2,

            blank_3,
            last_image_record: u16::from_be_bytes(last_image_record),
            blank_4,
            fcis_record: u32::from_be_bytes(fcis_record),
            blank_5,
            flis_record: u32::from_be_bytes(flis_record),
            blank_6,
            useful_blank_6,

            extra_bytes: 2 * (useful_blank_6_1 & 0xFFFE).count_ones(),
        })
    }

    pub fn has_drm(&self) -> bool {
        self.drm_offset != 0_u32
    }

    pub fn has_exth_header(&self) -> bool {
        self.exth_flags == 64
    }

    pub fn full_title(
        &self,
        offset: u32,
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<String, ReaderError> {
        let _ = cursor.seek(SeekFrom::Start(u64::from(offset + self.full_name_offset)));

        let mut buf = vec![0; self.full_name_offset as usize];
        let _ = cursor.read_exact(&mut buf);
        Ok(String::from_utf8_lossy(&buf)
            .to_owned()
            .trim_matches(char::from(0))
            .to_string())
    }
}
