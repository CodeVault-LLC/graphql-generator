use anyhow::Result;
use graphqlgen_schema::ast::{Directive, InputValue, TypeRef};

use crate::core::common::token::Token;

use super::{
    expect::{expect_name, expect_token},
    value::parse_value,
};

pub fn parse_directives(tokens: &[Token], index: &mut usize) -> Result<Vec<Directive>> {
    let mut directives: Vec<Directive> = Vec::new();

    while *index < tokens.len() && tokens[*index] == Token::At {
        *index += 1;

        let directive_name: String = expect_name(tokens, index)?;
        let mut args: Vec<InputValue> = Vec::new();

        if tokens.get(*index) == Some(&Token::ParenOpen) {
            *index += 1;

            while *index < tokens.len() && tokens.get(*index) != Some(&Token::ParenClose) {
                let name: String = expect_name(tokens, index)?;
                expect_token(tokens, index, Token::Colon)?;

                let value = parse_value(tokens, index)?;

                args.push(InputValue {
                    name,
                    value_type: TypeRef::Named("".to_string()),
                    default_value: Some(value),
                    description: None,
                });

                if tokens.get(*index) == Some(&Token::Comma) {
                    *index += 1;
                }
            }

            expect_token(tokens, index, Token::ParenClose)?;
        }

        directives.push(Directive {
            name: directive_name,
            arguments: if args.is_empty() { None } else { Some(args) },
        });
    }

    Ok(directives)
}
