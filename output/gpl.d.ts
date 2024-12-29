export interface User {
  id: string;
  email: string;
  username: string;
  role: string;
}

export interface Query {
  me: User;
  user: User;
  products: Product[];
  product: Product;
  newsByProduct: News[];
  newsStatisticsByProductId: NewsStatistics;
  newsById: News;
}

export interface Token {
  token: string;
}

export interface Mutation {
  login: Token;
  createProduct: Product;
  updateProduct: Product;
  createNews: News;
  updateNews: News;
}

export interface Product {
  id: string;
  name: string;
  description: string;
  tagline: string;
  category: any;
  status: any;
  public: boolean;
  createdAt: string;
  updatedAt: string;
}

export interface News {
  id: string;
  title: string;
  state: any;
  content: string;
  createdAt: string;
  updatedAt: string;
  publishedAt: string;
}

export interface NewsStatistics {
  total: number;
  published: number;
  draft: number;
  archived: number;
  recentlyUpdated: number;
}

/* 
A GraphQL Schema defines the capabilities of a GraphQL server. It exposes all available types and directives on the server, as well as the entry points for query, mutation, and subscription operations.
 */
export interface __Schema {
  description: string;
  types: __Type[];
  queryType: __Type;
  mutationType: __Type;
  subscriptionType: __Type;
  directives: __Directive[];
}

/* 
The fundamental unit of any GraphQL Schema is the type. There are many kinds of types in GraphQL as represented by the `__TypeKind` enum.

Depending on the kind of a type, certain fields describe information about that type. Scalar types provide no information beyond a name, description and optional `specifiedByUrl`, while Enum types provide their values. Object and Interface types provide the fields they describe. Abstract types, Union and Interface, provide the Object types possible at runtime. List and NonNull types compose other types.
 */
export interface __Type {
  kind: any;
  name: string;
  description: string;
  specifiedByUrl: string;
  fields: __Field[];
  interfaces: __Type[];
  possibleTypes: __Type[];
  enumValues: __EnumValue[];
  inputFields: __InputValue[];
  ofType: __Type;
}

/* 
Object and Interface types are described by a list of Fields, each of which has a name, potentially a list of arguments, and a return type.
 */
export interface __Field {
  name: string;
  description: string;
  args: __InputValue[];
  type: __Type;
  isDeprecated: boolean;
  deprecationReason: string;
}

/* 
Arguments provided to Fields or Directives and the input fields of an InputObject are represented as Input Values which describe their type and optionally a default value.
 */
export interface __InputValue {
  name: string;
  description: string;
  type: __Type;
  defaultValue: string;
  isDeprecated: boolean;
  deprecationReason: string;
}

/* 
One possible value for a given Enum. Enum values are unique values, not a placeholder for a string or numeric value. However an Enum value is returned in a JSON response as a string.
 */
export interface __EnumValue {
  name: string;
  description: string;
  isDeprecated: boolean;
  deprecationReason: string;
}

/* 
A Directive provides a way to describe alternate runtime execution and type validation behavior in a GraphQL document.

In some cases, you need to provide options to alter GraphQL's execution behavior in ways field arguments will not suffice, such as conditionally including or skipping a field. Directives provide this by describing additional information to the executor.
 */
export interface __Directive {
  name: string;
  description: string;
  isRepeatable: boolean;
  locations: any[];
  args: __InputValue[];
}

