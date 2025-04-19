use anyhow::Result;
use graphqlgen_schema::ast::{Definition, ScalarDef};

use crate::core::common::parse::{directives::parse_directives, expect::expect_name};

use super::token::Token;

pub fn parse_scalar(
    tokens: &[Token],
    index: &mut usize,
    description: Option<String>,
) -> Result<Definition> {
    *index += 1;
    let scalar_name = expect_name(&tokens, index)?;
    let directives = parse_directives(&tokens, index)?;

    Ok(Definition::Scalar(ScalarDef {
        name: scalar_name,
        directives: if directives.is_empty() {
            None
        } else {
            Some(directives)
        },
        description,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_scalar_basic() {
        let tokens = vec![
            Token::Name("scalar".to_string()),
            Token::Name("Date".to_string()),
        ];
        let mut index = 0;
        let result = parse_scalar(&tokens, &mut index, None);

        assert!(result.is_ok());
        let def = result.unwrap();
        if let Definition::Scalar(ScalarDef {
            name,
            directives,
            description: _,
        }) = def
        {
            assert_eq!(name, "Date");
            assert!(directives.is_none());
        } else {
            panic!("Expected ScalarDef");
        }
        assert_eq!(index, 2);
    }

    #[test]
    fn test_parse_scalar_with_directive() {
        let tokens = vec![
            Token::Name("scalar".to_string()),
            Token::Name("Date".to_string()),
            Token::At,
            Token::Name("deprecated".to_string()),
        ];
        let mut index = 0;
        let result = parse_scalar(&tokens, &mut index, None);

        assert!(result.is_ok());
        let def = result.unwrap();
        if let Definition::Scalar(ScalarDef {
            name,
            directives,
            description: _,
        }) = def
        {
            assert_eq!(name, "Date");
            let directives = directives.expect("Expected some directives");
            assert_eq!(directives.len(), 1);
            assert_eq!(directives[0].name, "deprecated");
        } else {
            panic!("Expected ScalarDef");
        }
        assert_eq!(index, 4);
    }

    #[test]
    fn test_parse_scalar_with_directive_arguments() {
        let tokens = vec![
            Token::Name("scalar".to_string()),
            Token::Name("Date".to_string()),
            Token::At,
            Token::Name("directive".to_string()),
            Token::ParenOpen,
            Token::Name("reason".to_string()),
            Token::Colon,
            Token::String("legacy".to_string()),
            Token::ParenClose,
        ];
        let mut index = 0;
        let result = parse_scalar(&tokens, &mut index, None);

        assert!(result.is_ok());
        let def = result.unwrap();
        if let Definition::Scalar(ScalarDef {
            name,
            directives,
            description: _,
        }) = def
        {
            assert_eq!(name, "Date");
            let directives = directives.expect("Expected directives");
            assert_eq!(directives.len(), 1);
            assert_eq!(directives[0].name, "directive");

            let args = directives[0].arguments.as_ref().unwrap();
            assert_eq!(args[0].name, "reason");
            assert_eq!(
                format!("{:?}", args[0].default_value),
                "Some(String(\"legacy\"))"
            );
        } else {
            panic!("Expected ScalarDef");
        }
        assert_eq!(index, 9);
    }

    #[test]
    fn test_parse_scalar_missing_name() {
        let tokens = vec![
            Token::Name("scalar".to_string()),
            Token::Colon, // Invalid — colon instead of a name
        ];
        let mut index = 0;
        let result = parse_scalar(&tokens, &mut index, None);

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Expected name"), "Unexpected error: {err}");
    }

    #[test]
    fn test_parse_scalar_unexpected_token() {
        let tokens = vec![
            Token::Name("scalar".to_string()),
            Token::ParenOpen,
            Token::Name("Date".to_string()),
        ];
        let mut index = 0;
        let result = parse_scalar(&tokens, &mut index, None);

        assert!(result.is_err(), "Expected Err, got {:?}", result);
        if let Err(err) = result {
            assert_eq!(err.to_string(), "Expected name, got Some(ParenOpen)");
        } else {
            panic!("Expected Err, got {:?}", result);
        }
    }
}
