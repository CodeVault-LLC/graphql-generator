package main

import (
	"fmt"
	"strings"

	"github.com/codevault-llc/graphql-generator/internal/schema/types"
)

func (g *Generator) turnQueryIntoTanstackQuery(queries types.ExperimentalSchemaField) FunctionGeneratorResult {
	var queryDefinitions string

	var requestFunctions string

	var hooks string

	var imports ImportsSlice

	for _, query := range queries.Types[0].Fields {
		name := strings.ToUpper(query.Name[:1]) + query.Name[1:]
		queryName := query.Name

		arguments := g.buildArguments(query)

		singularType := strings.Replace(mapGraphQLToTypeScript(query.Type).Value, "[]", "", -1)

		var argumentUsag string
		if len(query.Arguments) > 0 {
			argumentUsag = ", args"
		} else {
			argumentUsag = ""
		}

		queryDefinition := fmt.Sprintf(`export const %sQuery = `+"`"+`query %s {
  %s%s {
    {{fields}}
  }
}`+"`;", queryName, name, queryName, arguments.ArgumentUsage)

		imports.Connect(Imports{
			Location: ImportLocationResources,
			From:     ImportLocationTypes,
			Value:    singularType,
		}, Imports{
			Location: ImportLocationResources,
			From:     ImportLocationQueries,
			Value:    fmt.Sprintf("%sQuery", queryName),
		})

		requestFunction := fmt.Sprintf(`export const request%s = async (selection: Partial<Record<keyof %s, boolean>>, %s) => {
  const fields = Object.entries(selection)
    .filter(([_, include]) => include)
    .map(([key]) => key)
    .join("\n");

  if (!fields) throw new Error('No fields selected for query.');
  %s

  let query = %sQuery.replace('{{fields}}', fields);
	%s

  const response = await graphqlRequest(query) as { %s: %s };
  return response.%s;
};`, name, singularType, arguments.Arguments, arguments.ArgumentChecks, queryName, arguments.ArgumentReplacements, queryName, singularType, queryName)

		imports.Connect(Imports{
			Location: ImportLocationHooks,
			From:     ImportLocationResources,
			Value:    fmt.Sprintf("request%s", name),
		}, Imports{
			Location: ImportLocationHooks,
			From:     "@tanstack/react-query",
			Value:    "useQuery",
		}, Imports{
			Location: ImportLocationHooks,
			From:     ImportLocationTypes,
			Value:    singularType,
		})

		for _, arg := range arguments.SpecialArguments {
			imports.Connect(Imports{
				Location: ImportLocationResources,
				From:     ImportLocationTypes,
				Value:    arg,
			}, Imports{
				Location: ImportLocationHooks,
				From:     ImportLocationTypes,
				Value:    arg,
			})
		}

		// Define tanstack hook
		tanstackHook := fmt.Sprintf(`export const use%s = (selection: Partial<Record<keyof %s, boolean>>, %s) => {
  return useQuery<%s>({
    queryKey: ['%s', selection],
    queryFn: async () => {
      return await request%s(selection%s);
    },
  });
};`, name, singularType, arguments.Arguments, singularType, name, name, argumentUsag)

		queryDefinitions += queryDefinition + "\n\n"
		requestFunctions += requestFunction + "\n\n"
		hooks += tanstackHook + "\n\n"
	}

	return FunctionGeneratorResult{
		QueryDefinition: queryDefinitions,
		RequestFunction: requestFunctions,
		Hooks:           hooks,

		Imports: imports,
	}
}
