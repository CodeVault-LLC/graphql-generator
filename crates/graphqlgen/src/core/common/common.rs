use anyhow::{anyhow, Result};
use graphqlgen_schema::ast::{Directive, Field, InputValue, TypeRef, Value};
use log::{debug, error};

use crate::core::common::token::Token;

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

pub fn expect_name(tokens: &[Token], index: &mut usize) -> Result<String> {
    match tokens.get(*index) {
        Some(Token::Name(name)) => {
            *index += 1;
            Ok(name.clone())
        }
        other => Err(anyhow!("Expected name, got {:?}", other)),
    }
}

pub fn expect_token(tokens: &[Token], index: &mut usize, expected: Token) -> Result<()> {
    match tokens.get(*index) {
        Some(tok) if *tok == expected => {
            *index += 1;
            Ok(())
        }
        other => {
            debug!("Tokens: {:?}", tokens);
            error!("Expected {:?}, got {:?}", expected, other);
            Err(anyhow!("Expected {:?}, got {:?}", expected, other))
        }
    }
}

pub fn parse_type_ref(tokens: &[Token]) -> Result<(TypeRef, usize)> {
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

    if tokens.get(consumed) == Some(&Token::Bang) {
        consumed += 1;
        Ok((TypeRef::NonNull(Box::new(base_type)), consumed))
    } else {
        Ok((base_type, consumed))
    }
}

pub fn parse_directives(tokens: &[Token], index: &mut usize) -> Result<Vec<Directive>> {
    let mut directives = Vec::new();

    while *index < tokens.len() && tokens[*index] == Token::At {
        *index += 1;

        let directive_name = expect_name(tokens, index)?;
        let mut args = Vec::new();

        if tokens.get(*index) == Some(&Token::ParenOpen) {
            *index += 1;

            while *index < tokens.len() && tokens.get(*index) != Some(&Token::ParenClose) {
                let name = expect_name(tokens, index)?;
                expect_token(tokens, index, Token::Colon)?;

                let value = parse_value(tokens, index)?;

                args.push(InputValue {
                    name,
                    value_type: TypeRef::Named("".to_string()),
                    default_value: Some(value),
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

pub fn parse_value(tokens: &[Token], index: &mut usize) -> Result<Value> {
    match tokens.get(*index) {
        Some(Token::String(s)) => {
            *index += 1;
            Ok(Value::String(s.clone()))
        }
        Some(Token::Int(i)) => {
            *index += 1;
            Ok(Value::Int(*i)) // assuming Value::Int(i32) exists in your AST
        }
        Some(Token::Name(n)) if n == "true" || n == "false" => {
            let b = n == "true";
            *index += 1;
            Ok(Value::Bool(b))
        }
        Some(Token::Name(n)) => {
            *index += 1;
            Ok(Value::Enum(n.clone()))
        }
        Some(Token::BraceOpen) => {
            *index += 1;
            let mut fields = Vec::new();
            while *index < tokens.len() && tokens.get(*index) != Some(&Token::BraceClose) {
                let key = expect_name(tokens, index)?;
                expect_token(tokens, index, Token::Colon)?;
                let val = parse_value(tokens, index)?;
                fields.push((key, val));

                if tokens.get(*index) == Some(&Token::Comma) {
                    *index += 1;
                }
            }
            expect_token(tokens, index, Token::BraceClose)?;
            Ok(Value::Object(fields))
        }
        Some(Token::BracketOpen) => {
            *index += 1;
            let mut items = Vec::new();
            while *index < tokens.len() && tokens.get(*index) != Some(&Token::BracketClose) {
                let val = parse_value(tokens, index)?;
                items.push(val);

                if tokens.get(*index) == Some(&Token::Comma) {
                    *index += 1;
                }
            }
            expect_token(tokens, index, Token::BracketClose)?;
            Ok(Value::List(items))
        }
        other => Err(anyhow!("Unexpected token as value: {:?}", other)),
    }
}
