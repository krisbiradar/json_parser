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

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::RightBrace => write!(f, "RightBrace"),
            TokenType::LeftBrace => write!(f, "LeftBrace"),
            TokenType::RightSquareBracket => write!(f, "RightSquareBracket"),
            TokenType::LeftSquareBracket => write!(f, "LeftSquareBracket"),
            TokenType::Number => write!(f, "Number"),
            TokenType::Colon => write!(f, "Colon"),
            TokenType::Text => write!(f, "Text"),
            TokenType::Null => write!(f, "Null"),
            TokenType::Boolean => write!(f, "Boolean"),
            TokenType::Comma => write!(f, "Comma"),
            TokenType::DoubleQuote => write!(f, "DoubleQuote"),
            TokenType::EOF => write!(f, "EOF"),
            TokenType::Point => write!(f, "Point"),
            TokenType::MinusSign => write!(f, "MinusSign"),
            TokenType::NewLine => write!(f, "NewLine"),
            TokenType::Tab => write!(f, "Tab"),
            TokenType::CarriageReturn => write!(f, "CarriageReturn"),
            TokenType::Unknown => write!(f, "Unknown"),
            TokenType::Invalid => write!(f, "Invalid"),
        }
    }
}

impl TokenType {
    #[inline]
    pub fn get_token_type_from_byte(c: u8) -> TokenType {
        if (c >= b'0' && c <= b'9') {
            return TokenType::Number;
        }
        match c {
            b'{' => TokenType::LeftBrace,
            b'}' => TokenType::RightBrace,
            b'[' => TokenType::LeftSquareBracket,
            b']' => TokenType::RightSquareBracket,
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
            b'{' | b'}' | b'[' | b']' | b'.' | b':' | b',' | b'-' | b'\n' | b'\t' | b'\r' => true,
            _ => false,
        };
    }
    #[inline]
    pub fn is_whitespace(c: u8) -> bool {
        matches!(c, b' ' | b'\n' | b'\t' | b'\r')
    }
}
