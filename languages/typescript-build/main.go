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
	result := generator.Run()

	err := writeTextFile(outputDir, "index.ts", result.IndexContent)
	if err != nil {
		return err
	}

	err = writeTextFile(outputDir, "queries.ts", result.Queries)
	if err != nil {
		return err
	}

	err = writeTextFile(outputDir, "resources.ts", result.Resources)
	if err != nil {
		return err
	}

	err = writeTextFile(outputDir, "gpl.d.ts", result.TypeScript)
	if err != nil {
		return err
	}

	err = writeTextFile(outputDir, "gpl.ts", result.Tanstack)
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
func generateTypeScriptDefinitions(generator *Generator) string {
	var output strings.Builder
	schema := generator.schema

	for _, field := range schema.Fields {
		if field.Type == types.ExperimentalSchemaFieldTypeType || field.Type == types.ExperimentalSchemaFieldTypeInput {
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

func generateTanstack(data string, gen *Generator) string {
	var output strings.Builder

	imports := gen.GetImports().GetImportsFromLocation(ImportLocationHooks)
	output.WriteString(imports.ToImport())

	output.WriteString(data)

	return output.String()
}

func generateQueries(gen *Generator) string {
	var output strings.Builder

	imports := gen.GetImports().GetImportsFromLocation(ImportLocationQueries)
	output.WriteString(imports.ToImport())

	for _, query := range gen.GetQuery() {
		output.WriteString(query)
	}

	return output.String()
}

func generateResources(gen *Generator) string {
	var output strings.Builder

	imports := gen.GetImports().GetImportsFromLocation(ImportLocationResources)
	output.WriteString(imports.ToImport())

	for _, resource := range gen.GetResources() {
		output.WriteString(resource)
	}

	return output.String()
}
