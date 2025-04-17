use super::common::{expect_name, expect_token};
use super::token::Token;

use anyhow::Result;
use graphqlgen_schema::ast::{Definition, TypeRef, UnionDef};

pub fn parse_union(tokens: &[Token], index: &mut usize) -> Result<Definition> {
    *index += 1;
    let name = expect_name(tokens, index)?;
    expect_token(tokens, index, Token::Equals)?;

    let mut members = Vec::new();

    loop {
        match tokens.get(*index) {
            Some(Token::Name(member)) => {
                members.push(TypeRef::Named(member.clone()));
                *index += 1;
            }
            Some(Token::Pipe) => {
                *index += 1;
            }
            _ => break,
        }
    }

    Ok(Definition::Union(UnionDef {
        name,
        members,
        directives: None, // extend later if you add directives to unions
    }))
}
