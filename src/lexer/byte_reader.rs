pub trait ByteReader {
    fn next_byte(& mut self) ->  Result<u8,String>;
    fn next_chunk(& mut self) -> Result<Vec<u8>,String>;
    fn next_until(& mut self , byte:u8) -> Result<Vec<u8>,String>;
}
