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
}

func NewGenerator(schema types.ExperimentalSchema) *Generator {
	return &Generator{
		schema: schema,
	}
}

func (g *Generator) Run() string {
	var output strings.Builder
	var query []string
	var resources []string

	for _, field := range g.schema.Fields {
		if types.ExperimentalSchemaFieldType(field.Name) == types.ExperimentalSchemaFieldTypeQuery {
			tanstackContent, queryData, resourceData := g.turnQueryIntoTanstackQuery(field)

			output.WriteString(tanstackContent)
			query = append(query, queryData)
			resources = append(resources, resourceData)
		}

		if types.ExperimentalSchemaFieldType(field.Name) == types.ExperimentalSchemaFieldTypeMutation {
			tanstackContent, queryData, resourceData := g.turnMutationIntoTanstackMutation(field)

			output.WriteString(tanstackContent)
			query = append(query, queryData)
			resources = append(resources, resourceData)
		}
	}

	g.query = query
	g.resources = resources

	return output.String()
}

func (g *Generator) GetQuery() []string {
	return g.query
}

func (g *Generator) GetResources() []string {
	return g.resources
}

func (g *Generator) buildArguments(query types.Field) GeneratorBuildArgumentsResult {
	args := ""
	argumentChecks := ""
	argumentUsage := ""
	argumentReplacements := ""

	if len(query.Arguments) > 0 {
		args = "args: { "
		argumentUsage = "("
		for i, arg := range query.Arguments {
			args += fmt.Sprintf("%s: %s", arg.Name, mapGraphQLToTypeScript(arg.Type))
			argumentChecks += fmt.Sprintf("if (!args.%s) throw new Error('%s is required.');\n", arg.Name, arg.Name)
			argumentUsage += fmt.Sprintf("%s: \"{{args.%s}}\"", arg.Name, arg.Name)
			argumentReplacements += fmt.Sprintf("query = query.replace('{{args.%s}}', args.%s);\n", arg.Name, arg.Name)

			if i < len(query.Arguments)-1 {
				args += ", "
				argumentUsage += ", "
			}
		}
		args += " }"
		argumentUsage += ")"
	}

	return GeneratorBuildArgumentsResult{
		Arguments:            args,
		ArgumentChecks:       argumentChecks,
		ArgumentUsage:        argumentUsage,
		ArgumentReplacements: argumentReplacements,
	}
}
