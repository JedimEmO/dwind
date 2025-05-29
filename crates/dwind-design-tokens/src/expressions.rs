use crate::error::{TokenError, TokenResult};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, multispace0},
    combinator::{map, recognize},
    multi::many0,
    number::complete::double,
    sequence::{delimited, pair, preceded, terminated},
    IResult,
};
use serde::{Deserialize, Serialize};

/// Binary operators supported in expressions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
}

/// Expression AST node
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expr {
    /// Reference to another token (e.g., {token_name} or {group.token_name})
    Reference(String),
    /// Numeric literal
    Literal(f64),
    /// Binary operation
    BinaryOp {
        op: BinaryOperator,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

impl Expr {
    /// Parse an expression string into an AST
    pub fn parse(input: &str) -> TokenResult<Self> {
        match parse_expression(input.trim()) {
            Ok((remaining, expr)) => {
                if remaining.trim().is_empty() {
                    Ok(expr)
                } else {
                    Err(TokenError::ExpressionParsing(format!(
                        "Unexpected remaining input: '{}'",
                        remaining
                    )))
                }
            }
            Err(e) => Err(TokenError::ExpressionParsing(format!(
                "Failed to parse expression '{}': {}",
                input, e
            ))),
        }
    }

    /// Check if this expression contains any token references
    pub fn has_references(&self) -> bool {
        match self {
            Expr::Reference(_) => true,
            Expr::Literal(_) => false,
            Expr::BinaryOp { left, right, .. } => left.has_references() || right.has_references(),
        }
    }

    /// Get all token references in this expression
    pub fn get_references(&self) -> Vec<String> {
        match self {
            Expr::Reference(name) => vec![name.clone()],
            Expr::Literal(_) => vec![],
            Expr::BinaryOp { left, right, .. } => {
                let mut refs = left.get_references();
                refs.extend(right.get_references());
                refs
            }
        }
    }
}

// Nom parser functions

fn parse_expression(input: &str) -> IResult<&str, Expr> {
    parse_additive(input)
}

fn parse_additive(input: &str) -> IResult<&str, Expr> {
    let (input, init) = parse_multiplicative(input)?;

    let (input, ops) = many0(pair(
        delimited(multispace0, alt((tag("+"), tag("-"))), multispace0),
        parse_multiplicative,
    ))(input)?;

    Ok((
        input,
        ops.into_iter().fold(init, |acc, (op, val)| Expr::BinaryOp {
            op: match op {
                "+" => BinaryOperator::Add,
                "-" => BinaryOperator::Sub,
                _ => unreachable!(),
            },
            left: Box::new(acc),
            right: Box::new(val),
        }),
    ))
}

fn parse_multiplicative(input: &str) -> IResult<&str, Expr> {
    let (input, init) = parse_primary(input)?;

    let (input, ops) = many0(pair(
        delimited(multispace0, alt((tag("*"), tag("/"))), multispace0),
        parse_primary,
    ))(input)?;

    Ok((
        input,
        ops.into_iter().fold(init, |acc, (op, val)| Expr::BinaryOp {
            op: match op {
                "*" => BinaryOperator::Mul,
                "/" => BinaryOperator::Div,
                _ => unreachable!(),
            },
            left: Box::new(acc),
            right: Box::new(val),
        }),
    ))
}

fn parse_primary(input: &str) -> IResult<&str, Expr> {
    delimited(
        multispace0,
        alt((
            parse_parenthesized,
            parse_reference,
            parse_literal,
        )),
        multispace0,
    )(input)
}

fn parse_parenthesized(input: &str) -> IResult<&str, Expr> {
    delimited(char('('), parse_expression, char(')'))(input)
}

fn parse_reference(input: &str) -> IResult<&str, Expr> {
    map(
        delimited(
            char('{'),
            take_while1(|c: char| c.is_alphanumeric() || c == '_' || c == '.' || c == '-'),
            char('}'),
        ),
        |s: &str| Expr::Reference(s.to_string()),
    )(input)
}

fn parse_literal(input: &str) -> IResult<&str, Expr> {
    map(double, Expr::Literal)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_literal() {
        let expr = Expr::parse("42").unwrap();
        assert_eq!(expr, Expr::Literal(42.0));
    }

    #[test]
    fn test_parse_reference() {
        let expr = Expr::parse("{token_name}").unwrap();
        assert_eq!(expr, Expr::Reference("token_name".to_string()));
    }

    #[test]
    fn test_parse_dotted_reference() {
        let expr = Expr::parse("{group.token_name}").unwrap();
        assert_eq!(expr, Expr::Reference("group.token_name".to_string()));
    }

    #[test]
    fn test_parse_simple_addition() {
        let expr = Expr::parse("{a} + 3").unwrap();
        assert_eq!(
            expr,
            Expr::BinaryOp {
                op: BinaryOperator::Add,
                left: Box::new(Expr::Reference("a".to_string())),
                right: Box::new(Expr::Literal(3.0)),
            }
        );
    }

    #[test]
    fn test_parse_multiplication() {
        let expr = Expr::parse("{a}*3").unwrap();
        assert_eq!(
            expr,
            Expr::BinaryOp {
                op: BinaryOperator::Mul,
                left: Box::new(Expr::Reference("a".to_string())),
                right: Box::new(Expr::Literal(3.0)),
            }
        );
    }

    #[test]
    fn test_parse_complex_expression() {
        let expr = Expr::parse("({a} + {b}) * 2").unwrap();
        assert_eq!(
            expr,
            Expr::BinaryOp {
                op: BinaryOperator::Mul,
                left: Box::new(Expr::BinaryOp {
                    op: BinaryOperator::Add,
                    left: Box::new(Expr::Reference("a".to_string())),
                    right: Box::new(Expr::Reference("b".to_string())),
                }),
                right: Box::new(Expr::Literal(2.0)),
            }
        );
    }

    #[test]
    fn test_operator_precedence() {
        let expr = Expr::parse("2 + 3 * 4").unwrap();
        assert_eq!(
            expr,
            Expr::BinaryOp {
                op: BinaryOperator::Add,
                left: Box::new(Expr::Literal(2.0)),
                right: Box::new(Expr::BinaryOp {
                    op: BinaryOperator::Mul,
                    left: Box::new(Expr::Literal(3.0)),
                    right: Box::new(Expr::Literal(4.0)),
                }),
            }
        );
    }

    #[test]
    fn test_has_references() {
        let expr1 = Expr::parse("42").unwrap();
        assert!(!expr1.has_references());

        let expr2 = Expr::parse("{token}").unwrap();
        assert!(expr2.has_references());

        let expr3 = Expr::parse("{a} + 3").unwrap();
        assert!(expr3.has_references());
    }

    #[test]
    fn test_get_references() {
        let expr = Expr::parse("{a} + {b} * {c}").unwrap();
        let refs = expr.get_references();
        assert_eq!(refs, vec!["a", "b", "c"]);
    }
}