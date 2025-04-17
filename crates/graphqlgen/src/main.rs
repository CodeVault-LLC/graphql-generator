use clap::{arg, command, Parser};
use clap_derive::Parser;

use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::process::{Command, Stdio};
use std::time::Instant;

use log::{error, info};

mod config;
use config::config::CONFIG;

use graphqlgen::core::parse::{generate_tokens, parse_document};

#[derive(Debug, Parser)]
#[command(name = "graphqlgen")]
#[command(about = "GraphQLGen CLI")]
struct Cli {
    #[arg(short, long)]
    plugin: Option<String>,
}

fn main() {
    if !std::path::Path::new("config/log4rs.yaml").exists() {
        let stdout = log4rs::append::console::ConsoleAppender::builder().build();

        let settings = log4rs::Config::builder()
            .appender(log4rs::config::Appender::builder().build("stdout", Box::new(stdout)))
            .build(
                log4rs::config::Root::builder()
                    .appender("stdout")
                    .build(log::LevelFilter::Info),
            )
            .unwrap();

        log4rs::init_config(settings).unwrap();
    } else {
        log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    }

    let start = Instant::now();
    let args = Cli::parse();

    let plugin = args.plugin.as_deref().unwrap_or(&CONFIG.plugin);
    if plugin.is_empty() {
        error!("Error: No plugin specified via CLI or config.");
        std::process::exit(1);
    }

    let schema_content = match CONFIG.schema.as_str() {
        path if path.starts_with("http://") || path.starts_with("https://") => {
            info!("Using http_parser for remote schema");
            return;
        }
        path if path.ends_with(".graphql") => match fs::read_to_string(path) {
            Ok(content) => {
                info!("Using graphql_parser for GraphQL schema");
                content
            }
            Err(e) => {
                error!("Error: Failed to read schema file '{}': {}", path, e);
                std::process::exit(1);
            }
        },
        path if path.ends_with(".json") => {
            info!("Using json_parser for JSON schema");
            return;
        }
        _ => {
            error!("Error: Unsupported schema format. Please provide a .graphql or .json file.");
            std::process::exit(1);
        }
    };

    let tokens = match generate_tokens(&schema_content) {
        Ok(tokens) => tokens,
        Err(e) => {
            error!("Error: Failed to generate tokens: {}", e);
            std::process::exit(1);
        }
    };

    {
        let file: File = File::create("tokens.txt").expect("Failed to create tokens.txt");
        let mut writer: BufWriter<File> = BufWriter::new(file);
        for token in &tokens {
            writeln!(writer, "{:?}", token).expect("Write failed");
        }
    }

    let parsed_schema = parse_document(tokens).expect("Failed to parse schema");

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
        info!("Plugin executed successfully in {:.2?}", start.elapsed());
    } else {
        error!("Plugin execution failed with status: {}", status);
        std::process::exit(1);
    }
}
