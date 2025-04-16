use super::token::Token;
use anyhow::{bail, Result};

pub struct Lexer<'a> {
    chars: std::str::Chars<'a>,
    peeked: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        let mut chars = src.chars();
        let peeked = chars.next();
        Self { chars, peeked }
    }

    fn bump(&mut self) -> Option<char> {
        let current = self.peeked;
        self.peeked = self.chars.next();
        current
    }

    fn peek(&self) -> Option<char> {
        self.peeked
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek(), Some(c) if c.is_whitespace() || c == ',') {
            self.bump();
        }
    }

    pub fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace();
        match self.bump() {
            Some('{') => Ok(Token::BraceOpen),
            Some('}') => Ok(Token::BraceClose),
            Some('(') => Ok(Token::ParenOpen),
            Some(')') => Ok(Token::ParenClose),
            Some('[') => Ok(Token::BracketOpen),
            Some(']') => Ok(Token::BracketClose),
            Some(':') => Ok(Token::Colon),
            Some('=') => Ok(Token::Equals),
            Some('@') => Ok(Token::At),
            Some('$') => Ok(Token::Dollar),
            Some('!') => Ok(Token::Bang),
            Some('|') => Ok(Token::Pipe),
            Some('&') => Ok(Token::Ampersand),
            Some(',') => Ok(Token::Comma),
            Some('.') => {
                if self.peek() == Some('.') {
                    self.bump();
                    if self.bump() == Some('.') {
                        Ok(Token::Ellipsis)
                    } else {
                        bail!("Unexpected character after '..'")
                    }
                } else {
                    bail!("Unexpected character: '.'")
                }
            }
            Some('"') => {
                if self.peek() == Some('"') {
                    self.bump();
                    if self.peek() == Some('"') {
                        self.bump();
                        while let Some(c) = self.bump() {
                            if c == '"' && self.peek() == Some('"') {
                                self.bump();
                                if self.peek() == Some('"') {
                                    self.bump();
                                    break;
                                }
                            }
                        }
                        self.next_token()
                    } else {
                        bail!("Unexpected character after '\"'")
                    }
                } else {
                    self.read_string()
                }
            }
            Some(c) if c.is_alphabetic() || c == '_' => self.read_name_or_keyword(c),
            Some(c) if c.is_digit(10) || c == '-' => self.read_number(c),
            Some('#') => {
                while let Some(c) = self.bump() {
                    if c == '\n' {
                        break;
                    }
                }
                self.next_token()
            }
            None => Ok(Token::EOF),
            Some(c) => bail!("Unexpected character: '{}'", c),
        }
    }

    fn read_string(&mut self) -> Result<Token> {
        let mut s = String::new();

        while let Some(c) = self.bump() {
            if c == '"' {
                return Ok(Token::String(s));
            }
            s.push(c);
        }
        bail!("Unterminated string")
    }

    fn read_name_or_keyword(&mut self, first: char) -> Result<Token> {
        let mut name = String::new();
        name.push(first);
        while matches!(self.peek(), Some(c) if c.is_alphanumeric() || c == '_') {
            name.push(self.bump().unwrap());
        }

        match name.as_str() {
            "true" => Ok(Token::Boolean(true)),
            "false" => Ok(Token::Boolean(false)),
            "null" => Ok(Token::Null),
            _ => Ok(Token::Name(name)),
        }
    }

    fn read_number(&mut self, first: char) -> Result<Token> {
        let mut num = String::new();
        num.push(first);

        while let Some(c) = self.peek() {
            if c.is_digit(10) || c == '.' || c == 'e' || c == 'E' || c == '-' || c == '+' {
                num.push(self.bump().unwrap());
            } else {
                break;
            }
        }

        if num.contains('.') {
            Ok(Token::Float(num.parse()?))
        } else {
            Ok(Token::Int(num.parse()?))
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, String>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Ok(Token::EOF) => None,
            other => Some(other.map_err(|e| e.to_string())),
        }
    }
}
