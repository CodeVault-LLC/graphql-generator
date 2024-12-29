export const use{{name}} = (selection: Partial<Record<keyof {{singular_type}}, boolean>>{{args}}) => {
  return useQuery<{{type}}>({
    queryKey: ['{{name}}', selection{{argsKey}}],
    queryFn: async () => {
      const fields = Object.entries(selection)
        .filter(([_, include]) => include)
        .map(([key]) => key)
        .join('\n');

      if (!fields) throw new Error('No fields selected for query.');
      {{argumentChecks}}

      const query = `
        query {{name}} {
          {{query_name}}{{argumentUsage}} {
            ${fields}
          }
        }
      `;

      const response = await graphqlRequest(query{{argsUsage}}) as { {{name}}: {{type}} };
      return response.{{name}};
    },
  });
};
