package schema

type RawSchema struct {
	Schema struct {
		QueryType struct {
			Name string `json:"name"`
		} `json:"queryType"`
		MutationType struct {
			Name string `json:"name"`
		} `json:"mutationType"`
		SubscriptionType interface{} `json:"subscriptionType"`
		Types            []RawType   `json:"types"`
		Directives       []struct {
			Name        string        `json:"name"`
			Description string        `json:"description"`
			Locations   []string      `json:"locations"`
			Args        []interface{} `json:"args"`
		} `json:"directives"`
	} `json:"__schema"`
}

type RawType struct {
	Kind          string         `json:"kind"`
	Name          string         `json:"name"`
	Description   interface{}    `json:"description"`
	Fields        []RawTypeField `json:"fields"`
	InputFields   interface{}    `json:"inputFields"`
	Interfaces    []interface{}  `json:"interfaces"`
	EnumValues    interface{}    `json:"enumValues"`
	PossibleTypes interface{}    `json:"possibleTypes"`
}

type RawTypeField struct {
	Name        string           `json:"name"`
	Description interface{}      `json:"description"`
	Args        []RawArguments   `json:"args"`
	Type        RawTypeFieldType `json:"type"`
}

type RawArguments struct {
	Name         string           `json:"name"`
	Description  string           `json:"description"`
	Type         RawTypeFieldType `json:"type"`
	DefaultValue string           `json:"defaultValue"`
}

type RawTypeFieldType struct {
	Kind   string            `json:"kind"`
	Name   interface{}       `json:"name"`
	OfType *RawTypeFieldType `json:"ofType"`
}
