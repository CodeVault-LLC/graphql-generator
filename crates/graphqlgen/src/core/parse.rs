use anyhow::Result;
use graphqlgen_schema::ast::{Definition, Document};
use log::error;

use crate::core::common::lexers::Lexer;
use crate::core::common::token::Token;

use crate::core::document::input;
use crate::core::document::interface;
use crate::core::document::scalar;
use crate::core::document::type_def;
use crate::core::document::union;

use super::document::enum_def;
pub fn parse_document(tokens: Vec<Token>) -> Result<Document> {
    let mut definitions: Vec<Definition> = Vec::new();
    let mut index: usize = 0;
    let mut pending_description: Option<String> = None;

    while index < tokens.len() {
        match &tokens[index] {
            Token::Description(desc) => {
                // Save description and move to the next token
                pending_description = Some(desc.clone());
                index += 1;
            }

            Token::Name(name) if name == "type" => {
                let def: Definition =
                    type_def::parse_type(&tokens, &mut index, pending_description.take())?;
                definitions.push(def);
            }

            Token::Name(name) if name == "input" => {
                let def: Definition =
                    input::parse_input(&tokens, &mut index, pending_description.take())?;
                definitions.push(def);
            }

            Token::Name(name) if name == "scalar" => {
                let def: Definition =
                    scalar::parse_scalar(&tokens, &mut index, pending_description.take())?;
                definitions.push(def);
            }

            Token::Name(name) if name == "interface" => {
                let def: Definition =
                    interface::parse_interface(&tokens, &mut index, pending_description.take())?;
                definitions.push(def);
            }

            Token::Name(name) if name == "union" => {
                let def: Definition =
                    union::parse_union(&tokens, &mut index, pending_description.take())?;
                definitions.push(def);
            }

            Token::Name(name) if name == "enum" => {
                let def: Definition =
                    enum_def::parse_enum(&tokens, &mut index, pending_description.take())?;
                definitions.push(def);
            }

            _ => {
                index += 1;
                pending_description = None;
            }
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
