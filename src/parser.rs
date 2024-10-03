use std::collections::btree_map::OccupiedEntry;

use crate::token_type::*;
use crate::token::*;
use crate::LoxError;
use crate::expr::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser{ tokens, current : 0}
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, LoxError>{
        let mut expr = self.comparison()?;

        while self.is_match(&[TokenType::BangEqual, TokenType::Equals]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Ok(Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            }));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.term()?;

        while self.is_match(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]){
            let operator = self.previous();
            let right = self.term()?;

            expr = Ok(Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            }));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.factor()?;

        while self.is_match(&[TokenType::Minus, TokenType::Plus]){
            let operator = self.previous();
            let right = self.factor()?;

            expr = Ok(Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            }));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.unary()?;

        while self.is_match(&[TokenType::Slash, TokenType::Star]){
            let operator = self.previous();
            let right = self.unary()?;

            expr = Ok(Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            }));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxError>{
        if self.is_match(&[TokenType::Bang, TokenType::Minus]){
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary(
                UnaryExpr {operator, right: Box::new(right)}
            ));
        }

        Ok(self.primary()?)
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        if self.is_match(&[TokenType::False]) {
            return Ok(Expr::Literal(LiteralExpr {value: Some(Object::False)}));
        }

        if self.is_match(&[TokenType::True]) {
            return Ok(Expr::Literal(LiteralExpr {value: Some(Object::True)}));
        }

        if self.is_match(&[TokenType::Nil]) {
            return Ok(Expr::Literal(LiteralExpr {value: Some(Object::Nil)}));
        }

        if self.is_match(&[TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(LiteralExpr {value: Some(self.previous().literal)}));
        }

        if self.is_match(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression".to_string());
            return Ok(Expr::Grouping(GroupingExpr{expression: Box::new(expr)}));
        }

        Err(LoxError::error(0, "failed primary parser".to_string()))

    }

    fn consume(&mut self, ttype: TokenType, message: String) -> Result<Token, LoxError> {
        if self.check(ttype) {
            Ok(self.advance())
        } else {
            let p = self.peek();
            Err(LoxError::error(p.line, message))
        }
    }

    fn is_match(&mut self, types: &[TokenType]) -> bool{
         for t in &types {
            if self.check(t) {
                self.advance();
                return true;
            }
         }
         false
    }

    fn check(&self, ttype: TokenType) -> bool {
        if self.is_at_end(){
            false
        } else {
            self.peek().ttype == ttype
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current +=1 ;
        }

        self.previous();
    }

    fn is_at_end(&self) -> bool {
        self.peek() == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.current)
    }

    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1)
    }

}