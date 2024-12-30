package main

import (
	"fmt"
	"strings"

	"github.com/codevault-llc/graphql-generator/internal/schema/types"
)

type TypescriptType struct {
	Nullable bool
	IsEnum   bool
	Value    string
}

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
	case "INPUT_OBJECT":
		return *typeRef.Name
	case "LIST":
		return fmt.Sprintf("%s[]", mapGraphQLToTypeScript(*typeRef.OfType))
	case "NON_NULL":
		return mapGraphQLToTypeScript(*typeRef.OfType)
	case "ENUM":
		return *typeRef.Name
	default:
		return "any"
	}
}

func getGraphQLTypeKind(typeRef types.TypeReference, typeWanted string) bool {
	return typeRef.Kind == typeWanted || (typeRef.OfType != nil && typeRef.OfType.Kind == typeWanted) || (typeRef.OfType != nil && typeRef.OfType.OfType != nil && typeRef.OfType.OfType.Kind == typeWanted)
}

func isSpecialType(typeRef types.TypeReference) string {
	if getGraphQLTypeKind(typeRef, "OBJECT") || getGraphQLTypeKind(typeRef, "INPUT_OBJECT") {
		return mapGraphQLToTypeScript(typeRef)
	}

	return ""
}

func isEnumType(typeRef types.TypeReference) bool {
	return getGraphQLTypeKind(typeRef, "ENUM")
}

type FunctionGeneratorResult struct {
	QueryDefinition string
	RequestFunction string
	Hooks           string

	Imports ImportsSlice
}

type GeneratorResults struct {
	IndexContent string
	Queries      string
	Resources    string
	TypeScript   string
	Tanstack     string
}

type GeneratorBuildArgumentsResult struct {
	Arguments            string
	ArgumentChecks       string
	ArgumentUsage        string
	ArgumentReplacements string
	ArgumentTypes        string
	SpecialArguments     []string
}

type Imports struct {
	Location ImportLocation // ex: "hooks" - "resources" - "queries"
	From     ImportLocation // ex: "./cpl" - "./resources" - "./queries"
	Value    string         // ex: "useQuery" - "useMutation" - "requestQuery"
}

type ImportLocation string

const (
	ImportLocationHooks     ImportLocation = "./gpl"
	ImportLocationResources ImportLocation = "./resources"
	ImportLocationQueries   ImportLocation = "./queries"
	ImportLocationTypes     ImportLocation = "./gpl.d"
)

func (i *Imports) AddImport(importValue Imports) {
	if i.Location == importValue.Location && i.Value == importValue.Value {
		return
	}

	i.Value += ", " + importValue.Value
}

type ImportsSlice []Imports

func (i ImportsSlice) ToImport() string {
	groupedImports := make(map[string][]string)

	// Group imports by their source (`From`)
	for _, imp := range i {
		groupedImports[string(imp.From)] = append(groupedImports[string(imp.From)], imp.Value)
	}

	// Generate import statements
	var importStatements []string
	for source, values := range groupedImports {
		importStatements = append(importStatements, fmt.Sprintf("import { %s } from '%s';", strings.Join(values, ", "), source))
	}

	// Join all import statements into a single string
	return strings.Join(importStatements, "\n")
}

func (i *ImportsSlice) Connect(imports ...Imports) {
	for _, imp := range imports {
		exists := false
		for _, existingImp := range *i {
			if existingImp.Location == imp.Location && existingImp.Value == imp.Value {
				exists = true
				break
			}
		}
		if !exists {
			*i = append(*i, imp)
		}
	}
}

func (i ImportsSlice) GetImportsFromLocation(location ImportLocation) ImportsSlice {
	var imports ImportsSlice
	for _, imp := range i {
		if imp.Location == location {
			imports = append(imports, imp)
		}
	}
	return imports
}
