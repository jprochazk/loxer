use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: u32, // TODO: Line numbers on tokens
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: u32) -> Token {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{token_type:?} {lexeme}",
            token_type = self.token_type,
            lexeme = self.lexeme,
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // --- Single-character tokens. ---
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // --- One or two character tokens. ---
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier(String),
    // --- Literals. ---
    // String(String),
    String,
    // Number(f64),
    Number,
    // --- Keywords. ---
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}
