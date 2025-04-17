use anyhow::Result;
use graphqlgen_schema::ast::{Definition, TypeDef};

use crate::core::common::common::{expect_name, expect_token, parse_directives, parse_fields};
use crate::core::common::token::Token;

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
