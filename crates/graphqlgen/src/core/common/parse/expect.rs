use anyhow::{anyhow, Result};
use log::{debug, error};

use crate::core::common::token::Token;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::common::token::Token;

    #[test]
    fn test_expect_name_success() {
        let tokens = vec![Token::Name("test".to_string())];
        let mut index = 0;
        let result = expect_name(&tokens, &mut index);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test");
        assert_eq!(index, 1);
    }

    #[test]
    fn test_expect_name_failure_wrong_token() {
        let tokens = vec![Token::BraceOpen];
        let mut index = 0;
        let result = expect_name(&tokens, &mut index);
        assert!(result.is_err());
        assert_eq!(index, 0, "Index should not advance on failure");
    }

    #[test]
    fn test_expect_name_failure_out_of_bounds() {
        let tokens: Vec<Token> = vec![];
        let mut index = 0;
        let result = expect_name(&tokens, &mut index);
        assert!(result.is_err());
        assert_eq!(index, 0, "Index should remain unchanged");
    }

    #[test]
    fn test_expect_token_success() {
        let tokens = vec![Token::Equals];
        let mut index = 0;
        let result = expect_token(&tokens, &mut index, Token::Equals);
        assert!(result.is_ok());
        assert_eq!(index, 1);
    }

    #[test]
    fn test_expect_token_failure_wrong_token() {
        let tokens = vec![Token::Pipe];
        let mut index = 0;
        let result = expect_token(&tokens, &mut index, Token::Equals);
        assert!(result.is_err());
        assert_eq!(index, 0, "Index should not advance on failure");
    }

    #[test]
    fn test_expect_token_failure_out_of_bounds() {
        let tokens: Vec<Token> = vec![];
        let mut index = 0;
        let result = expect_token(&tokens, &mut index, Token::Equals);
        assert!(result.is_err());
        assert_eq!(index, 0);
    }

    #[test]
    fn test_token_list_not_mutated() {
        let original_tokens = vec![Token::Name("field".into()), Token::Colon];
        let tokens = original_tokens.clone();
        let mut index = 0;
        let _ = expect_name(&tokens, &mut index);
        assert_eq!(tokens, original_tokens, "Tokens must not be mutated");
    }

    #[test]
    fn test_multiple_expect_name_calls() {
        let tokens = vec![
            Token::Name("one".into()),
            Token::Name("two".into()),
            Token::Name("three".into()),
        ];
        let mut index = 0;

        let first = expect_name(&tokens, &mut index).unwrap();
        assert_eq!(first, "one");
        assert_eq!(index, 1);

        let second = expect_name(&tokens, &mut index).unwrap();
        assert_eq!(second, "two");
        assert_eq!(index, 2);

        let third = expect_name(&tokens, &mut index).unwrap();
        assert_eq!(third, "three");
        assert_eq!(index, 3);
    }

    #[test]
    fn test_expect_token_does_not_consume_non_matching_token() {
        let tokens = vec![Token::Colon];
        let mut index = 0;
        let _ = expect_token(&tokens, &mut index, Token::Equals);
        assert_eq!(index, 0, "Index should not move if token does not match");
    }
}
