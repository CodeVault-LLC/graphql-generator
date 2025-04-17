/*#[path = "../src/primary/common/lexers.rs"]
mod lexer;

#[path = "../src/primary/common/token.rs"]
mod token;

#[path = "../src/primary/parse.rs"]
mod parse;

#[cfg(test)]
mod parse_document_tests {
    use graphqlgen_schema::ast::Definition;

    use crate::lexer::Lexer;
    use crate::parse::parse_document;
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

    #[test]
    fn parse_directives() {
        let input = r#"
            type User @auth(requires: USER) {
                id: ID! @external
                name: String! @auth(requires: USER)
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
            assert_eq!(t.fields.len(), 2);
            assert_eq!(t.fields[0].name, "id");
            assert_eq!(t.fields[0].directives.as_ref().unwrap().len(), 1);
            assert_eq!(t.fields[0].directives.as_ref().unwrap()[0].name, "external");
        }
    }

    #[test]
    fn unimplemented_nested_directives() {
        let input = r#"
            type User @auth(requires: USER, permissions: { create: true, read: false }) {
                id: ID! @external
                name: String! @auth(requires: USER)
            }
        "#;

        let lexer: Lexer<'_> = Lexer::new(input);

        let tokens: Vec<_> = lexer
            .filter_map(|t| match t {
                Ok(tok) if tok != Token::EOF => Some(tok),
                _ => None,
            })
            .collect();

        assert!(
            parse_document(tokens).is_err(),
            "Expected error for nested directives"
        );
    }

    #[test]
    fn parse_complex_directives() {
        // @auth(requires: USER, permissions: [ { create: true }, { read: false } ])
        let input = r#"
            type User @auth(requires: USER, permissions: [ { create: true }, { read: false } ]) {
                id: ID! @external
                name: String! @auth(requires: USER)
            }
        "#;

        let lexer: Lexer<'_> = Lexer::new(input);

        let tokens: Vec<_> = lexer
            .filter_map(|t| match t {
                Ok(tok) if tok != Token::EOF => Some(tok),
                _ => None,
            })
            .collect();

        assert!(
            parse_document(tokens).is_err(),
            "Expected error for complex directive structure"
        );
    }

    #[test]
    fn parse_directive_default() {
        let input = r#"
            type User @auth(requires: { USER: ADMIN }) {
                id: ID! @external
                name: String! @auth(requires: USER)
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

        print!("Parsed: {:?}", parsed);

        assert_eq!(parsed.definitions.len(), 1);
    }

    #[test]
    fn parse_scalar() {
        let input = r#"
            scalar DateTime
            scalar Decimal @precision(scale: 2)
        "#;

        let lexer: Lexer<'_> = Lexer::new(input);

        let tokens: Vec<_> = lexer
            .filter_map(|t: Result<Token, String>| match t {
                Ok(tok) if tok != Token::EOF => Some(tok),
                _ => None,
            })
            .collect();

        let parsed = parse_document(tokens).expect("Failed to parse");

        assert_eq!(parsed.definitions.len(), 2);

        if let Definition::Scalar(s) = &parsed.definitions[0] {
            assert_eq!(s.name, "DateTime");
            assert!(s.directives.is_none());
        }

        if let Definition::Scalar(s) = &parsed.definitions[1] {
            assert_eq!(s.name, "Decimal");
            let directives = s.directives.as_ref().expect("Expected directives");
            assert_eq!(directives.len(), 1);
            assert_eq!(directives[0].name, "precision");
        }
    }
}
*/
