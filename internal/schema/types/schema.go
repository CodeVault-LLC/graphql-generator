package types

import (
	"encoding/json"
	"errors"
)

// ExperimentalSchemaFieldType represents the type of a GraphQL schema field.
type ExperimentalSchemaFieldType string

const (
	ExperimentalSchemaFieldTypeMutation     ExperimentalSchemaFieldType = "Mutation"
	ExperimentalSchemaFieldTypeQuery        ExperimentalSchemaFieldType = "Query"
	ExperimentalSchemaFieldTypeType         ExperimentalSchemaFieldType = "OBJECT"
	ExperimentalSchemaFieldTypeEnum         ExperimentalSchemaFieldType = "ENUM"
	ExperimentalSchemaFieldTypeInput        ExperimentalSchemaFieldType = "Input"
	ExperimentalSchemaFieldTypeInterface    ExperimentalSchemaFieldType = "Interface"
	ExperimentalSchemaFieldTypeUnion        ExperimentalSchemaFieldType = "Union"
	ExperimentalSchemaFieldTypeSubscription ExperimentalSchemaFieldType = "Subscription"
)

// Type represents a GraphQL object type with its fields.
type Type struct {
	Name        string
	Fields      []Field
	Interfaces  []string // Interfaces implemented by this type
	Description string
}

// Enum represents a GraphQL enum type.
type Enum struct {
	Name        string
	Values      []string
	Description string
}

// Field represents a GraphQL field with its type and arguments.
type Field struct {
	Name              string
	Type              TypeReference
	Arguments         []Argument
	Description       string
	Tags              []string
	IsDeprecated      bool
	DeprecationReason string
}

// Argument represents a GraphQL field argument.
type Argument struct {
	Name         string
	Type         TypeReference
	DefaultValue *string
	Description  string
}

// TypeReference represents a GraphQL type reference that can handle nested and composite types.
type TypeReference struct {
	Kind        string         // SCALAR, OBJECT, LIST, NON_NULL, etc.
	Name        *string        // Name of the type (e.g., "User", "ID")
	OfType      *TypeReference // Nested type for LIST, NON_NULL, etc.
	Description string
}

// ExperimentalSchemaField represents a GraphQL schema field.
type ExperimentalSchemaField struct {
	Name         string
	Type         ExperimentalSchemaFieldType
	Types        []Type
	Arguments    []Argument
	Enums        []Enum
	Interfaces   []string
	Unions       []string
	Description  string
	Tags         []string
	IsDeprecated bool
}

// ExperimentalSchema represents the entire GraphQL schema.
type ExperimentalSchema struct {
	Fields []ExperimentalSchemaField
}

// ParseSchemaFromJSON parses a GraphQL schema JSON file into an ExperimentalSchema structure.
func ParseSchemaFromJSON(data []byte) (*ExperimentalSchema, error) {
	var rawSchema map[string]interface{}
	if err := json.Unmarshal(data, &rawSchema); err != nil {
		return nil, err
	}

	schemaData, ok := rawSchema["__schema"].(map[string]interface{})
	if !ok {
		return nil, errors.New("missing __schema in JSON")
	}

	types, ok := schemaData["types"].([]interface{})
	if !ok {
		return nil, errors.New("missing types in schema")
	}

	fields := []ExperimentalSchemaField{}

	for _, t := range types {
		if typeMap, ok := t.(map[string]interface{}); ok {
			field, err := parseType(typeMap)
			if err == nil {
				fields = append(fields, field)
			}
		}
	}

	return &ExperimentalSchema{Fields: fields}, nil
}

func parseType(typeMap map[string]interface{}) (ExperimentalSchemaField, error) {
	name, _ := typeMap["name"].(string)
	kind, _ := typeMap["kind"].(string)
	description, _ := typeMap["description"].(string)

	fields := []Field{}
	if rawFields, ok := typeMap["fields"].([]interface{}); ok {
		for _, rawField := range rawFields {
			if fieldMap, ok := rawField.(map[string]interface{}); ok {
				field := parseField(fieldMap)
				fields = append(fields, field)
			}
		}
	}

	return ExperimentalSchemaField{
		Name:        name,
		Type:        ExperimentalSchemaFieldType(kind),
		Types:       []Type{{Name: name, Fields: fields, Description: description}},
		Description: description,
	}, nil
}

func parseField(fieldMap map[string]interface{}) Field {
	name, _ := fieldMap["name"].(string)
	description, _ := fieldMap["description"].(string)
	isDeprecated, _ := fieldMap["isDeprecated"].(bool)
	deprecationReason, _ := fieldMap["deprecationReason"].(string)

	typeRef := parseTypeReference(fieldMap["type"].(map[string]interface{}))
	arguments := []Argument{}
	if rawArgs, ok := fieldMap["args"].([]interface{}); ok {
		for _, rawArg := range rawArgs {
			if argMap, ok := rawArg.(map[string]interface{}); ok {
				arguments = append(arguments, parseArgument(argMap))
			}
		}
	}

	return Field{
		Name:              name,
		Type:              typeRef,
		Arguments:         arguments,
		Description:       description,
		IsDeprecated:      isDeprecated,
		DeprecationReason: deprecationReason,
	}
}

func parseArgument(argMap map[string]interface{}) Argument {
	name, _ := argMap["name"].(string)
	description, _ := argMap["description"].(string)
	typeRef := parseTypeReference(argMap["type"].(map[string]interface{}))
	var defaultValue *string
	if rawDefault, ok := argMap["defaultValue"].(string); ok {
		defaultValue = &rawDefault
	}

	return Argument{
		Name:         name,
		Type:         typeRef,
		DefaultValue: defaultValue,
		Description:  description,
	}
}

func parseTypeReference(typeMap map[string]interface{}) TypeReference {
	kind, _ := typeMap["kind"].(string)
	name, _ := typeMap["name"].(string)
	ofType := (*TypeReference)(nil)
	if rawOfType, ok := typeMap["ofType"].(map[string]interface{}); ok {
		ofTypeRef := parseTypeReference(rawOfType)
		ofType = &ofTypeRef
	}

	return TypeReference{
		Kind:   kind,
		Name:   &name,
		OfType: ofType,
	}
}
