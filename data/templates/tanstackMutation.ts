export const use{{name}} = (selection: Partial<Record<keyof {{singular_type}}, boolean>>) => {
  return useMutation<{{type}}, unknown, {{argsTypes}}> ({
    mutationKey: ['{{query_name}}'],
    mutationFn: async (data) => {
      const fields = Object.entries(selection)
        .filter(([_, include]) => include)
        .map(([key]) => key)
        .join("\n");

      if (!fields) throw new Error('No fields selected for mutation.');
      {{argumentChecks}}

      const query = `
        mutation {
          {{query_name}}{{argumentUsage}} {
            ${fields}
          }
        }
      `;

      const response = await graphqlRequest(query) as { {{name}}: {{type}} };
      return response.{{name}};
    },
  });
};
