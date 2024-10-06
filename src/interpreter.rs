use crate::expr::*;
use crate::object::*;
use crate::error::*;
use crate::token_type::*;

pub struct Interpreter {}

impl ExprVisitor<Object> for Interpreter{
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Object, LoxError>{
        Ok(expr.value.clone().unwrap())
    }

    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Object, LoxError>{
        Ok(Object::Nil)
    }
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Object, LoxError>{
        Ok(self.evaluate(&expr.expression)?)
    }
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<Object, LoxError>{
        let right = self.evaluate(&expr.right)?;

        match expr.operator.token_type(){
            TokenType::Minus => {
                match right {
                    Object::Num(n) => { return Ok(Object::Num(-n))},
                    _ => {return Ok(Object::Nil)}
                }
            },
            TokenType::Bang => Ok(Object::Bool(!self.is_truthy(&right))),
            _ =>  Err(LoxError::error(expr.operator.line, "Unreachable"))

        }
    }
}

impl Interpreter{
    fn evaluate(&self, expr: &Expr) -> Result<Object, LoxError>{
        expr.accept(self)
    }

    // anything that is not Nil or False is true
    fn is_truthy(&self, object:&Object) -> bool {
        !matches!(object, Object::Bool(false) | Object::Nil) }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::*;

    #[test]
    fn test_unary_minus() {
        let terp = Interpreter {};
        let unary_expr = Expr::Unary(UnaryExpr{
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 0),
            right: Box::new(Expr::Literal(LiteralExpr{
                value: Some(Object::Num(123.0))
            }))
        });

        let result = terp.evaluate(&unary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Num(-123.0)));
    }

    #[test]
    fn test_unary_not() {
        let terp = Interpreter {};
        let unary_expr = Expr::Unary(UnaryExpr{
            operator: Token::new(TokenType::Bang, "!".to_string(), None, 0),
            right: Box::new(Expr::Literal(LiteralExpr{
                value: Some(Object::Bool(false))
            }))
        });

        let result = terp.evaluate(&unary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Bool(true)));
    }
}