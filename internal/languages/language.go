package languages

import (
	"fmt"
	"log"

	"github.com/codevault-llc/graphql-generator/config"
	"github.com/codevault-llc/graphql-generator/internal/schema/types"
)

type LanguageModule struct {
	languages map[string]Plugin
}

func NewLanguageModule(config config.InternalConfig) *LanguageModule {
	var languages = map[string]Plugin{}
	for _, language := range config.Languages {
		plugin := NewPlugin(language.Entrypoint)
		if plugin != nil {
			log.Printf("Loaded plugin for language: %s\n", language.Name)
			languages[language.Name] = *plugin
		}
	}

	return &LanguageModule{
		languages,
	}
}

func (lm *LanguageModule) Execute(language string, config config.InternalConfig, schema types.ExperimentalSchema) error {
	plugin, ok := lm.languages[language]
	if !ok {
		return fmt.Errorf("plugin for language %s not found", language)
	}

	err := plugin.ExecuteFunc(config, schema)
	if err != nil {
		return err
	}

	return nil
}
