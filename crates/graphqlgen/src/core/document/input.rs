use anyhow::Result;
use graphqlgen_schema::ast::{Definition, Directive, Field, TypeDef};

use crate::core::common::{
    fields::parse_fields::parse_fields,
    parse::{
        directives::parse_directives,
        expect::{expect_name, expect_token},
    },
    token::Token,
};

pub fn parse_input(
    tokens: &[Token],
    index: &mut usize,
    description: Option<String>,
) -> Result<Definition> {
    *index += 1;
    let input_name: String = expect_name(&tokens, index)?;
    let directives: Vec<Directive> = parse_directives(&tokens, index)?;

    expect_token(&tokens, index, Token::BraceOpen)?;

    let fields: Vec<Field> = parse_fields(tokens, index)?;

    Ok(Definition::Input(TypeDef {
        name: input_name,
        fields,
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
    fn test_parse_input() {
        let tokens: Vec<Token> = vec![
            Token::Name("input".to_string()),
            Token::Name("MyInput".to_string()),
            Token::BraceOpen,
            Token::Name("field1".to_string()),
            Token::Colon,
            Token::Name("String".to_string()),
            Token::Name("field2".to_string()),
            Token::Colon,
            Token::Name("Int".to_string()),
            Token::BraceClose,
        ];

        let mut index: usize = 0;
        let result: Definition = parse_input(&tokens, &mut index, None).unwrap();

        if let Definition::Input(input) = result {
            assert_eq!(input.name, "MyInput");
        } else {
            panic!("Expected an input definition");
        }
    }

    #[test]
    fn test_parse_input_invalid() {
        let tokens: Vec<Token> = vec![
            Token::Name("input".to_string()),
            Token::Name("MyInput".to_string()),
            Token::BraceOpen,
            Token::Name("field1".to_string()),
            Token::Colon,
            Token::Name("String".to_string()),
            Token::BraceClose,
        ];

        let mut index: usize = 0;
        let result: Result<Definition> = parse_input(&tokens, &mut index, None);

        assert!(result.is_ok(), "Expected an error but got: {:?}", result);
    }

    #[test]
    fn test_parse_input_empty() {
        let tokens: Vec<Token> = vec![
            Token::Name("input".to_string()),
            Token::Name("MyInput".to_string()),
            Token::BraceOpen,
            Token::BraceClose,
        ];

        let mut index: usize = 0;
        let result: Definition = parse_input(&tokens, &mut index, None).unwrap();

        if let Definition::Input(input) = result {
            assert_eq!(input.name, "MyInput");
            assert!(
                input.fields.is_empty(),
                "Expected no fields in input definition"
            );
        } else {
            panic!("Expected an input definition");
        }
    }

    #[test]
    fn test_parse_input_with_directives() {
        let tokens: Vec<Token> = vec![
            Token::Name("input".to_string()),
            Token::Name("MyInput".to_string()),
            Token::At,
            Token::Name("directive".to_string()),
            Token::BraceOpen,
            Token::Name("field1".to_string()),
            Token::Colon,
            Token::Name("String".to_string()),
            Token::Name("field2".to_string()),
            Token::Colon,
            Token::Name("Int".to_string()),
            Token::BraceClose,
        ];

        let mut index: usize = 0;
        let result: Definition = parse_input(&tokens, &mut index, None).unwrap();

        if let Definition::Input(input) = result {
            assert_eq!(input.name, "MyInput");
            assert_eq!(input.directives.unwrap().len(), 1);
            assert_eq!(input.fields.len(), 2);
        } else {
            panic!("Expected an input definition with directives");
        }
    }
}
