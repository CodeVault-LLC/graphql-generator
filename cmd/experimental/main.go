package main

import (
	"log"

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
		log.Fatalf("Error executing language module: %v\n", err)
	}
}
