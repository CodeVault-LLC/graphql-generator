package types

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net/http"
	"os"
	"strings"
)

// ExperimentalSchemaFieldType represents the type of a GraphQL schema field.
type ExperimentalSchemaFieldType string

const (
	ExperimentalSchemaFieldTypeMutation     ExperimentalSchemaFieldType = "Mutation"
	ExperimentalSchemaFieldTypeQuery        ExperimentalSchemaFieldType = "Query"
	ExperimentalSchemaFieldTypeType         ExperimentalSchemaFieldType = "OBJECT"
	ExperimentalSchemaFieldTypeEnum         ExperimentalSchemaFieldType = "ENUM"
	ExperimentalSchemaFieldTypeInput        ExperimentalSchemaFieldType = "INPUT_OBJECT"
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

func ParseSchema(schemaLocation string) (*ExperimentalSchema, error) {
	if strings.Contains(schemaLocation, ".json") {
		file, err := os.Open(schemaLocation)
		if err != nil {
			panic(err)
		}
		defer file.Close()

		fileInfo, _ := file.Stat()
		schemaData := make([]byte, fileInfo.Size())
		_, err = file.Read(schemaData)

		if err != nil {
			panic(err)
		}

		return parseSchemaFromJSON(schemaData)
	}

	return parseSchemaFromUrl(schemaLocation)
}

func parseSchemaFromUrl(url string) (*ExperimentalSchema, error) {
	introspectionQuery := `query IntrospectionQuery {
      __schema {

        queryType { name }
        mutationType { name }
        subscriptionType { name }
        types {
          ...FullType
        }
        directives {
          name
          description

          locations
          args(includeDeprecated: true) {
            ...InputValue
          }
        }
      }
    }

    fragment FullType on __Type {
      kind
      name
      description

      fields(includeDeprecated: true) {
        name
        description
        args(includeDeprecated: true) {
          ...InputValue
        }
        type {
          ...TypeRef
        }
        isDeprecated
        deprecationReason
      }
      inputFields(includeDeprecated: true) {
        ...InputValue
      }
      interfaces {
        ...TypeRef
      }
      enumValues(includeDeprecated: true) {
        name
        description
        isDeprecated
        deprecationReason
      }
      possibleTypes {
        ...TypeRef
      }
    }

    fragment InputValue on __InputValue {
      name
      description
      type { ...TypeRef }
      defaultValue
      isDeprecated
      deprecationReason
    }

    fragment TypeRef on __Type {
      kind
      name
      ofType {
        kind
        name
        ofType {
          kind
          name
          ofType {
            kind
            name
            ofType {
              kind
              name
              ofType {
                kind
                name
                ofType {
                  kind
                  name
                  ofType {
                    kind
                    name
                  }
                }
              }
            }
          }
        }
      }
    }`

	payload := map[string]string{"query": introspectionQuery}
	payloadBytes, err := json.Marshal(payload)

	if err != nil {
		return nil, fmt.Errorf("failed to encode query: %w", err)
	}

	req, err := http.NewRequest("POST", url, bytes.NewBuffer(payloadBytes))
	if err != nil {
		return nil, fmt.Errorf("failed to create request: %w", err)
	}

	req.Header.Set("Content-Type", "application/json")

	client := &http.Client{}
	resp, err := client.Do(req)

	if err != nil {
		return nil, fmt.Errorf("failed to execute request: %w", err)
	}

	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		body, _ := io.ReadAll(resp.Body)
		return nil, fmt.Errorf("unexpected status: %s\n%s", resp.Status, string(body))
	}

	body := &bytes.Buffer{}

	_, err = body.ReadFrom(resp.Body)

	if err != nil {
		return nil, fmt.Errorf("failed to read response body: %w", err)
	}

	return parseSchemaFromJSON(body.Bytes()[8 : len(body.Bytes())-2])
}

// parseSchemaFromJSON parses a GraphQL schema JSON file into an ExperimentalSchema structure.
func parseSchemaFromJSON(data []byte) (*ExperimentalSchema, error) {
	var rawSchema map[string]interface{}
	if err := json.Unmarshal(data, &rawSchema); err != nil {
		return nil, errors.New("failed to unmarshal JSON")
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
			field := parseType(typeMap)
			fields = append(fields, field)
		}
	}

	return &ExperimentalSchema{Fields: fields}, nil
}

func parseType(typeMap map[string]interface{}) ExperimentalSchemaField {
	name, _ := typeMap["name"].(string)
	kind, _ := typeMap["kind"].(string)
	description, _ := typeMap["description"].(string)

	fields := []Field{}
	enums := []Enum{}

	if rawFields, ok := typeMap["fields"].([]interface{}); ok {
		for _, rawField := range rawFields {
			if fieldMap, ok := rawField.(map[string]interface{}); ok {
				field := parseField(fieldMap)
				fields = append(fields, field)
			}
		}
	}

	if rawInputFields, ok := typeMap["inputFields"].([]interface{}); ok {
		for _, rawField := range rawInputFields {
			if fieldMap, ok := rawField.(map[string]interface{}); ok {
				field := parseField(fieldMap)
				fields = append(fields, field)
			}
		}
	}

	if rawEnums, ok := typeMap["enumValues"].([]interface{}); ok {
		enum := Enum{
			Name:        name,
			Description: description,
		}

		for _, rawEnum := range rawEnums {
			if enumMap, ok := rawEnum.(map[string]interface{}); ok {
				enum.Values = append(enum.Values, enumMap["name"].(string))
			}
		}

		enums = append(enums, enum)
	}

	return ExperimentalSchemaField{
		Name:        name,
		Type:        ExperimentalSchemaFieldType(kind),
		Types:       []Type{{Name: name, Fields: fields, Description: description}},
		Enums:       enums,
		Description: description,
	}
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
