use anyhow::{anyhow, Result};
use graphqlgen_schema::ast::Value;

use crate::core::common::token::Token;

use super::expect::{expect_name, expect_token};

pub fn parse_value(tokens: &[Token], index: &mut usize) -> Result<Value> {
    match tokens.get(*index) {
        Some(Token::String(s)) => {
            *index += 1;
            Ok(Value::String(s.clone()))
        }
        Some(Token::Int(i)) => {
            *index += 1;
            Ok(Value::Int(*i)) // assuming Value::Int(i32) exists in your AST
        }
        Some(Token::Name(n)) if n == "true" || n == "false" => {
            let b = n == "true";
            *index += 1;
            Ok(Value::Bool(b))
        }
        Some(Token::Name(n)) => {
            *index += 1;
            Ok(Value::Enum(n.clone()))
        }
        Some(Token::BraceOpen) => {
            *index += 1;
            let mut fields = Vec::new();
            while *index < tokens.len() && tokens.get(*index) != Some(&Token::BraceClose) {
                let key = expect_name(tokens, index)?;
                expect_token(tokens, index, Token::Colon)?;
                let val = parse_value(tokens, index)?;
                fields.push((key, val));

                if tokens.get(*index) == Some(&Token::Comma) {
                    *index += 1;
                }
            }
            expect_token(tokens, index, Token::BraceClose)?;
            Ok(Value::Object(fields))
        }
        Some(Token::BracketOpen) => {
            *index += 1;
            let mut items = Vec::new();
            while *index < tokens.len() && tokens.get(*index) != Some(&Token::BracketClose) {
                let val = parse_value(tokens, index)?;
                items.push(val);

                if tokens.get(*index) == Some(&Token::Comma) {
                    *index += 1;
                }
            }
            expect_token(tokens, index, Token::BracketClose)?;
            Ok(Value::List(items))
        }
        other => Err(anyhow!("Unexpected token as value: {:?}", other)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::common::token::Token;

    #[test]
    fn test_parse_string() {
        let tokens = vec![Token::String("hello".into())];
        let mut index = 0;
        let result = parse_value(&tokens, &mut index).unwrap();
        assert_eq!(result, Value::String("hello".into()));
        assert_eq!(index, 1);
    }

    #[test]
    fn test_parse_int() {
        let tokens = vec![Token::Int(42)];
        let mut index = 0;
        let result = parse_value(&tokens, &mut index).unwrap();
        assert_eq!(result, Value::Int(42));
        assert_eq!(index, 1);
    }

    #[test]
    fn test_parse_bool_true() {
        let tokens = vec![Token::Name("true".into())];
        let mut index = 0;
        let result = parse_value(&tokens, &mut index).unwrap();
        assert_eq!(result, Value::Bool(true));
        assert_eq!(index, 1);
    }

    #[test]
    fn test_parse_bool_false() {
        let tokens = vec![Token::Name("false".into())];
        let mut index = 0;
        let result = parse_value(&tokens, &mut index).unwrap();
        assert_eq!(result, Value::Bool(false));
        assert_eq!(index, 1);
    }

    #[test]
    fn test_parse_enum() {
        let tokens = vec![Token::Name("SOME_ENUM".into())];
        let mut index = 0;
        let result = parse_value(&tokens, &mut index).unwrap();
        assert_eq!(result, Value::Enum("SOME_ENUM".into()));
        assert_eq!(index, 1);
    }

    #[test]
    fn test_parse_object() {
        let tokens = vec![
            Token::BraceOpen,
            Token::Name("foo".into()),
            Token::Colon,
            Token::Int(7),
            Token::Comma,
            Token::Name("bar".into()),
            Token::Colon,
            Token::String("baz".into()),
            Token::BraceClose,
        ];
        let mut index = 0;
        let result = parse_value(&tokens, &mut index).unwrap();
        assert_eq!(index, 9);
        assert_eq!(
            result,
            Value::Object(vec![
                ("foo".into(), Value::Int(7)),
                ("bar".into(), Value::String("baz".into()))
            ])
        );
    }

    #[test]
    fn test_parse_list() {
        let tokens = vec![
            Token::BracketOpen,
            Token::String("one".into()),
            Token::Comma,
            Token::Int(2),
            Token::Comma,
            Token::Name("false".into()),
            Token::BracketClose,
        ];
        let mut index = 0;
        let result = parse_value(&tokens, &mut index).unwrap();
        assert_eq!(index, 7);
        assert_eq!(
            result,
            Value::List(vec![
                Value::String("one".into()),
                Value::Int(2),
                Value::Bool(false)
            ])
        );
    }

    #[test]
    fn test_parse_nested() {
        let tokens = vec![
            Token::BraceOpen,
            Token::Name("nested".into()),
            Token::Colon,
            Token::BracketOpen,
            Token::BraceOpen,
            Token::Name("ok".into()),
            Token::Colon,
            Token::Name("true".into()),
            Token::BraceClose,
            Token::BracketClose,
            Token::BraceClose,
        ];
        let mut index = 0;
        let result = parse_value(&tokens, &mut index).unwrap();
        assert_eq!(index, 11);
        assert_eq!(
            result,
            Value::Object(vec![(
                "nested".into(),
                Value::List(vec![Value::Object(vec![("ok".into(), Value::Bool(true))])])
            )])
        );
    }

    // --- INVALID CASES ---

    #[test]
    fn test_unexpected_token_error() {
        let tokens = vec![Token::Colon];
        let mut index = 0;
        let result = parse_value(&tokens, &mut index);
        assert!(result.is_err());
    }

    #[test]
    fn test_object_missing_colon() {
        let tokens = vec![
            Token::BraceOpen,
            Token::Name("foo".into()),
            Token::String("oops".into()), // Missing colon
            Token::BraceClose,
        ];
        let mut index = 0;
        let result = parse_value(&tokens, &mut index);
        assert!(result.is_err());
    }

    #[test]
    fn test_object_missing_brace_close() {
        let tokens = vec![
            Token::BraceOpen,
            Token::Name("x".into()),
            Token::Colon,
            Token::Int(1),
        ]; // No closing brace
        let mut index = 0;
        let result = parse_value(&tokens, &mut index);
        assert!(result.is_err());
    }

    #[test]
    fn test_list_missing_bracket_close() {
        let tokens = vec![
            Token::BracketOpen,
            Token::Int(1),
            Token::Comma,
            Token::Int(2),
        ];
        let mut index = 0;
        let result = parse_value(&tokens, &mut index);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_token_stream() {
        let tokens = vec![];
        let mut index = 0;
        let result = parse_value(&tokens, &mut index);
        assert!(result.is_err());
    }
}
