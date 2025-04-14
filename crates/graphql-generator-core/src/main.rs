use clap::{arg, command, Parser};
use clap_derive::Parser;
use config::config::{SchemaSource, CONFIG};
use graphql_parser::schema::parse_schema;
use serde::Serialize;
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

mod config;

#[derive(Debug, Parser)]
#[command(name = "graphqlgen")]
#[command(about = "GraphQLGen CLI")]
struct Cli {
    #[arg(short, long)]
    plugin: Option<String>,
}

#[derive(Serialize)]
struct ParsedSchema {
    source: String,
    definitions: Vec<String>,
}

fn main() {
    let args = Cli::parse();
    let schema_content: String;

    // Get plugin either from CLI or config
    let plugin = match args.plugin.or_else(|| Some(CONFIG.plugin.clone())) {
        Some(p) => p,
        None => {
            eprintln!("❌ Error: No plugin specified via CLI or config.");
            std::process::exit(1);
        }
    };

    let schema_source = match CONFIG.schema.as_str() {
        path if path.starts_with("http://") || path.starts_with("https://") => SchemaSource::Http(path.to_string()),
        path if path.ends_with(".graphql") || path.ends_with(".json") => SchemaSource::File(path.to_string()),
        _ => panic!("Unsupported schema format"),
    };

    // Read schema content
    match schema_source {
        SchemaSource::File(path) => {
            if path.ends_with(".graphql") {
                schema_content = fs::read_to_string(&path)
                    .unwrap_or_else(|_| panic!("Failed to read schema file: {}", path));
            } else if path.ends_with(".json") {
                println!("Using json_parser for .json file");
                return;
            } else {
                panic!("Unsupported schema file extension");
            }
        }

        SchemaSource::Http(url) => {
            println!("Using http_parser for HTTP request to {}", url);
            return;
        }
    }

    let ast = parse_schema::<String>(&schema_content).expect("Failed to parse schema");

    let parsed_schema = ParsedSchema {
        source: schema_content.to_string(),
        definitions: ast.definitions.iter().map(|d| format!("{:?}", d)).collect(),
    };

    let json = serde_json::to_string(&parsed_schema).expect("Failed to serialize schema");

    // Use resolved plugin
    let plugin_bin = format!("graphqlgen-{}", plugin);

    let mut child = Command::new(&plugin_bin)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn plugin");

    if let Some(mut stdin) = child.stdin.take() {
        writeln!(stdin, "{}", CONFIG.output.clone()).expect("Failed to write to stdin");
        stdin.flush().expect("Failed to flush stdin");

        writeln!(stdin, "{}", json).expect("Failed to write to stdin");
        stdin.flush().expect("Failed to flush stdin");
    }

    let status = child.wait().expect("Plugin failed");

    if status.success() {
        println!("✅ Plugin executed successfully");
    } else {
        eprintln!("❌ Plugin failed");
    }
}
