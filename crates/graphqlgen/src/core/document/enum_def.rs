use anyhow::Result;
use graphqlgen_schema::ast::{Definition, EnumDef, EnumValue};

use crate::core::common::{
    parse::{
        directives::parse_directives,
        expect::{expect_name, expect_token},
    },
    token::Token,
};

pub fn parse_enum(
    tokens: &[Token],
    index: &mut usize,
    description: Option<String>,
) -> Result<Definition> {
    *index += 1;

    let name: String = expect_name(tokens, index)?;
    let directives: Vec<graphqlgen_schema::ast::Directive> = parse_directives(tokens, index)?;

    expect_token(tokens, index, Token::BraceOpen)?;

    let mut values: Vec<EnumValue> = Vec::new();

    while *index < tokens.len() {
        let mut sub_description: Option<String> = None;

        match tokens.get(*index) {
            Some(Token::BraceClose) => {
                *index += 1;
                break;
            }

            Some(Token::Description(desc)) => {
                sub_description = Some(desc.clone());
                *index += 1;
            }

            Some(Token::Name(_)) => {
                let name: String = expect_name(tokens, index)?;
                let directives = parse_directives(tokens, index)?;
                values.push(EnumValue {
                    name,
                    description: sub_description,
                    directives: if directives.is_empty() {
                        None
                    } else {
                        Some(directives)
                    },
                });
            }

            _ => {
                println!("Unexpected token in enum definition: {:?}", tokens[*index]);
                break;
            }
        }
    }

    Ok(Definition::Enum(EnumDef {
        name,
        values,
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
    fn test_parse_simple_enum() {
        let tokens = vec![
            Token::Name("enum".into()),
            Token::Name("Role".into()),
            Token::BraceOpen,
            Token::Name("ADMIN".into()),
            Token::Name("USER".into()),
            Token::Name("GUEST".into()),
            Token::BraceClose,
        ];

        let mut index = 0;
        let result = parse_enum(&tokens, &mut index, None).unwrap();
        assert_eq!(index, 7);

        if let Definition::Enum(enum_def) = result {
            assert_eq!(enum_def.name, "Role");
            assert_eq!(enum_def.values.len(), 3);
            assert_eq!(enum_def.values[0].name, "ADMIN");
            assert_eq!(enum_def.values[1].name, "USER");
            assert_eq!(enum_def.values[2].name, "GUEST");
        } else {
            panic!("Expected Definition::Enum");
        }
    }

    #[test]
    fn test_parse_enum_with_directives() {
        let tokens = vec![
            Token::Name("enum".into()),
            Token::Name("Status".into()),
            Token::At,
            Token::Name("deprecated".into()),
            Token::BraceOpen,
            Token::Name("ACTIVE".into()),
            Token::Name("INACTIVE".into()),
            Token::BraceClose,
        ];
        let mut index = 0;
        let result = parse_enum(&tokens, &mut index, None).unwrap();

        if let Definition::Enum(enum_def) = result {
            assert_eq!(enum_def.name, "Status");
            assert!(enum_def.directives.is_some());
            assert_eq!(enum_def.values.len(), 2);
        }
    }

    #[test]
    fn test_parse_enum_values_with_directives() {
        let tokens = vec![
            Token::Name("enum".into()),
            Token::Name("Color".into()),
            Token::BraceOpen,
            Token::Name("RED".into()),
            Token::At,
            Token::Name("deprecated".into()),
            Token::ParenOpen,
            Token::Name("reason".into()),
            Token::Colon,
            Token::String("use CRIMSON".into()),
            Token::ParenClose,
            Token::Name("BLUE".into()),
            Token::BraceClose,
        ];
        let mut index = 0;
        let result = parse_enum(&tokens, &mut index, None).unwrap();

        if let Definition::Enum(enum_def) = result {
            assert_eq!(enum_def.name, "Color");
            assert_eq!(enum_def.values.len(), 2);
            assert_eq!(enum_def.values[0].name, "RED");
            assert!(enum_def.values[0].directives.is_some());
            assert_eq!(enum_def.values[1].name, "BLUE");
        }
    }

    #[test]
    fn test_invalid_enum_missing_name() {
        let tokens = vec![Token::Name("enum".into())];
        let mut index = 0;
        let result = parse_enum(&tokens, &mut index, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_enum_with_comments() {
        let tokens = vec![
            Token::Description(("A enum with comments").into()),
            Token::Name("enum".into()),
            Token::Name("CommentedEnum".into()),
            Token::BraceOpen,
            Token::Description("This is a comment".into()),
            Token::Name("VALUE1".into()),
            Token::Name("VALUE2".into()),
            Token::BraceClose,
        ];
        let mut index = 0;
        let result = parse_enum(&tokens, &mut index, None).unwrap();

        if let Definition::Enum(enum_def) = result {
            assert_eq!(enum_def.description, Some("A enum with comments".into()));
            assert_eq!(enum_def.name, "CommentedEnum");
            assert_eq!(enum_def.values.len(), 2);
            assert_eq!(
                enum_def.values[0].description,
                Some("This is a comment".into())
            );
            assert_eq!(enum_def.values[0].name, "VALUE1");
            assert_eq!(enum_def.values[1].name, "VALUE2");
        }
    }
}
