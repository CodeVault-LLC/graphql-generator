const APOLLO_CONFIG_STRING: &str = r#"// Auto-generated by graphqlgen-apollo
// This is a temp file for the Apollo client initialization. Change this file or create a new one.
import { ApolloClient, InMemoryCache } from '@apollo/client';

const client = new ApolloClient({
    uri: 'https://your-graphql-endpoint.com/graphql',
    cache: new InMemoryCache(),
});

export { client };

/*
Example usage in a React app:

import React from 'react';
import * as ReactDOM from 'react-dom/client';
import { ApolloProvider } from '@apollo/client';
import App from './App';

import { client } from './apollo-client'; // Adjust the path as needed

// Supported in React 18+
const root = ReactDOM.createRoot(document.getElementById('root'));

root.render(
  <ApolloProvider client={client}>
    <App />
  </ApolloProvider>,
);
*/"#;

pub fn generate_apollo_config(
    output: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = std::path::Path::new(output)
        .parent()
        .expect("Failed to get parent directory")
        .to_path_buf();

    std::fs::create_dir_all(&output_dir).expect("Failed to create output dir");

    let file_path = output_dir.join("apollo-client.ts");

    std::fs::write(&file_path, APOLLO_CONFIG_STRING).expect("Failed to write output file");

    Ok(())
}
