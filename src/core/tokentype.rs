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
    Invalid = 14,
}

impl TokenType {
    #[inline]
    pub fn get_token_type(str: &'static str) -> TokenType {
        let string = String::from(str);
        let mut chars = string.chars();
        let first = chars.next();

        if let (Some(c), None) = (first, chars.next()) {
            return Self::get_token_type_from_char(c);
        }
        match str {
            "null" => TokenType::Null,
            "true" | "false" => TokenType::Boolean,
            _ => TokenType::Invalid,
        }
    }

    #[inline]
    const fn get_token_type_from_char(c: char) -> TokenType {
        let u8char = c as u8;
        if (u8char >= b'0' && u8char <= b'9') {
            return TokenType::Number;
        }
        match c {
            '{' => TokenType::RightBrace,
            '}' => TokenType::LeftBrace,
            '[' => TokenType::RightSquareBracket,
            ']' => TokenType::LeftSquareBracket,
            '.' => TokenType::Point,
            ':' => TokenType::Colon,
            ',' => TokenType::Comma,
            '-' => TokenType::MinusSign,
            '"' => TokenType::DoubleQuote,
            _ => TokenType::Text,
        }
    }
}
