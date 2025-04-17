use anyhow::Result;
use graphqlgen_schema::ast::{Definition, TypeDef};

use crate::core::common::{
    fields::parse_fields::parse_fields,
    parse::{
        directives::parse_directives,
        expect::{expect_name, expect_token},
    },
    token::Token,
};

pub fn parse_interface(tokens: &[Token], index: &mut usize) -> Result<Definition> {
    *index += 1;
    let name = expect_name(tokens, index)?;
    let directives = parse_directives(tokens, index)?;
    expect_token(tokens, index, Token::BraceOpen)?;

    let fields = parse_fields(tokens, index)?;

    Ok(Definition::Interface(TypeDef {
        name,
        fields,
        directives: if directives.is_empty() {
            None
        } else {
            Some(directives)
        },
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use graphqlgen_schema::ast::{Definition, TypeDef, TypeRef, Value};

    #[test]
    fn test_parse_interface_basic() {
        let tokens: Vec<Token> = vec![
            Token::Name("interface".to_string()),
            Token::Name("Node".to_string()),
            Token::BraceOpen,
            Token::Name("id".to_string()),
            Token::Colon,
            Token::Name("ID".to_string()),
            Token::BraceClose,
        ];

        let mut index = 0;
        let result = parse_interface(&tokens, &mut index);
        assert!(result.is_ok(), "Expected Ok, got {:?}", result);

        if let Definition::Interface(TypeDef {
            name,
            fields,
            directives,
        }) = result.unwrap()
        {
            assert_eq!(name, "Node");
            assert_eq!(fields.len(), 1);
            assert_eq!(fields[0].name, "id");
            match &fields[0].field_type {
                TypeRef::Named(name) => assert_eq!(name, "ID"),
                _ => panic!("Expected Named TypeRef"),
            }
            assert!(directives.is_none());
        } else {
            panic!("Expected Interface");
        }
    }

    #[test]
    fn test_parse_interface_with_directives() {
        let tokens = vec![
            Token::Name("interface".to_string()),
            Token::Name("Searchable".to_string()),
            Token::At,
            Token::Name("someDirective".to_string()),
            Token::ParenOpen,
            Token::Name("arg".to_string()),
            Token::Colon,
            Token::String("value".to_string()),
            Token::ParenClose,
            Token::BraceOpen,
            Token::Name("query".to_string()),
            Token::Colon,
            Token::Name("String".to_string()),
            Token::BraceClose,
        ];

        let mut index = 0;
        let result = parse_interface(&tokens, &mut index);
        assert!(result.is_ok(), "Expected Ok, got {:?}", result);

        if let Definition::Interface(TypeDef {
            name,
            fields,
            directives,
        }) = result.unwrap()
        {
            assert_eq!(name, "Searchable");
            assert_eq!(fields.len(), 1);
            assert_eq!(fields[0].name, "query");

            let directives = directives.expect("Expected some directives");
            assert_eq!(directives.len(), 1);
            assert_eq!(directives[0].name, "someDirective");

            let args = directives[0].arguments.as_ref().unwrap();
            assert_eq!(args.len(), 1);
            assert_eq!(args[0].name, "arg");
            assert!(matches!(args[0].default_value, Some(Value::String(ref v)) if v == "value"));
        }
    }

    #[test]
    fn test_parse_interface_with_multiple_fields() {
        let tokens = vec![
            Token::Name("interface".to_string()),
            Token::Name("Thing".to_string()),
            Token::BraceOpen,
            Token::Name("id".to_string()),
            Token::Colon,
            Token::Name("ID".to_string()),
            Token::Name("title".to_string()),
            Token::Colon,
            Token::Name("String".to_string()),
            Token::BraceClose,
        ];

        let mut index = 0;
        let result = parse_interface(&tokens, &mut index);
        assert!(result.is_ok());

        if let Definition::Interface(TypeDef { name, fields, .. }) = result.unwrap() {
            assert_eq!(name, "Thing");
            assert_eq!(fields.len(), 2);
            assert_eq!(fields[0].name, "id");
            assert_eq!(fields[1].name, "title");
        }
    }

    #[test]
    fn test_parse_interface_missing_name() {
        let tokens = vec![
            Token::Name("interface".to_string()),
            Token::Colon,
            Token::BraceOpen,
        ];

        let mut index = 0;
        let result = parse_interface(&tokens, &mut index);
        assert!(result.is_err(), "Expected error for missing name");
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("Expected name"));
    }

    #[test]
    fn test_parse_interface_missing_brace() {
        let tokens = vec![
            Token::Name("interface".to_string()),
            Token::Name("Node".to_string()),
        ];

        let mut index = 0;
        let result = parse_interface(&tokens, &mut index);
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("Expected BraceOpen, got None"));
    }

    #[test]
    fn test_parse_interface_with_unexpected_token() {
        let tokens = vec![
            Token::Name("interface".to_string()),
            Token::Name("Broken".to_string()),
            Token::BraceOpen,
            Token::Colon, // Unexpected token
            Token::BraceClose,
        ];

        let mut index = 0;
        let result = parse_interface(&tokens, &mut index);
        assert!(result.is_err(), "Expected parse failure");
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("Unexpected token in fields"));
    }

    #[test]
    fn test_parse_interface_with_field_arguments() {
        let tokens = vec![
            Token::Name("interface".to_string()),
            Token::Name("Query".to_string()),
            Token::BraceOpen,
            Token::Name("find".to_string()),
            Token::ParenOpen,
            Token::Name("id".to_string()),
            Token::Colon,
            Token::Name("ID".to_string()),
            Token::ParenClose,
            Token::Colon,
            Token::Name("Result".to_string()),
            Token::BraceClose,
        ];

        let mut index = 0;
        let result = parse_interface(&tokens, &mut index);
        assert!(result.is_ok());

        if let Definition::Interface(TypeDef { name, fields, .. }) = result.unwrap() {
            assert_eq!(name, "Query");
            assert_eq!(fields.len(), 1);
            let field = &fields[0];
            assert_eq!(field.name, "find");

            let args = field.arguments.as_ref().unwrap();
            assert_eq!(args.len(), 1);
            assert_eq!(args[0].name, "id");
            if let TypeRef::Named(name) = &args[0].value_type {
                assert_eq!(name, "ID");
            } else {
                panic!("Expected TypeRef::Named");
            }
        }
    }
}
