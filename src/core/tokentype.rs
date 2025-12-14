#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
    RightBrace=0,
    LeftBrace=1,
    RightSquareBracket=2,
    LeftSquareBracket=3,
    Number=4,
    Colon=5,
    Text=6,
    Null=7,
    Boolean=8,
    Comma=9,
    DoubleQuote=10,
    EOF=11,
    Point=12
}


impl TokenType {
    #[inline]
    const fn idx(self) -> usize {
        self as usize
    }
}
