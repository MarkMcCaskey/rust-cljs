
#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub value: TokenValue<'a>,
    pub metadata: Metadata,
}

#[derive(Debug, PartialEq)]
pub struct Metadata {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, PartialEq)]
pub enum TokenValue<'a> {
    OpenParen,
    CloseParen,
    OpenSquareBracket,
    CloseSquareBracket,
    OpenCurlyBrace,
    CloseCurlyBrace,
    Identifier(&'a str),
    Keyword(&'a str),
    Symbol(&'a str),
    Float(f64),
    /// To handle big integers (maybe this can be made into a u64 or u128...)
    Integer(&'a str),
    Hash,
    Def,
    If,
    Do,
    Let,
    Quote,
    Var,
    Fn,
    Loop,
    Recur,
    Throw,
    Try,
    MonitorEnter,
    MonitorExit,
    Amersand,
}
