use clap::Parser;
use graphql_parser::schema::{parse_schema, Document};
use serde::Serialize;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

#[derive(Parser, Debug)]
#[command(name = "graphqlgen")]
#[command(about = "GraphQLGen CLI")]
struct Cli {
    /// Path to GraphQL schema file
    #[arg(short, long)]
    input: String,

    /// Output directory
    #[arg(short, long)]
    output: String,

    /// Plugin to run (e.g., apollo)
    #[arg(short, long)]
    plugin: String,
}

#[derive(Serialize)]
struct ParsedSchema {
    // You can expand this later
    source: String,
    definitions: Vec<String>,
}

fn main() {
    let args = Cli::parse();

    let schema_content = fs::read_to_string(&args.input)
        .expect("Failed to read schema file");

    let ast = parse_schema(&schema_content)
        .expect("Failed to parse schema");

    let parsed_schema = ParsedSchema {
        source: args.input.clone(),
        definitions: ast.definitions.iter().map(|d| format!("{:?}", d)).collect(),
    };

    let json = serde_json::to_string(&parsed_schema)
        .expect("Failed to serialize schema");

    // Spawn plugin and send it the schema JSON
    let plugin_bin = format!("graphqlgen-{}", args.plugin);

    let mut child = Command::new(&plugin_bin)
        .arg("--output")
        .arg(&args.output)
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to spawn plugin");

    child.stdin
        .as_mut()
        .unwrap()
        .write_all(json.as_bytes())
        .expect("Failed to write to plugin stdin");

    let status = child.wait().expect("Plugin failed");

    if status.success() {
        println!("✅ Plugin executed successfully");
    } else {
        eprintln!("❌ Plugin failed");
    }
}
