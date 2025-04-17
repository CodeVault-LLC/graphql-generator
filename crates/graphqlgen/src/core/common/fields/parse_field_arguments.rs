use crate::core::common::{
    parse::{
        expect::{expect_name, expect_token},
        type_ref::parse_type_ref,
    },
    token::Token,
};
use anyhow::Result;
use graphqlgen_schema::ast::Field;

pub fn parse_field_arguments(tokens: &[Token], index: &mut usize) -> Result<Vec<Field>> {
    let mut args = Vec::new();
    *index += 1; // Skip '('

    while *index < tokens.len() && tokens.get(*index) != Some(&Token::ParenClose) {
        let name = expect_name(tokens, index)?;
        expect_token(tokens, index, Token::Colon)?;
        let (arg_type, consumed) = parse_type_ref(&tokens[*index..])?;
        *index += consumed;

        args.push(Field {
            name,
            field_type: arg_type,
            arguments: None,
            directives: None,
        });

        // Optional comma
        if tokens.get(*index) == Some(&Token::Comma) {
            *index += 1;
        }
    }

    expect_token(tokens, index, Token::ParenClose)?;
    Ok(args)
}
