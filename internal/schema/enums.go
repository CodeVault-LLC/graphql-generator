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
