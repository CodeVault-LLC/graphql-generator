package schema

import (
	"os"

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

	parsedSchema, err := types.ParseSchemaFromJSON(schemaData)
	if err != nil {
		panic(err)
	}

	parser.Schema = parsedSchema

	return parser
}
