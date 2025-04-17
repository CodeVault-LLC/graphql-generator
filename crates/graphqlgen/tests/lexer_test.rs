/*#[path = "../src/primary/common/lexers.rs"]
mod lexer;

#[path = "../src/primary/common/token.rs"]
mod token;

#[cfg(test)]
mod lexer_tests {
    use crate::lexer::Lexer;
    use crate::token::Token;

    fn collect_tokens(input: &str) -> Vec<Token> {
        let mut lexer = Lexer::new(input);
        let mut tokens = vec![];

        loop {
            match lexer.next_token() {
                Ok(Token::EOF) => break,
                Ok(tok) => tokens.push(tok),
                Err(e) => panic!("Lexer error: {}", e),
            }
        }

        tokens
    }

    #[test]
    fn test_basic_tokenization() {
        let tokens = collect_tokens(
            r#"
            type User {
                id: ID!
            }
        "#,
        );

        assert!(tokens.contains(&Token::Name("type".to_string())));
        assert!(tokens.contains(&Token::Name("User".to_string())));
        assert!(tokens.contains(&Token::Name("id".to_string())));
        assert!(tokens.contains(&Token::Name("ID".to_string())));
        assert!(tokens.contains(&Token::Bang));
        assert!(tokens.contains(&Token::BraceOpen));
        assert!(tokens.contains(&Token::BraceClose));
    }

    #[test]
    fn test_string_tokenization() {
        let tokens = collect_tokens(
            r#"
            type Post {
                title: String
                content: String
                summary: String = "default summary"
            }
        "#,
        );

        assert!(tokens.contains(&Token::String("default summary".to_string())));
    }

    #[test]
    fn test_number_tokenization() {
        let tokens = collect_tokens("scalar Custom @value(1234)");

        assert!(tokens.contains(&Token::Int(1234)));
    }

    #[test]
    fn test_float_tokenization() {
        let tokens = collect_tokens("value: Float = 3.1415");

        assert!(tokens.contains(&Token::Float(3.1415)));
    }

    #[test]
    fn test_boolean_tokenization() {
        let tokens = collect_tokens("flag: Boolean = true");

        assert!(tokens.contains(&Token::Boolean(true)));
    }

    #[test]
    fn test_null_tokenization() {
        let tokens = collect_tokens("field: String = null");

        assert!(tokens.contains(&Token::Null));
    }

    #[test]
    fn test_directives_and_symbols() {
        let tokens = collect_tokens(
            r#"
            type User @entity {
                id: ID!
                email: String @unique
                name: String @deprecated(reason: "Use 'fullName' instead")
            }
        "#,
        );

        assert!(tokens.contains(&Token::At));
        assert!(tokens.contains(&Token::Name("entity".to_string())));
        assert!(tokens.contains(&Token::Name("unique".to_string())));
        assert!(tokens.contains(&Token::Name("deprecated".to_string())));
        assert!(tokens.contains(&Token::Name("reason".to_string())));
    }

    #[test]
    fn test_list_and_non_null_combination() {
        let tokens = collect_tokens("tags: [String!]!");

        assert!(tokens.iter().filter(|t| **t == Token::BracketOpen).count() == 1);
        assert!(tokens.iter().filter(|t| **t == Token::BracketClose).count() == 1);
        assert_eq!(tokens.iter().filter(|t| **t == Token::Bang).count(), 2);
    }

    #[test]
    fn test_ellipsis_tokenization() {
        let tokens = collect_tokens("... on User");

        assert!(tokens.contains(&Token::Ellipsis));
    }

    #[test]
    fn test_dollar_for_variables() {
        let tokens = collect_tokens("query ($id: ID!)");

        assert!(tokens.contains(&Token::Dollar));
    }

    #[test]
    #[should_panic(expected = "Lexer error")]
    fn test_unterminated_string_panics() {
        let _ = collect_tokens(r#"type Broken { name: String = "oops }"#);
    }

    #[test]
    #[should_panic(expected = "Lexer error")]
    fn test_unexpected_dot_panics() {
        let _ = collect_tokens("...on.User");
    }

    #[test]
    #[should_panic(expected = "Lexer error")]
    fn test_illegal_character_panics() {
        let _ = collect_tokens("type User { id: ID § }");
    }
}
*/
