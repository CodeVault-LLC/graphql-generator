use clap::{arg, command, Parser};
use clap_derive::Parser;

use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::process::{Command, Stdio};
use std::time::Instant;

mod config;
mod parsers;

use config::config::CONFIG;

use crate::parsers::common::parse_document;
use crate::parsers::lexers::Lexer;
use crate::parsers::token::Token;

#[derive(Debug, Parser)]
#[command(name = "graphqlgen")]
#[command(about = "GraphQLGen CLI")]
struct Cli {
    #[arg(short, long)]
    plugin: Option<String>,
}

fn main() {
    let start = Instant::now();
    let args = Cli::parse();

    let plugin = args.plugin.as_deref().unwrap_or(&CONFIG.plugin);
    if plugin.is_empty() {
        eprintln!("❌ Error: No plugin specified via CLI or config.");
        std::process::exit(1);
    }

    let schema_content = match CONFIG.schema.as_str() {
        path if path.starts_with("http://") || path.starts_with("https://") => {
            println!("Using http_parser for HTTP request to {}", path);
            return;
        }
        path if path.ends_with(".graphql") => {
            fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to read: {}", path))
        }
        path if path.ends_with(".json") => {
            println!("Using json_parser for .json file");
            return;
        }
        _ => panic!("Unsupported schema format"),
    };

    let lexer: Lexer<'_> = Lexer::new(&schema_content);
    let tokens: Vec<_> = lexer
        .filter_map(|t: Result<Token, String>| match t {
            Ok(t) if t != Token::EOF => Some(t),
            Ok(_) => None,
            Err(e) => {
                eprintln!("Lexer error: {}", e);
                std::process::exit(1);
            }
        })
        .collect();

    let parsed_schema = parse_document(tokens).expect("Failed to parse schema");

    println!("Parsed schema: {:#?}", parsed_schema);

    {
        let file: File = File::create("test.txt").expect("Failed to create test.txt");
        let mut writer: BufWriter<File> = BufWriter::new(file);
        writeln!(writer, "{:#?}", parsed_schema).expect("Write failed");
    }

    let plugin_bin = format!("graphqlgen_{}", plugin);

    let mut child = Command::new(&plugin_bin)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn plugin");

    if let Some(mut stdin) = child.stdin.take() {
        let output_path = std::env::current_dir()
            .expect("Failed to get current directory")
            .join(&CONFIG.output);

        writeln!(stdin, "{}", output_path.display()).expect("Failed to write output path");

        let json = serde_json::to_string(&parsed_schema).expect("Failed to serialize schema");
        writeln!(stdin, "{}", json).expect("Failed to write AST");
    }

    let status = child.wait().expect("Plugin execution failed");

    if status.success() {
        println!("✅ Plugin executed successfully in {:.2?}", start.elapsed());
    } else {
        eprintln!("❌ Plugin failed");
    }
}
