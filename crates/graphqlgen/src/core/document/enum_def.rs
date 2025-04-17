use anyhow::Result;
use graphqlgen_schema::ast::{Definition, EnumDef, EnumValue};

use crate::core::common::{
    parse::{
        directives::parse_directives,
        expect::{expect_name, expect_token},
    },
    token::Token,
};

pub fn parse_enum(tokens: &[Token], index: &mut usize) -> Result<Definition> {
    *index += 1;

    let name = expect_name(tokens, index)?;
    let directives = parse_directives(tokens, index)?;

    expect_token(tokens, index, Token::BraceOpen)?;

    let mut values = Vec::new();

    while *index < tokens.len() {
        match tokens.get(*index) {
            Some(Token::BraceClose) => {
                *index += 1;
                break;
            }

            Some(Token::Name(_)) => {
                let name = expect_name(tokens, index)?;
                let directives = parse_directives(tokens, index)?;
                values.push(EnumValue {
                    name,
                    directives: if directives.is_empty() {
                        None
                    } else {
                        Some(directives)
                    },
                });
            }

            _ => break,
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
        let result = parse_enum(&tokens, &mut index).unwrap();
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
        let result = parse_enum(&tokens, &mut index).unwrap();

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
        let result = parse_enum(&tokens, &mut index).unwrap();

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
        let result = parse_enum(&tokens, &mut index);
        assert!(result.is_err());
    }
}
