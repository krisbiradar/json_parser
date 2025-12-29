use std::fs;
use std::path::PathBuf;

use json_parser::lexer::buffered_string_reader::BufferedStringReader;
use json_parser::lexer::byte_reader::ByteReader;

fn setup(json_file_name: String) -> BufferedStringReader {
    let cargo_manifest_dir = env!("CARGO_MANIFEST_DIR");
    let text = fs::read_to_string(
        PathBuf::from(cargo_manifest_dir)
            .join("tests")
            .join("data")
            .join(json_file_name),
    )
    .unwrap();
    let reader = BufferedStringReader::new(text.as_bytes().to_vec());
    return reader;
}

#[test]
fn text_reader_next_byte_should_return_correct_value() {
    let mut file_reader1 = setup("sample1.json".to_string());
    let actual1 = file_reader1.next_byte();
    assert_eq!(actual1.unwrap(), b'{');

    let mut file_reader2 = setup("sample2.json".to_string());
    let actual2 = file_reader2.next_byte();

    assert_eq!(actual2.unwrap(), b'[');
}

#[test]
fn text_reader_next_chunk_should_return_correct_value() {
    let mut file_reader1 = setup("sample1.json".to_string()).with_chunk_size(100);
    let actual1 = file_reader1.next_chunk();
    let expected1 = br#"{
    "flights": [
        {
            "flight_number": "DL8509",
            "airline_name": "Del"#;
    let mut file_reader2 = setup("sample2.json".to_string()).with_chunk_size(100);
    let actual2 = file_reader2.next_chunk();
    let expected2 = br#"[
        {
            "flight_number": "DL8509",
            "airline_name": "Delta Air Lines",
  "#;

    assert_eq!(actual1.unwrap(), expected1);
    assert_eq!(actual2.unwrap(), expected2);
}

#[test]
fn text_reader_next_until_should_return_correct_value() {
    let mut file_reader1 = setup("sample1.json".to_string());
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

    let mut file_reader2 = setup("sample2.json".to_string());
    let actual2 = file_reader2.next_until(b'.');
    let expected2 = br#"[
        {
            "flight_number": "DL8509",
            "airline_name": "Delta Air Lines",
            "departure_airport": "John F."#;

    assert_eq!(actual1.unwrap(), expected1);
    assert_eq!(actual2.unwrap(), expected2);
}
