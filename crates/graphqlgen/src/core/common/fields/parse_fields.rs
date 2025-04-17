use crate::core::common::{
    parse::{directives::parse_directives, expect::expect_token, type_ref::parse_type_ref},
    token::Token,
};
use anyhow::{anyhow, Result};
use graphqlgen_schema::ast::{Field, InputValue};

use super::parse_field_arguments::parse_field_arguments;

pub fn parse_fields(tokens: &[Token], index: &mut usize) -> Result<Vec<Field>> {
    let mut fields: Vec<Field> = Vec::new();

    while *index < tokens.len() {
        match &tokens[*index] {
            Token::BraceClose => {
                *index += 1;
                break;
            }

            Token::Name(field_name) => {
                let name = field_name.clone();
                *index += 1;

                let arguments = if tokens.get(*index) == Some(&Token::ParenOpen) {
                    Some(parse_field_arguments(tokens, index)?)
                } else {
                    None
                };

                expect_token(tokens, index, Token::Colon)?;
                let (field_type, consumed) = parse_type_ref(&tokens[*index..])?;
                *index += consumed;

                let directives = parse_directives(tokens, index)?;

                fields.push(Field {
                    name,
                    field_type,
                    arguments: arguments.map(|args| {
                        args.into_iter()
                            .map(|arg| InputValue {
                                name: arg.name,
                                value_type: arg.field_type,
                                default_value: None,
                            })
                            .collect()
                    }),
                    directives: if directives.is_empty() {
                        None
                    } else {
                        Some(directives)
                    },
                });
            }

            other => {
                return Err(anyhow!("Unexpected token in fields: {:?}", other));
            }
        }
    }

    Ok(fields)
}
