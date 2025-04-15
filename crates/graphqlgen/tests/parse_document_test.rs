#[path = "../src/parsers/lexers.rs"]
mod lexer;

#[path = "../src/parsers/token.rs"]
mod token;

#[path = "../src/parsers/common.rs"]
mod common;

#[cfg(test)]
mod parse_document_tests {
    use graphqlgen_schema::ast::Definition;

    use crate::common::parse_document;
    use crate::lexer::Lexer;
    use crate::token::Token;

    #[test]
    fn parse_simple_type() {
        let input = r#"
            type User {
                id: ID!
                name: String
                friends: [User!]!
            }
        "#;

        let lexer: Lexer<'_> = Lexer::new(input);

        let tokens: Vec<_> = lexer
            .filter_map(|t| match t {
                Ok(tok) if tok != Token::EOF => Some(tok),
                _ => None,
            })
            .collect();

        let parsed = parse_document(tokens).expect("Failed to parse");

        assert_eq!(parsed.definitions.len(), 1);
        if let Definition::Type(t) = &parsed.definitions[0] {
            assert_eq!(t.name, "User");
            assert_eq!(t.fields.len(), 3);
            assert_eq!(t.fields[0].name, "id");
        }
    }
}
