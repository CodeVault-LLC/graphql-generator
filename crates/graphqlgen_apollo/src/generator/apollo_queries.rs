use graphqlgen_schema::ast::{Definition, Document, Field, TypeDef, TypeRef};
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn generate_apollo_queries(output_path: &str, ast: &Document) -> Result<(), String> {
    let file_path = Path::new(output_path).join("queries.ts");

    let mut file =
        File::create(&file_path).map_err(|e| format!("Failed to create queries.ts file: {}", e))?;

    file.write_all(b"import { gql } from '@apollo/client';\n\n")
        .map_err(|e| format!("Failed to write to file: {}", e))?;

    let mut gql_vars: Vec<String> = Vec::new();

    for def in &ast.definitions {
        if let Definition::Type(TypeDef {
            name,
            fields,
            directives,
        }) = def
        {
            if name == "Query" || name == "Mutation" {
                for field in fields {
                    let gql_var_name: String =
                        format!("{}_{}", name.to_lowercase(), field.name).to_uppercase();

                    let operation_type: String = name.to_lowercase(); // query or mutation
                    let (params_str, param_values) = render_params(&field);

                    let query_string: String = format!(
                        "const {} = gql`\n  {} {}{} {{\n    {}{} {{\n      {}\n    }}\n  }}\n`;\n\n",
                        gql_var_name,
                        operation_type,
                        field.name,
                        params_str,
                        name,
                        param_values,
                        "id" // Default field output (you can later infer this better)
                    );

                    file.write_all(query_string.as_bytes())
                        .map_err(|e| format!("Failed to write {}: {}", gql_var_name, e))?;

                    gql_vars.push(gql_var_name);
                }
            }
        }
    }

    // Export all constants
    let export_string = format!("export {{ {} }};\n", gql_vars.join(", "));
    file.write_all(export_string.as_bytes())
        .map_err(|e| format!("Failed to write export block: {}", e))?;

    Ok(())
}

fn render_params(field: &Field) -> (String, String) {
    if let TypeRef::NonNull(_) | TypeRef::Named(_) | TypeRef::List(_) = &field.field_type {
        // If there are no args (should be handled above), return empty
    }

    // Let's assume a simple argument list for now based on field.name
    // For example: Tweet(id: ID!) or Tweets(limit: Int, skip: Int)
    let mut var_declarations = vec![];
    let mut var_usages = vec![];

    if let Some(args) = &field.arguments {
        for arg in args {
            let var_type = match &arg.value_type {
                TypeRef::NonNull(inner) => format!("{}!", type_ref_to_str(inner)),
                other => type_ref_to_str(other),
            };

            var_declarations.push(format!("${}: {}", arg.name, var_type));
            var_usages.push(format!("{}: ${}", arg.name, arg.name));
        }
    }

    if var_declarations.is_empty() {
        ("".to_string(), "".to_string())
    } else {
        (
            format!("({})", var_declarations.join(", ")),
            format!("({})", var_usages.join(", ")),
        )
    }
}

fn type_ref_to_str(t: &TypeRef) -> String {
    match t {
        TypeRef::Named(name) => name.clone(),
        TypeRef::NonNull(inner) => format!("{}!", type_ref_to_str(inner)),
        TypeRef::List(inner) => format!("[{}]", type_ref_to_str(inner)),
    }
}
