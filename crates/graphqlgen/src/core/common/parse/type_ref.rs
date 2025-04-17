use anyhow::{anyhow, Result};
use graphqlgen_schema::ast::TypeRef;

use crate::core::common::token::Token;

pub fn parse_type_ref(tokens: &[Token]) -> Result<(TypeRef, usize)> {
    let mut consumed = 0;

    let base_type = match tokens.get(consumed) {
        Some(Token::Name(name)) => {
            consumed += 1;
            TypeRef::Named(name.clone())
        }
        Some(Token::BracketOpen) => {
            consumed += 1;
            let (inner, inner_consumed) = parse_type_ref(&tokens[consumed..])?;
            consumed += inner_consumed;

            if tokens.get(consumed) != Some(&Token::BracketClose) {
                return Err(anyhow!("Expected closing bracket for list type"));
            }
            consumed += 1;
            TypeRef::List(Box::new(inner))
        }
        other => return Err(anyhow!("Unexpected token in type reference: {:?}", other)),
    };

    if tokens.get(consumed) == Some(&Token::Bang) {
        consumed += 1;
        Ok((TypeRef::NonNull(Box::new(base_type)), consumed))
    } else {
        Ok((base_type, consumed))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::common::token::Token;

    #[test]
    fn test_named_type() {
        let tokens = vec![Token::Name("String".into())];
        let result = parse_type_ref(&tokens);
        assert!(result.is_ok());
        let (type_ref, consumed) = result.unwrap();
        assert_eq!(consumed, 1);
        assert!(matches!(type_ref, TypeRef::Named(name) if name == "String"));
    }

    #[test]
    fn test_non_null_named_type() {
        let tokens = vec![Token::Name("ID".into()), Token::Bang];
        let result = parse_type_ref(&tokens);
        assert!(result.is_ok());
        let (type_ref, consumed) = result.unwrap();
        assert_eq!(consumed, 2);
        match type_ref {
            TypeRef::NonNull(inner) => match *inner {
                TypeRef::Named(name) => assert_eq!(name, "ID"),
                _ => panic!("Expected Named inside NonNull"),
            },
            _ => panic!("Expected NonNull"),
        }
    }

    #[test]
    fn test_list_of_named() {
        let tokens = vec![
            Token::BracketOpen,
            Token::Name("String".into()),
            Token::BracketClose,
        ];
        let result = parse_type_ref(&tokens);
        assert!(result.is_ok());
        let (type_ref, consumed) = result.unwrap();
        assert_eq!(consumed, 3);
        match type_ref {
            TypeRef::List(inner) => match *inner {
                TypeRef::Named(name) => assert_eq!(name, "String"),
                _ => panic!("Expected Named inside List"),
            },
            _ => panic!("Expected List"),
        }
    }

    #[test]
    fn test_list_of_non_null_named() {
        let tokens = vec![
            Token::BracketOpen,
            Token::Name("String".into()),
            Token::Bang,
            Token::BracketClose,
        ];
        let result = parse_type_ref(&tokens);
        assert!(result.is_ok());
        let (type_ref, consumed) = result.unwrap();
        assert_eq!(consumed, 4);
        match type_ref {
            TypeRef::List(inner) => match *inner {
                TypeRef::NonNull(inner2) => match *inner2 {
                    TypeRef::Named(name) => assert_eq!(name, "String"),
                    _ => panic!("Expected Named inside NonNull inside List"),
                },
                _ => panic!("Expected NonNull inside List"),
            },
            _ => panic!("Expected List"),
        }
    }

    #[test]
    fn test_non_null_list() {
        let tokens = vec![
            Token::BracketOpen,
            Token::Name("Int".into()),
            Token::BracketClose,
            Token::Bang,
        ];
        let result = parse_type_ref(&tokens);
        assert!(result.is_ok());
        let (type_ref, consumed) = result.unwrap();
        assert_eq!(consumed, 4);
        match type_ref {
            TypeRef::NonNull(inner) => match *inner {
                TypeRef::List(inner2) => match *inner2 {
                    TypeRef::Named(name) => assert_eq!(name, "Int"),
                    _ => panic!("Expected Named inside List inside NonNull"),
                },
                _ => panic!("Expected List inside NonNull"),
            },
            _ => panic!("Expected NonNull"),
        }
    }

    #[test]
    fn test_nested_list_non_null() {
        let tokens = vec![
            Token::BracketOpen,
            Token::BracketOpen,
            Token::Name("String".into()),
            Token::BracketClose,
            Token::BracketClose,
            Token::Bang,
        ];
        let result = parse_type_ref(&tokens);
        assert!(result.is_ok());
        let (type_ref, consumed) = result.unwrap();
        assert_eq!(consumed, 6);

        match type_ref {
            TypeRef::NonNull(inner) => match *inner {
                TypeRef::List(inner2) => match *inner2 {
                    TypeRef::List(inner3) => match *inner3 {
                        TypeRef::Named(name) => assert_eq!(name, "String"),
                        _ => panic!("Expected Named inside nested List"),
                    },
                    _ => panic!("Expected List inside List"),
                },
                _ => panic!("Expected List inside NonNull"),
            },
            _ => panic!("Expected NonNull"),
        }
    }

    #[test]
    fn test_invalid_missing_closing_bracket() {
        let tokens = vec![Token::BracketOpen, Token::Name("String".into())];
        let result = parse_type_ref(&tokens);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_unexpected_token() {
        let tokens = vec![Token::Colon];
        let result = parse_type_ref(&tokens);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_input() {
        let tokens: Vec<Token> = vec![];
        let result = parse_type_ref(&tokens);
        assert!(result.is_err());
    }

    #[test]
    fn test_bang_without_base_type() {
        let tokens = vec![Token::Bang];
        let result = parse_type_ref(&tokens);
        assert!(result.is_err());
    }
}
