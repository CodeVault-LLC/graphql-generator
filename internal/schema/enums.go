package schema

import (
	"bytes"
	"fmt"

	"github.com/codevault-llc/graphql-generator/pkg/utils"
)

type Enum struct {
	Name   string
	Values []string
}

func (p *Parser) parseSchemaToEnums() ([]Enum, error) {
	enums := make([]Enum, 0)

	for _, t := range p.RawSchema.Schema.Types {
		if t.Kind != "ENUM" {
			continue
		}

		// Check if EnumValues exists and is valid
		if t.EnumValues == nil {
			return nil, fmt.Errorf("enum %s has nil EnumValues", t.Name)
		}

		enumValues, ok := t.EnumValues.([]interface{})
		if !ok {
			return nil, fmt.Errorf("expected EnumValues of %s to be a slice of interfaces, got %T", t.Name, t.EnumValues)
		}

		values := make([]string, 0)
		for _, v := range enumValues {
			// Ensure each value is a string
			value, ok := v.(map[string]interface{})
			if !ok {
				return nil, fmt.Errorf("expected enum value for %s to be a map, got %T", t.Name, v)
			}

			if valueName, exists := value["name"]; exists {
				if nameStr, ok := valueName.(string); ok {
					values = append(values, nameStr)
				} else {
					return nil, fmt.Errorf("expected enum value name for %s to be string, got %T", t.Name, valueName)
				}
			}
		}

		enum := Enum{
			Name:   t.Name,
			Values: values,
		}

		enums = append(enums, enum)
	}

	return enums, nil
}

// Helper function to generate TypeScript enums
func (p *Parser) GenerateTypescriptEnums(enums []Enum) string {
	var buffer bytes.Buffer

	buffer.WriteString(utils.GenerateMetadata("gpl.d.ts", p.SchemaLocation) + "\n\n")

	for _, enum := range enums {
		buffer.WriteString(fmt.Sprintf("export enum %s {\n", enum.Name))
		for _, value := range enum.Values {
			buffer.WriteString(fmt.Sprintf("\t%s = \"%s\",\n", value, value))
		}
		buffer.WriteString("}\n\n")
	}

	return buffer.String()
}
