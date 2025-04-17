use crate::core::common::parse::expect::{expect_name, expect_token};

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

#[cfg(test)]
mod tests {
    use super::*;
    use graphqlgen_schema::ast::{Definition, TypeRef, UnionDef};

    #[test]
    fn test_parse_union_single_member() {
        let tokens = vec![
            Token::Name("union".to_string()),
            Token::Name("SearchResult".to_string()),
            Token::Equals,
            Token::Name("Photo".to_string()),
        ];

        let mut index = 0;
        let result = parse_union(&tokens, &mut index);
        assert!(result.is_ok());

        if let Definition::Union(UnionDef {
            name,
            members,
            directives,
        }) = result.unwrap()
        {
            assert_eq!(name, "SearchResult");
            assert_eq!(members.len(), 1);
            assert!(directives.is_none());
        } else {
            panic!("Expected UnionDef");
        }
    }

    #[test]
    fn test_parse_union_multiple_members() {
        let tokens = vec![
            Token::Name("union".to_string()),
            Token::Name("SearchResult".to_string()),
            Token::Equals,
            Token::Name("Photo".to_string()),
            Token::Pipe,
            Token::Name("User".to_string()),
            Token::Pipe,
            Token::Name("Comment".to_string()),
        ];

        let mut index = 0;
        let result = parse_union(&tokens, &mut index);
        assert!(result.is_ok());

        if let Definition::Union(UnionDef { name, members, .. }) = result.unwrap() {
            assert_eq!(name, "SearchResult");
            assert!(members.len() == 3);
            assert!(matches!(members[0], TypeRef::Named(ref name) if name == "Photo"));
            assert!(matches!(members[1], TypeRef::Named(ref name) if name == "User"));
            assert!(matches!(members[2], TypeRef::Named(ref name) if name == "Comment"));
        }
    }

    #[test]
    fn test_parse_union_pipe_at_start() {
        let tokens = vec![
            Token::Name("union".to_string()),
            Token::Name("Result".to_string()),
            Token::Equals,
            Token::Pipe,
            Token::Name("A".to_string()),
            Token::Pipe,
            Token::Name("B".to_string()),
        ];

        let mut index = 0;
        let result = parse_union(&tokens, &mut index);
        assert!(result.is_ok());

        if let Definition::Union(UnionDef { name, members, .. }) = result.unwrap() {
            assert_eq!(name, "Result");
            assert_eq!(members.len(), 2);
            assert!(matches!(members[0], TypeRef::Named(ref name) if name == "A"));
            assert!(matches!(members[1], TypeRef::Named(ref name) if name == "B"));
        }
    }

    #[test]
    fn test_parse_union_missing_name() {
        let tokens = vec![
            Token::Name("union".to_string()),
            Token::Equals,
            Token::Name("Something".to_string()),
        ];

        let mut index = 0;
        let result = parse_union(&tokens, &mut index);
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("Expected name"));
    }

    #[test]
    fn test_parse_union_missing_equals() {
        let tokens = vec![
            Token::Name("union".to_string()),
            Token::Name("Result".to_string()),
            Token::Name("Photo".to_string()),
        ];

        let mut index = 0;
        let result = parse_union(&tokens, &mut index);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Expected Equals, got Some(Name(\"Photo\"))"));
    }

    #[test]
    fn test_parse_union_no_members() {
        let tokens = vec![
            Token::Name("union".to_string()),
            Token::Name("EmptyUnion".to_string()),
            Token::Equals,
        ];

        let mut index = 0;
        let result = parse_union(&tokens, &mut index);
        assert!(result.is_ok());

        if let Definition::Union(UnionDef { members, .. }) = result.unwrap() {
            assert!(
                members.is_empty(),
                "Expected empty members, got {:?}",
                members
            );
        }
    }

    #[test]
    fn test_parse_union_invalid_token_in_members() {
        let tokens = vec![
            Token::Name("union".to_string()),
            Token::Name("Weird".to_string()),
            Token::Equals,
            Token::Colon, // Invalid token here
            Token::Name("X".to_string()),
        ];

        let mut index = 0;
        let result = parse_union(&tokens, &mut index);
        assert!(result.is_ok());

        if let Definition::Union(UnionDef { members, .. }) = result.unwrap() {
            assert!(
                members.is_empty(),
                "Unexpectedly parsed members: {:?}",
                members
            );
            assert_eq!(index, 3); // parsing should stop at unexpected token
        }
    }

    #[test]
    fn test_parse_union_trailing_pipe() {
        let tokens = vec![
            Token::Name("union".to_string()),
            Token::Name("Broken".to_string()),
            Token::Equals,
            Token::Name("X".to_string()),
            Token::Pipe,
        ];

        let mut index = 0;
        let result = parse_union(&tokens, &mut index);
        assert!(result.is_ok());

        if let Definition::Union(UnionDef { members, .. }) = result.unwrap() {
            assert_eq!(members.len(), 1); // Only X should be collected
        }
    }
}
