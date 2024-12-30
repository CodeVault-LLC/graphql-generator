package schema

import (
	"github.com/codevault-llc/graphql-generator/internal/schema/types"
)

type Parser struct {
	SchemaLocation string
	RawSchema      *RawSchema
	Schema         *types.ExperimentalSchema
}

func NewParser(schemaLocation string) *Parser {
	parser := &Parser{
		SchemaLocation: schemaLocation,
	}

	parsedSchema, err := types.ParseSchema(schemaLocation)
	if err != nil {
		panic(err)
	}

	parser.Schema = parsedSchema

	return parser
}
