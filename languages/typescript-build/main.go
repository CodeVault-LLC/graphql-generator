package main

import (
	"fmt"
	"os"
	"strings"

	"github.com/codevault-llc/graphql-generator/config"
	"github.com/codevault-llc/graphql-generator/internal/schema/types"
)

func GenerateDefinitions(config config.InternalConfig, schema types.ExperimentalSchema) error {
	if _, err := os.Stat(config.Output.Path); os.IsNotExist(err) {
		err := os.MkdirAll(config.Output.Path, 0755)
		if err != nil {
			return err
		}
	}

	err := writeTypescriptDefinitions(config.Output.Path, &schema)
	if err != nil {
		return err
	}

	return nil
}

// Helper function to write TypeScript definitions to a file
func writeTypescriptDefinitions(outputDir string, schema *types.ExperimentalSchema) error {
	generator := NewGenerator(*schema)

	typescriptDefinitions := generateTypeScriptDefinitions(schema)
	generatedTanstackFunctions := generateTanstack(schema, generator)
	indexContent := generateIndexFile(schema)
	queries := generateQueries(generator)

	err := writeTextFile(outputDir, "index.ts", indexContent)
	if err != nil {
		return err
	}

	err = writeTextFile(outputDir, "queries.ts", queries)
	if err != nil {
		return err
	}

	err = writeTextFile(outputDir, "resources.ts", generateResources(generator))
	if err != nil {
		return err
	}

	err = writeTextFile(outputDir, "gpl.d.ts", typescriptDefinitions)
	if err != nil {
		return err
	}

	err = writeTextFile(outputDir, "gpl.ts", generatedTanstackFunctions)
	if err != nil {
		return err
	}

	return nil
}

func generateIndexFile(_ *types.ExperimentalSchema) string {
	var output strings.Builder

	output.WriteString("export * from './gpl';\n")
	output.WriteString("export * from './queries';\n")
	output.WriteString("export * from './resources';\n")

	return output.String()
}

// Helper function to write a text file
func writeTextFile(outputDir string, filename string, content string) error {
	file, err := os.Create(fmt.Sprintf("%s/%s", outputDir, filename))
	if err != nil {
		return err
	}
	defer file.Close()

	_, err = file.WriteString(content)
	if err != nil {
		return err
	}

	return nil
}

// Helper function to generate TypeScript definitions from a GraphQL schema
func generateTypeScriptDefinitions(schema *types.ExperimentalSchema) string {
	var output strings.Builder

	for _, field := range schema.Fields {
		if field.Type == types.ExperimentalSchemaFieldTypeType {
			for _, gqlType := range field.Types {
				if gqlType.Description != "" {
					output.WriteString(fmt.Sprintf("/* \n%s\n */\n", gqlType.Description))
				}
				output.WriteString(fmt.Sprintf("export interface %s {\n", gqlType.Name))
				for _, gqlField := range gqlType.Fields {
					tsType := mapGraphQLToTypeScript(gqlField.Type)
					output.WriteString(fmt.Sprintf("  %s: %s;\n", gqlField.Name, tsType))
				}
				output.WriteString("}\n\n")
			}
		} else if field.Type == types.ExperimentalSchemaFieldTypeEnum {
			for _, gqlEnum := range field.Enums {
				output.WriteString(fmt.Sprintf("/* \n%s\n */\n", gqlEnum.Description))
				output.WriteString(fmt.Sprintf("export enum %s {\n", gqlEnum.Name))
				for _, gqlEnumValue := range gqlEnum.Values {
					output.WriteString(fmt.Sprintf("  %s = \"%s\",\n", gqlEnumValue, gqlEnumValue))
				}
				output.WriteString("}\n\n")
			}
		}
	}

	return output.String()
}

func generateTanstack(schema *types.ExperimentalSchema, gen *Generator) string {
	var output strings.Builder

	output.WriteString("import { useQuery, useMutation } from '@tanstack/react-query';\n")
	output.WriteString("import { ")
	for _, field := range schema.Fields {
		if types.ExperimentalSchemaFieldType(field.Type) != types.ExperimentalSchemaFieldTypeType && types.ExperimentalSchemaFieldType(field.Type) != types.ExperimentalSchemaFieldTypeEnum {
			continue
		}

		for _, gqlType := range field.Types {
			if isBlacklistedType(gqlType.Name) {
				continue
			}

			output.WriteString(fmt.Sprintf("%s, ", gqlType.Name))
		}
	}
	output.WriteString("} from './gpl.d';\n\n")

	output.WriteString(gen.Run())

	return output.String()
}

func generateQueries(gen *Generator) string {
	var output strings.Builder

	for _, query := range gen.GetQuery() {
		output.WriteString(query)
	}

	return output.String()
}

func generateResources(gen *Generator) string {
	var output strings.Builder

	for _, resource := range gen.GetResources() {
		output.WriteString(resource)
	}

	return output.String()
}
