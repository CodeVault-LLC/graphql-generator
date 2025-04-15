use std::env::temp_dir;
use std::process::{Command, Stdio};
use std::fs;
use std::io::Write;

const SAMPLE_SCHEMA: &str = r#"
    type Query {
        hello: String
    }
"#;

#[test]
fn test_generated_apollo_typescript_output() {
    let temp_dir = temp_dir();

    let parsed_schema = serde_json::json!({
        "source": SAMPLE_SCHEMA,
        "definitions": [
            "type Query { hello: String }"
        ]
    });

    let json_payload = serde_json::to_string(&parsed_schema).unwrap();

    let generator_path = format!(
      "{}/../../target/debug/graphqlgen-apollo",
      env!("CARGO_MANIFEST_DIR")
    );

    let mut child = Command::new(generator_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to start generator");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");

        writeln!(stdin, "{}", temp_dir.to_string_lossy()).unwrap(); // First line: output path
        stdin.write_all(json_payload.as_bytes()).unwrap(); // Then: JSON payload
    }

    let status = child.wait().expect("Failed to wait on child");
    assert!(status.success(), "Generator process failed");

    assert!(temp_dir.exists(), "Expected output file was not created");

    const EXPECTED_FILES: [&str; 2] = ["generated_apollo.ts", "apollo-client.ts"];
    for file in EXPECTED_FILES.iter() {
        let file_path = temp_dir.join(file);
        assert!(file_path.exists(), "Expected file {} was not created", file);

        let content = fs::read_to_string(&file_path).expect("Failed to read file");
        assert!(!content.is_empty(), "File {} is empty", file);
    }
}
