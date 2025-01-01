package languages

import (
	"log"
	"plugin"

	"github.com/codevault-llc/graphql-generator/config"
	"github.com/codevault-llc/graphql-generator/internal/schema/types"
)

type Plugin struct {
	Entrypoint  string
	Plugin      plugin.Plugin
	ExecuteFunc func(config.InternalConfig, types.ExperimentalSchema) error
}

func NewPlugin(entrypoint string) *Plugin {
	plug, err := plugin.Open(entrypoint)
	if err != nil {
		log.Fatalf("Error loading plugin: %v\n", err)

		return nil
	}

	symGenerateDefinitions, err := plug.Lookup("GenerateDefinitions")
	if err != nil {
		log.Fatalf("Error loading GenerateDefinitions symbol: %v\n", err)

		return nil
	}

	generateDefinitions, ok := symGenerateDefinitions.(func(config.InternalConfig, types.ExperimentalSchema) error)
	if !ok {
		log.Fatalf("Error casting GenerateDefinitions symbol: %v\n", err)

		return nil
	}

	return &Plugin{
		Entrypoint:  entrypoint,
		Plugin:      *plug,
		ExecuteFunc: generateDefinitions,
	}
}
