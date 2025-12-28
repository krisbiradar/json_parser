use std::path::PathBuf;

use json_parser::lexer::{buffered_file_reader::BufferedFileReader, byte_reader::ByteReader};

#[test]
fn file_reader_next_byte_should_return_correct_value() {
    let mut file_reader1 = setup("sample1.json");
    let first_byte1 = file_reader1.next_byte();
    assert_eq!(first_byte1.unwrap(), b'{');

    let mut file_reader2 = setup("sample2.json");
    let first_byte2 = file_reader2.next_byte();

    assert_eq!(first_byte2.unwrap(), b'[');
}

fn setup(file_name: &str) -> BufferedFileReader {
    let cargo_manifest_dir = env!("CARGO_MANIFEST_DIR");
    BufferedFileReader::new(
        PathBuf::from(cargo_manifest_dir)
            .join("tests")
            .join("data")
            .join(file_name),
    )
}

#[test]
fn file_reader_next_chunk_should_return_valid_chunk() {
    let mut file_reader1 = setup("sample1.json").with_chunk_size(100);
    let first_chunk = file_reader1.next_chunk();
    let first_100: &[u8; 100] = br#"{
    "flights": [
        {
            "flight_number": "DL8509",
            "airline_name": "Del"#;

    assert_eq!(first_chunk.unwrap(), first_100);
    let mut file_reader2 = setup("sample2.json").with_chunk_size(100);
    let first_chunk_2 = file_reader2.next_chunk();
    let first_100_2: &[u8; 100] = br#"[
        {
            "flight_number": "DL8509",
            "airline_name": "Delta Air Lines",
  "#;

    assert_eq!(first_chunk_2.unwrap(), first_100_2);
}

#[test]
fn file_reader_next_until_should_return_correct_values() {
    let mut file_reader1 = setup("sample1.json").with_chunk_size(100);
    let actual1 = file_reader1.next_until(b'2');
    let expected1 = br#"{
    "flights": [
        {
            "flight_number": "DL8509",
            "airline_name": "Delta Air Lines",
            "departure_airport": "John F. Kennedy International",
            "departure_city": "New York",
            "departure_country": "USA",
            "departure_time": "4:02"#;

    assert_eq!(actual1.unwrap(), expected1);
    let mut file_reader2 = setup("sample2.json").with_chunk_size(100);
    let actual2 = file_reader2.next_until(b'.');
    let expected2  = br#"[
        {
            "flight_number": "DL8509",
            "airline_name": "Delta Air Lines",
            "departure_airport": "John F."#;

    assert_eq!(actual2.unwrap(), expected2);
}
