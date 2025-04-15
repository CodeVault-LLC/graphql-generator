use serde::Deserialize;
use std::fs;
use std::io::{self, BufRead, Read};

use graphqlgen_schema::ast::Document;

mod generator;

#[derive(Deserialize)]
struct ParsedInput {
    source: String,
    ast: Document,
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut output_path = String::new();
    handle.read_line(&mut output_path).unwrap();
    let output_path = output_path.trim();

    let mut json_input = String::new();
    handle.read_to_string(&mut json_input).unwrap();

    let ast: Document = serde_json::from_str(&json_input).expect("Invalid AST JSON");

    let schema = ParsedInput {
        source: output_path.to_string(),
        ast,
    };

    let output_dir = std::path::Path::new(output_path)
        .parent()
        .expect("Failed to get parent directory");

    fs::create_dir_all(output_dir).unwrap_or_else(|e| panic!("Failed to create output dir: {e}"));

    generator::apollo_config::generate_apollo_config(schema.source.clone())
        .expect("Failed to generate Apollo config");

    generator::apollo_queries::generate_apollo_queries(&schema.source, &schema.ast)
        .expect("Failed to generate Apollo queries");
}
