#[path = "../src/generator/apollo_queries.rs"]
mod apollo_queries;

#[cfg(test)]
mod apollo_queries_tests {
    use crate::apollo_queries::generate_apollo_queries;
    use graphqlgen_schema::ast::*;
    use std::fs;

    fn mock_typedef(name: &str, fields: &[&str]) -> TypeDef {
        TypeDef {
            name: name.into(),
            fields: fields
                .iter()
                .map(|f| Field {
                    name: f.to_string(),
                    field_type: TypeRef::Named("String".to_string()),
                })
                .collect(),
        }
    }

    #[test]
    fn generates_valid_query() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let path = tmp_dir.path().join("output.ts");

        let doc = Document {
            definitions: vec![Definition::Type(mock_typedef("User", &["id", "name"]))],
        };

        generate_apollo_queries(path.to_str().unwrap(), &doc).unwrap();

        let query_file = tmp_dir.path().join("queries/getUser.ts");
        let contents = fs::read_to_string(query_file).unwrap();

        assert!(contents.contains("query getUser"));
        assert!(contents.contains("id"));
        assert!(contents.contains("name"));
    }

    #[test]
    fn skips_empty_type() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let path = tmp_dir.path().join("output.ts");

        let doc = Document {
            definitions: vec![Definition::Type(mock_typedef("Empty", &[]))],
        };

        generate_apollo_queries(path.to_str().unwrap(), &doc).unwrap();

        let query_file = tmp_dir.path().join("queries/getEmpty.ts");
        assert!(!query_file.exists());
    }
}
