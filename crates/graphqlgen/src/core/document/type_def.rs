use anyhow::Result;
use graphqlgen_schema::ast::{Definition, Directive, TypeDef};

use super::common::{expect_name, expect_token, parse_directives, parse_fields};
use super::token::Token;

pub fn parse_type(tokens: &[Token], index: &mut usize) -> Result<Definition> {
    *index += 1;
    let type_name: String = expect_name(&tokens, index)?;
    let directives: Vec<Directive> = parse_directives(&tokens, index)?;

    expect_token(&tokens, index, Token::BraceOpen)?;

    let fields = parse_fields(&tokens, index)?;

    Ok(Definition::Type(TypeDef {
        name: type_name,
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
    use graphqlgen_schema::ast::TypeRef;

    use super::*;

    #[test]
    fn test_parse_type() {
        let tokens: Vec<Token> = vec![
            Token::Name("type".to_string()),
            Token::Name("Address".to_string()),
            Token::BraceOpen,
            Token::Name("address".to_string()),
            Token::Colon,
            Token::Name("String".to_string()),
            Token::Name("city".to_string()),
            Token::Colon,
            Token::Name("String".to_string()),
            Token::BraceClose,
        ];

        let mut index: usize = 0;
        let result: std::result::Result<Definition, anyhow::Error> =
            parse_type(&tokens, &mut index);
        assert!(result.is_ok(), "Expected Ok, got {:?}", result);
        let definition: Definition = result.unwrap();

        if let Definition::Type(type_def) = definition {
            assert_eq!(type_def.name, "Address");
            assert_eq!(type_def.fields.len(), 2);
            assert_eq!(type_def.fields[0].name, "address");
            if let TypeRef::Named(ref name) = type_def.fields[0].field_type {
                assert_eq!(name, "String");
            } else {
                panic!(
                    "Expected TypeRef::Named(\"String\"), got {:?}",
                    type_def.fields[0].field_type
                );
            }
            assert_eq!(type_def.fields[1].name, "city");
            if let TypeRef::Named(ref name) = type_def.fields[1].field_type {
                assert_eq!(name, "String");
            } else {
                panic!(
                    "Expected TypeRef::Named(\"String\"), got {:?}",
                    type_def.fields[1].field_type
                );
            }
            assert_eq!(index, 10);

            assert!(type_def.directives.is_none());
        } else {
            panic!("Expected TypeDef, got {:?}", definition);
        }
    }

    #[test]
    fn test_parse_type_invalid() {
        let tokens: Vec<Token> = vec![
            Token::Name("type".to_string()),
            Token::Name("InvalidAddress".to_string()),
            Token::BraceOpen,
            Token::Name("address".to_string()),
            Token::Colon,
            Token::Colon,
            Token::Name("String".to_string()),
            Token::BraceClose,
        ];

        let mut index: usize = 0;
        let result: std::result::Result<Definition, anyhow::Error> =
            parse_type(&tokens, &mut index);
        assert!(result.is_err(), "Expected Err, got {:?}", result);
        if let Err(err) = result {
            assert_eq!(
                err.to_string(),
                "Unexpected token in type reference: Some(Colon)"
            );
        } else {
            panic!("Expected Err, got {:?}", result);
        }
    }

    #[test]
    fn test_parse_type_with_directives() {
        let tokens: Vec<Token> = vec![
            Token::Name("type".to_string()),
            Token::Name("Address".to_string()),
            Token::BraceOpen,
            Token::Name("address".to_string()),
            Token::Colon,
            Token::Name("String".to_string()),
            Token::Name("city".to_string()),
            Token::Colon,
            Token::Name("String".to_string()),
            Token::BraceClose,
        ];

        let mut index: usize = 0;
        let result: std::result::Result<Definition, anyhow::Error> =
            parse_type(&tokens, &mut index);
        assert!(result.is_ok(), "Expected Ok, got {:?}", result);
        let definition: Definition = result.unwrap();

        if let Definition::Type(type_def) = definition {
            assert_eq!(type_def.name, "Address");
            assert_eq!(type_def.fields.len(), 2);
            assert_eq!(type_def.fields[0].name, "address");
            if let TypeRef::Named(ref name) = type_def.fields[0].field_type {
                assert_eq!(name, "String");
            } else {
                panic!(
                    "Expected TypeRef::Named(\"String\"), got {:?}",
                    type_def.fields[0].field_type
                );
            }
            assert_eq!(type_def.fields[1].name, "city");
            if let TypeRef::Named(ref name) = type_def.fields[1].field_type {
                assert_eq!(name, "String");
            } else {
                panic!(
                    "Expected TypeRef::Named(\"String\"), got {:?}",
                    type_def.fields[1].field_type
                );
            }
            assert_eq!(index, 10);

            assert!(type_def.directives.is_none());
        } else {
            panic!("Expected TypeDef, got {:?}", definition);
        }
    }
}
