use anyhow::{anyhow, Result};
use graphqlgen_schema::ast::{
    Definition, Directive, Document, Field, InputValue, ScalarDef, TypeDef, TypeRef,
};

use super::token::Token;

pub fn parse_document(tokens: Vec<Token>) -> Result<Document> {
    let mut definitions: Vec<Definition> = Vec::new();
    let mut index = 0;

    while index < tokens.len() {
        match &tokens[index] {
            Token::Name(name) if name == "type" => {
                index += 1;

                let type_name = match tokens.get(index) {
                    Some(Token::Name(name)) => name.clone(),
                    other => return Err(anyhow!("Expected type name, got {:?}", other)),
                };
                index += 1;

                // Expect {
                if tokens.get(index) != Some(&Token::BraceOpen) {
                    return Err(anyhow!("Expected '{{' after type name"));
                }
                index += 1;

                let mut fields = Vec::new();

                while index < tokens.len() {
                    match &tokens[index] {
                        // Handle when we reach the end of the fields
                        Token::BraceClose => {
                            index += 1;
                            break;
                        }

                        // Handle field definitions
                        Token::Name(field_name) => {
                            let name: String = field_name.clone();
                            let mut arguments: Option<Vec<Field>> = None;

                            index += 1;

                            // Handle optional arguments in parentheses
                            if tokens.get(index) == Some(&Token::ParenOpen) {
                                index += 1;
                                let mut args = Vec::new();

                                while index < tokens.len()
                                    && tokens.get(index) != Some(&Token::ParenClose)
                                {
                                    // Parse argument name
                                    let arg_name = match tokens.get(index) {
                                        Some(Token::Name(name)) => name.clone(),
                                        other => {
                                            return Err(anyhow!(
                                                "Expected argument name, got {:?}",
                                                other
                                            ))
                                        }
                                    };
                                    index += 1;

                                    // Expect :
                                    if tokens.get(index) != Some(&Token::Colon) {
                                        return Err(anyhow!(
                                            "Expected ':' after argument name, got {:?}",
                                            tokens.get(index)
                                        ));
                                    }
                                    index += 1;

                                    // Parse argument type
                                    let (arg_type, consumed) = parse_type_ref(&tokens[index..])?;
                                    index += consumed;

                                    args.push(Field {
                                        name: arg_name,
                                        field_type: arg_type,
                                        arguments: None, // Arguments for the argument itself (if any)
                                        directives: None, // Directives for the argument (if any)
                                    });
                                }

                                if tokens.get(index) == Some(&Token::ParenClose) {
                                    index += 1;
                                    arguments = Some(args);
                                } else {
                                    return Err(anyhow!("Unclosed parentheses after field name"));
                                }
                            }

                            // Expect :
                            if tokens.get(index) != Some(&Token::Colon) {
                                return Err(anyhow!(
                                    "Expected ':' after field name, got {:?}",
                                    tokens.get(index)
                                ));
                            }
                            index += 1;

                            let (type_ref, consumed) = parse_type_ref(&tokens[index..])?;
                            index += consumed;

                            let mut directives = Vec::new();

                            while tokens.get(index) == Some(&Token::At) {
                                index += 1;

                                let directive_name = match tokens.get(index) {
                                    Some(Token::Name(n)) => n.clone(),
                                    other => {
                                        return Err(anyhow!(
                                            "Expected directive name, got {:?}",
                                            other
                                        ))
                                    }
                                };
                                index += 1;

                                // Optionally support arguments later here...
                                directives.push(Directive {
                                    name: directive_name,
                                    arguments: None,
                                });
                            }

                            fields.push(Field {
                                name,
                                field_type: type_ref,
                                arguments: arguments.map(|args| {
                                    args.into_iter()
                                        .map(|arg| InputValue {
                                            name: arg.name,
                                            value_type: arg.field_type,
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

                        // Handle directives (e.g. @deprecated)
                        // TODO: Handle directives properly
                        Token::At => {
                            index += 1;
                            while index < tokens.len() && tokens[index] != Token::BraceClose {
                                index += 1;
                            }
                        }

                        other => return Err(anyhow!("Unexpected token in fields: {:?}", other)),
                    }
                }

                definitions.push(Definition::Type(TypeDef {
                    name: type_name,
                    fields,
                }));
            }

            Token::Name(name) if name == "scalar" => {
                index += 1;

                let scalar_name: String = match tokens.get(index) {
                    Some(Token::Name(name)) => name.clone(),
                    other => return Err(anyhow!("Expected scalar name, got {:?}", other)),
                };
                index += 1;

                definitions.push(Definition::Scalar(ScalarDef { name: scalar_name }));
            }

            _ => index += 1, // Skip unrelated tokens (e.g. comments or extra whitespace)
        }
    }

    Ok(Document { definitions })
}

fn parse_type_ref(tokens: &[Token]) -> Result<(TypeRef, usize)> {
    let mut consumed = 0;

    let base_type = match tokens.get(consumed) {
        Some(Token::Name(name)) => {
            consumed += 1;
            TypeRef::Named(name.clone())
        }
        Some(Token::BracketOpen) => {
            consumed += 1;
            let (inner, inner_consumed) = parse_type_ref(&tokens[consumed..])?;
            consumed += inner_consumed;

            if tokens.get(consumed) != Some(&Token::BracketClose) {
                return Err(anyhow!("Expected closing bracket for list type"));
            }
            consumed += 1;

            TypeRef::List(Box::new(inner))
        }
        other => return Err(anyhow!("Unexpected token in type reference: {:?}", other)),
    };

    // Optional !
    if tokens.get(consumed) == Some(&Token::Bang) {
        consumed += 1;
        Ok((TypeRef::NonNull(Box::new(base_type)), consumed))
    } else {
        Ok((base_type, consumed))
    }
}
