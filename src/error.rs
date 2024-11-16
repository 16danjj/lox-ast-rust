use crate::{token::Token, token_type::TokenType};

#[derive(Debug)]
pub enum LoxResult {
    ParseError { token: Token, message: String },
    RuntimeError { token: Token, message: String },
    LoxError { line: usize, message: String },
    Break,
}

impl LoxResult {
    pub fn error(line: usize, message: &str) -> LoxResult {
        let err = LoxResult::LoxError {
            line,
            message: message.to_string(),
        };
        err.report("");
        err
    }

    pub fn parse_error(token: &Token, message: &str) -> LoxResult {
        let err = LoxResult::ParseError {
            token: token.dup(),
            message: message.to_string(),
        };
        err.report("");
        err
    }

    pub fn runtime_error(token: &Token, message: &str) -> LoxResult {
        let err = LoxResult::RuntimeError {
            token: token.dup(),
            message: message.to_string(),
        };
        err.report("");
        err
    }

    fn report(&self, loc: &str) {
        match self {
            LoxResult::ParseError { token, message }
            | LoxResult::RuntimeError { token, message } => {
                if token.is(TokenType::Eof) {
                    eprintln!("{} at end {}", token.line, message);
                } else {
                    eprintln!("line {} at '{}' {}", token.line, token.as_string(), message)
                }
            }
            LoxResult::LoxError { line, message } => {
                eprintln!("[line {}] Error{}: {}", line, loc, message);
            }
            LoxResult::Break => {}
        };
    }
}
