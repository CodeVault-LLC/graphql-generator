package main

import (
	"fmt"
	"strings"

	"github.com/codevault-llc/graphql-generator/internal/schema/types"
)

func (g *Generator) turnMutationIntoTanstackMutation(mutations types.ExperimentalSchemaField) (string, string, string) {
	var queryDefinitions string
	var requestFunctions string
	var hooks string

	for _, query := range mutations.Types[0].Fields {
		name := strings.ToUpper(query.Name[:1]) + query.Name[1:]
		queryName := query.Name

		// Construct arguments for query
		args := ""
		argumentChecks := ""
		argumentUsage := ""
		argumentReplacements := ""

		if len(query.Arguments) > 0 {
			args = "args: { "
			argumentUsage = "("
			for i, arg := range query.Arguments {
				args += fmt.Sprintf("%s: %s", arg.Name, mapGraphQLToTypeScript(arg.Type))
				argumentChecks += fmt.Sprintf("if (!args.%s) throw new Error('%s is required.');\n", arg.Name, arg.Name)
				argumentUsage += fmt.Sprintf("%s: \"{{args.%s}}\"", arg.Name, arg.Name)
				argumentReplacements += fmt.Sprintf("query = query.replace('{{args.%s}}', args.%s);\n", arg.Name, arg.Name)

				if i < len(query.Arguments)-1 {
					args += ", "
					argumentUsage += ", "
				}
			}
			args += " }"
			argumentUsage += ")"
		}

		singularType := strings.Replace(mapGraphQLToTypeScript(query.Type), "[]", "", -1)
		var argumentUsag string

		if len(query.Arguments) > 0 {
			argumentUsag = ", args"
		} else {
			argumentUsag = ""
		}

		// Define query
		queryDefinition := fmt.Sprintf(`export const %sQuery = `+"`"+`mutation %s %s {
  %s%s {
    {{fields}}
  }
}`+"`;", queryName, name, argumentUsage, queryName, argumentUsage)

		// Define request function
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
};`, name, singularType, args, argumentChecks, argumentReplacements, queryName, queryName, singularType, queryName)

		// Define tanstack hook
		tanstackHook := fmt.Sprintf(`export const use%s = (selection: Partial<Record<keyof %s, boolean>>, %s) => {
  return useMutation<%s>({
    queryKey: ['%s', selection],
    queryFn: async () => {
      return await request%s(selection%s);
    },
  });
};`, name, singularType, args, singularType, name, name, argumentUsag)

		queryDefinitions += queryDefinition + "\n\n"
		requestFunctions += requestFunction + "\n\n"
		hooks += tanstackHook + "\n\n"
	}

	return hooks, queryDefinitions, requestFunctions
}
