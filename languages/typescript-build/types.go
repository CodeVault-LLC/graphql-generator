package main

import (
	"fmt"

	"github.com/codevault-llc/graphql-generator/internal/schema/types"
)

// Helper function to map GraphQL types to TypeScript types
func mapGraphQLToTypeScript(typeRef types.TypeReference) string {
	switch typeRef.Kind {
	case "SCALAR":
		switch *typeRef.Name {
		case "String":
			return "string"
		case "ID":
			return "string"
		case "Int", "Float":
			return "number"
		case "Boolean":
			return "boolean"
		default:
			return "any"
		}
	case "OBJECT":
		return *typeRef.Name
	case "LIST":
		return fmt.Sprintf("%s[]", mapGraphQLToTypeScript(*typeRef.OfType))
	case "NON_NULL":
		return mapGraphQLToTypeScript(*typeRef.OfType)
	default:
		return "any"
	}
}

type FunctionGeneratorResult struct {
	QueryDefinition string
	RequestFunction string
	Hooks           string
}

type GeneratorBuildArgumentsResult struct {
	Arguments          string
	ArgumentChecks     string
	ArgumentUsage       string
	ArgumentReplacements string
}
