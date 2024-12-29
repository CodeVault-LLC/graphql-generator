package languages

import (
	"fmt"
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
		fmt.Printf("Error loading plugin: %v\n", err)
		return nil
	}

	symGenerateDefinitions, err := plug.Lookup("GenerateDefinitions")
	if err != nil {
		fmt.Printf("Error looking up GenerateDefinitions: %v\n", err)
		return nil
	}

	generateDefinitions, ok := symGenerateDefinitions.(func(config.InternalConfig, types.ExperimentalSchema) error)
	if !ok {
		fmt.Printf("Error asserting GenerateDefinitions function signature\n")
		return nil
	}

	return &Plugin{
		Entrypoint:  entrypoint,
		Plugin:      *plug,
		ExecuteFunc: generateDefinitions,
	}
}
