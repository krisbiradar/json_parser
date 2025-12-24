use std::path::PathBuf;

use json_parser::lexer::{buffered_file_reader::BufferedFileReader, byte_reader::ByteReader};

#[test]
fn file_reader_next_byte_should_return_correct_value() {
    let mut file_reader1 = BufferedFileReader::new(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("data").join("sample1.json"));
    let first_byte1 = file_reader1.next_byte();
    assert_eq!(first_byte1.unwrap(),b'{');

    let mut file_reader2 = BufferedFileReader::new(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("data").join("sample2.json"));
    let first_byte2 = file_reader2.next_byte();
   
    assert_eq!(first_byte2.unwrap(),b'[');

}