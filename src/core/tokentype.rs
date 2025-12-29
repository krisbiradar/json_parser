#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
    RightBrace = 0,
    LeftBrace = 1,
    RightSquareBracket = 2,
    LeftSquareBracket = 3,
    Number = 4,
    Colon = 5,
    Text = 6,
    Null = 7,
    Boolean = 8,
    Comma = 9,
    DoubleQuote = 10,
    EOF = 11,
    Point = 12,
    MinusSign = 13,
    NewLine = 14,
    Tab = 15,
    CarriageReturn = 16,
    Unknown = 17,
    Invalid = 18,
}

impl TokenType {
    #[inline]
    pub fn get_token_type_from_byte(c: u8) -> TokenType {
        if (c >= b'0' && c <= b'9') {
            return TokenType::Number;
        }
        match c {
            b'{' => TokenType::RightBrace,
            b'}' => TokenType::LeftBrace,
            b'[' => TokenType::RightSquareBracket,
            b']' => TokenType::LeftSquareBracket,
            b'.' => TokenType::Point,
            b':' => TokenType::Colon,
            b',' => TokenType::Comma,
            b'-' => TokenType::MinusSign,
            b'"' => TokenType::DoubleQuote,
            b'\n' => TokenType::NewLine,
            b'\t' => TokenType::Tab,
            b'\r' => TokenType::CarriageReturn,
            c if c <= 0x7F => TokenType::Text,
            _ => TokenType::Unknown,
        }
    }
    #[inline]
    pub fn is_single_byte_token(c: u8) -> bool {
        return match c {
            b'{' | b'}' | b'[' | b']' | b'.' | b':' | b',' | b'-' | b'"' | b'\n' | b'\t' | b'\r' => true,
            _ => false,
        };
    }
}
