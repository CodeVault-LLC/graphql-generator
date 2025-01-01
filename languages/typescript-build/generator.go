package main

import (
	"fmt"
	"strings"

	"github.com/codevault-llc/graphql-generator/internal/schema/types"
)

type Generator struct {
	schema    types.ExperimentalSchema
	query     []string
	resources []string
	imports   ImportsSlice
}

func NewGenerator(schema types.ExperimentalSchema) *Generator {
	return &Generator{
		schema: schema,
	}
}

func (g *Generator) Run() GeneratorResults {
	var output strings.Builder

	var query []string

	var resources []string

	var imports ImportsSlice

	for _, field := range g.schema.Fields {
		if types.ExperimentalSchemaFieldType(field.Name) == types.ExperimentalSchemaFieldTypeQuery {
			queryData := g.turnQueryIntoTanstackQuery(field)

			output.WriteString(queryData.Hooks)
			query = append(query, queryData.QueryDefinition)
			resources = append(resources, queryData.RequestFunction)
			imports.Connect(queryData.Imports...)
		}

		if types.ExperimentalSchemaFieldType(field.Name) == types.ExperimentalSchemaFieldTypeMutation {
			mutationData := g.turnMutationIntoTanstackMutation(field)

			output.WriteString(mutationData.Hooks)
			query = append(query, mutationData.QueryDefinition)
			resources = append(resources, mutationData.RequestFunction)
			imports.Connect(mutationData.Imports...)
		}
	}

	g.query = query
	g.resources = resources
	g.imports = imports

	return GeneratorResults{
		IndexContent: generateIndexFile(&g.schema),
		Queries:      generateQueries(g),
		Resources:    generateResources(g),
		TypeScript:   generateTypeScriptDefinitions(g),
		Tanstack:     generateTanstack(output.String(), g),
	}
}

func (g *Generator) GetQuery() []string {
	return g.query
}

func (g *Generator) GetResources() []string {
	return g.resources
}

func (g *Generator) GetImports() ImportsSlice {
	return g.imports
}

func (g *Generator) buildArguments(query types.Field) GeneratorBuildArgumentsResult {
	args := ""
	argumentChecks := ""
	argumentUsage := ""
	argumentReplacements := ""
	argumentTypes := ""
	specialType := []string{}
	enums := []string{}

	if len(query.Arguments) > 0 {
		for _, arg := range query.Arguments {
			if isEnumType(arg.Type) {
				enums = append(enums, arg.Name)
			}
		}

		argumentReplacements += "const isEnumField = (field: string): boolean => {\n return [\n"
		for _, enum := range enums {
			argumentReplacements += fmt.Sprintf("'%s',\n", enum)
		}

		argumentReplacements += "].includes(field);\n};\n"

		args = "args: { "
		argumentUsage = "("
		argumentTypes = "{ "

		for ilen, arg := range query.Arguments {
			if mapGraphQLToTypeScript(arg.Type).Nullable {
				args += fmt.Sprintf("%s: %s | null", arg.Name, mapGraphQLToTypeScript(arg.Type).Value)
			} else {
				args += fmt.Sprintf("%s: %s", arg.Name, mapGraphQLToTypeScript(arg.Type).Value)
			}

			if isSpecialType(arg.Type) != "" {
				specialType = append(specialType, isSpecialType(arg.Type))
			}

			argumentChecks += fmt.Sprintf("if (!args.%s) throw new Error('%s is required.');\n", arg.Name, arg.Name)

			if isSpecialType(arg.Type) == "" {
				argumentUsage += fmt.Sprintf("%s: {{args.%s}}", arg.Name, arg.Name)
				argumentReplacements += fmt.Sprintf("query = query.replace('{{args.%s}}', `\"${args.%s}\"`);\n", arg.Name, arg.Name)
			} else if isEnumType(arg.Type) {
				argumentUsage += fmt.Sprintf("%s: {{args.%s}}", arg.Name, arg.Name)
				argumentReplacements += fmt.Sprintf("query = query.replace('{{args.%s}}', `${args.%s}`);\n", arg.Name, arg.Name)
			} else {
				argumentUsage += fmt.Sprintf("%s: { {{args.%s}} }", arg.Name, arg.Name)
				argumentReplacements += fmt.Sprintf("const %sFields = Object.entries(args.%s)\n  .map(([key, value]) => {\n    if (isEnumField(key)) {\n      return `${key}: ${value}`;\n    }\n    return `${key}: ${JSON.stringify(value)}`;\n  })\n  .join(\", \");", arg.Name, arg.Name)
				argumentReplacements += fmt.Sprintf("query = query.replace(\"{{args.data}}\", %sFields);\n", arg.Name)
			}

			if mapGraphQLToTypeScript(arg.Type).Nullable {
				argumentTypes += fmt.Sprintf("%s: %s | null", arg.Name, mapGraphQLToTypeScript(arg.Type).Value)
			} else {
				argumentTypes += fmt.Sprintf("%s: %s", arg.Name, mapGraphQLToTypeScript(arg.Type).Value)
			}

			if ilen < len(query.Arguments)-1 {
				args += ", "
				argumentUsage += ", "
				argumentTypes += ", "
			}
		}

		args += " }"
		argumentUsage += ")"
		argumentTypes += " }"
	}

	return GeneratorBuildArgumentsResult{
		Arguments:            args,
		ArgumentChecks:       argumentChecks,
		ArgumentUsage:        argumentUsage,
		ArgumentReplacements: argumentReplacements,
		ArgumentTypes:        argumentTypes,
		SpecialArguments:     specialType,
	}
}
