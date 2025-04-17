use anyhow::Result;
use graphqlgen_schema::ast::{Definition, Document};
use log::error;

use crate::core::common::lexers::Lexer;
use crate::core::common::token::Token;

use crate::core::document::input;
use crate::core::document::scalar;
use crate::core::document::type_def;

pub fn parse_document(tokens: Vec<Token>) -> Result<Document> {
    let mut definitions: Vec<Definition> = Vec::new();
    let mut index: usize = 0;

    while index < tokens.len() {
        match &tokens[index] {
            Token::Name(name) if name == "type" => {
                let def = type_def::parse_type(&tokens, &mut index)?;
                definitions.push(def);
            }

            Token::Name(name) if name == "input" => {
                let def = input::parse_input(&tokens, &mut index)?;
                definitions.push(def);
            }

            Token::Name(name) if name == "scalar" => {
                let def = scalar::parse_scalar(&tokens, &mut index)?;
                definitions.push(def);
            }

            /*Token::Name(name) if name == "interface" => {
                let def = interface::parse_interface(&tokens, &mut index)?;
                definitions.push(def);
            }

            Token::Name(name) if name == "union" => {
                let def = union::parse_union(&tokens, &mut index)?;
                definitions.push(def);
            }*/
            _ => index += 1, // Skip unrelated tokens
        }
    }

    Ok(Document { definitions })
}

pub fn generate_tokens(schema_content: &str) -> Result<Vec<Token>> {
    let lexer: Lexer<'_> = Lexer::new(&schema_content);
    let tokens: Vec<_> = lexer
        .filter_map(|t: Result<Token, String>| match t {
            Ok(t) if t != Token::EOF => Some(t),
            Ok(_) => None,
            Err(e) => {
                error!("Lexer error: {}", e);
                std::process::exit(1);
            }
        })
        .collect();

    Ok(tokens)
}
