package config

import (
	"encoding/json"
	"errors"
	"io"
	"os"
)

type Filenames struct {
	Types string `json:"types"`
	Main  string `json:"main"`
}

type OutputOptions struct {
	Path      string    `json:"path"`
	Filenames Filenames `json:"filenames"`
}

type LanguageConfig struct {
	Name       string `json:"name"`
	Entrypoint string `json:"entrypoint"`
}

type InternalConfig struct {
	Schema    string           `json:"schema"`
	Output    OutputOptions    `json:"output"`
	Language  string           `json:"language"`
	Languages []LanguageConfig `json:"languages"`
}

// NewConfig creates a new configuration instance with defaults or from a file.
func NewConfig() *InternalConfig {
	config, err := OpenConfig("graphql-generator.conf.json")
	if err != nil {
		// Provide default values if the config file cannot be read or parsed
		return &InternalConfig{
			Schema: "schema.json",
			Output: OutputOptions{
				Path: "output/gpl",
				Filenames: Filenames{
					Types: "gpl.d.ts",
					Main:  "gpl.ts",
				},
			},
			Language:  "typescript",
			Languages: []LanguageConfig{},
		}
	}

	return config
}

// OpenConfig reads and parses the configuration from a JSON file.
func OpenConfig(path string) (*InternalConfig, error) {
	file, err := os.Open(path)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	fileContent, err := io.ReadAll(file)
	if err != nil {
		return nil, err
	}

	if len(fileContent) == 0 {
		return nil, errors.New("config file is empty")
	}

	config := &InternalConfig{}
	err = json.Unmarshal(fileContent, config)
	if err != nil {
		return nil, err
	}

	return config, nil
}
