use crate::core::tokentype::TokenType;
pub struct TokenTypeRelationShips;

impl TokenTypeRelationShips {
    const TOKENTYPE_RELATIONSHIPS: [&[TokenType]; 19] = [
        // 0: RightBrace (End of Object) -> Comma, RightBrace, RightSquareBracket, EOF
        &[
            TokenType::Comma,
            TokenType::RightBrace,
            TokenType::RightSquareBracket,
            TokenType::EOF,
        ],
        // 1: LeftBrace (Start of Object) -> DoubleQuote (Key), RightBrace (Empty)
        &[TokenType::DoubleQuote, TokenType::RightBrace],
        // 2: RightSquareBracket (End of Array) -> Comma, RightBrace, RightSquareBracket, EOF
        &[
            TokenType::Comma,
            TokenType::RightBrace,
            TokenType::RightSquareBracket,
            TokenType::EOF,
        ],
        // 3: LeftSquareBracket (Start of Array) -> Value start, RightSquareBracket (Empty)
        &[
            TokenType::LeftBrace,
            TokenType::LeftSquareBracket,
            TokenType::DoubleQuote,
            TokenType::Number,
            TokenType::Boolean,
            TokenType::Null,
            TokenType::RightSquareBracket,
        ],
        // 4: Number -> Comma, RightBrace, RightSquareBracket
        &[
            TokenType::Comma,
            TokenType::RightBrace,
            TokenType::RightSquareBracket,
        ],
        // 5: Colon -> Value start
        &[
            TokenType::LeftBrace,
            TokenType::LeftSquareBracket,
            TokenType::DoubleQuote,
            TokenType::Number,
            TokenType::Boolean,
            TokenType::Null,
        ],
        // 6: Text -> Invalid usually, but if parsed as Bool/Null, see below.
        &[],
        // 7: Null -> Comma, RightBrace, RightSquareBracket
        &[
            TokenType::Comma,
            TokenType::RightBrace,
            TokenType::RightSquareBracket,
        ],
        // 8: Boolean -> Comma, RightBrace, RightSquareBracket
        &[
            TokenType::Comma,
            TokenType::RightBrace,
            TokenType::RightSquareBracket,
        ],
        // 9: Comma -> Key (in Object) or Value (in Array)
        &[
            TokenType::DoubleQuote,
            TokenType::LeftBrace,
            TokenType::LeftSquareBracket,
            TokenType::Number,
            TokenType::Boolean,
            TokenType::Null,
        ],
        // 10: DoubleQuote -> Colon (if Key), Comma/End (if Value)
        &[
            TokenType::Colon,
            TokenType::Comma,
            TokenType::RightBrace,
            TokenType::RightSquareBracket,
        ],
        // 11: EOF -> None
        &[],
        // 12: Point -> None (Internal)
        &[],
        // 13: MinusSign -> None (Internal)
        &[
            TokenType::Number,
            TokenType::Text,
        ],
        // 14: NewLine -> None (Whitespace)
        &[],
        // 15: Tab -> None (Whitespace)
        &[],
        // 16: CarriageReturn -> None (Whitespace)
        &[],
        // 17: Unknown -> None
        &[],
        // 18: Invalid -> None
        &[],
    ];

    pub fn get_allowed_next_tokens(token_type: TokenType) -> &'static [TokenType] {
        let idx = token_type as usize;
        if idx < Self::TOKENTYPE_RELATIONSHIPS.len() {
            Self::TOKENTYPE_RELATIONSHIPS[idx]
        } else {
            &[]
        }
    }
}
