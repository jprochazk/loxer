use crate::{
    expr::{BinaryExpr, Expr, GroupingExpr, Literal, UnaryExpr},
    lox_error::LoxError,
    token::{Token, TokenType},
};

/// Recursive decent parser
pub struct Parser<'a> {
    tokens: std::iter::Peekable<std::slice::Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens: tokens.iter().peekable(),
        }
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.comparison()?;

        while let Some(t) = self.tokens.next_if(|t| {
            t.token_type == TokenType::BangEqual || t.token_type == TokenType::EqualEqual
        }) {
            let operator = t;
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(BinaryExpr::new(expr, operator.to_owned(), right)));
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.term()?;

        while let Some(t) = self.tokens.next_if(|t| {
            t.token_type == TokenType::Greater
                || t.token_type == TokenType::GreaterEqual
                || t.token_type == TokenType::Less
                || t.token_type == TokenType::LessEqual
        }) {
            let operator = t;
            let right = self.term()?;
            expr = Expr::Binary(Box::new(BinaryExpr::new(expr, operator.to_owned(), right)));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.factor()?;

        while let Some(t) = self
            .tokens
            .next_if(|t| t.token_type == TokenType::Minus || t.token_type == TokenType::Plus)
        {
            let operator = t;
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(BinaryExpr::new(expr, operator.to_owned(), right)));
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.unary()?;

        while let Some(t) = self
            .tokens
            .next_if(|t| t.token_type == TokenType::Slash || t.token_type == TokenType::Star)
        {
            let operator = t;
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(BinaryExpr::new(expr, operator.to_owned(), right)));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxError> {
        if let Some(t) = self
            .tokens
            .next_if(|t| t.token_type == TokenType::Bang || t.token_type == TokenType::Minus)
        {
            let operator = t;
            let right = self.unary()?;
            let e = Expr::Unary(Box::new(UnaryExpr::new(operator.clone(), right)));
            return Ok(e);
        }
        self.primary()
    }

    // TODO: Error propagation and handle panics.
    fn primary(&mut self) -> Result<Expr, LoxError> {
        if let Some(t) = self.tokens.next() {
            match &t.token_type {
                TokenType::False => Ok(Expr::Literal(Literal::Boolean(false))),
                TokenType::True => Ok(Expr::Literal(Literal::Boolean(true))),
                TokenType::Nil => Ok(Expr::Literal(Literal::Nil)),
                TokenType::String(s) => Ok(Expr::Literal(Literal::String(s.to_string()))),
                TokenType::Number(n) => Ok(Expr::Literal(Literal::Number(*n))),
                TokenType::LeftParen => {
                    let expr = self.expression()?;
                    if let Some(t) = self.tokens.peek() {
                        if t.token_type == TokenType::RightParen {
                            self.tokens.next();
                        } else if t.token_type == TokenType::Eof {
                            return Err(LoxError::new(
                                t.line,
                                "at end Expect ')' after expression",
                            ));
                        } else {
                            return Err(LoxError::new(
                                t.line,
                                &format!("at {}. Expect ')' after expression", t.lexeme),
                            ));
                        }
                    } // TODO: Else?
                    Ok(Expr::Grouping(Box::new(GroupingExpr::new(expr))))
                }
                _ => match self.tokens.peek() {
                    Some(t) => Err(LoxError::new(t.line, "expected expression")),
                    None => Err(LoxError::new(t.line, "EOF, something unterminated")), // TODO: Better error msg
                },
            }
        } else {
            Err(LoxError::new(0, "ran out of tokens lol"))
        }
    }

    #[allow(dead_code)]
    fn sync(&mut self) {
        while let Some(t) = self.tokens.next() {
            if t.token_type == TokenType::Semicolon {
                return;
            };

            match self.tokens.peek() {
                Some(t) => match t.token_type {
                    TokenType::Class
                    | TokenType::Fun
                    | TokenType::Var
                    | TokenType::For
                    | TokenType::If
                    | TokenType::While
                    | TokenType::Print
                    | TokenType::Return => return,
                    _ => {}
                },
                None => panic!("Was looking for semicolon... ran out of tokens"),
            }
        }
    }

    pub fn parse(&mut self) -> Result<Expr, LoxError> {
        self.expression()
    }
}

#[cfg(test)]
#[test]
fn test_parser() {
    let mut scanner = crate::scanner::Scanner::new(r#"(!"hello" -3 + true) != "hi""#);
    let tokens = scanner.scan_tokens().to_vec();
    let mut parser = Parser::new(&tokens);
    let expression = parser.parse();
    assert!(expression.is_ok());
    if let Ok(e) = expression {
        assert_eq!(
            e.to_string(),
            r#"(!= (group (+ (- (! "hello") 3) true)) "hi")"#
        )
    }
}

#[test]
fn test_precedence() {
    let mut scanner = crate::scanner::Scanner::new(r#"1+2*4-5"#);
    let tokens = scanner.scan_tokens().to_vec();
    let mut parser = Parser::new(&tokens);
    let expression = parser.parse();
    assert!(expression.is_ok());
    if let Ok(e) = expression {
        assert_eq!(e.to_string(), r#"(- (+ 1 (* 2 4)) 5)"#)
    }
}
