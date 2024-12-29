package main

import (
	"fmt"
	"os"

	"github.com/codevault-llc/graphql-generator/config"
	"github.com/codevault-llc/graphql-generator/internal/languages"
	"github.com/codevault-llc/graphql-generator/internal/schema"
)

func main() {
	config := config.NewConfig()

	parser := schema.NewParser(config.Schema)

	languageModule := languages.NewLanguageModule(*config)
	err := languageModule.Execute(config.Language, *config, *parser.Schema)
	if err != nil {
		fmt.Printf("Error executing language module: %v\n", err)
		os.Exit(1)
	}
}
