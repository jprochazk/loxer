use std::fmt::Display;

pub struct LoxError {
    line: u32,
    message: String,
}

impl LoxError {
    pub fn new(line: u32, message: String) -> LoxError {
        LoxError { line, message }
    }
}

impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Line {line}: {message}",
            line = self.line,
            message = self.message
        )
    }
}