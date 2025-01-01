export interface User {
  id: string;
  email: string;
  username: string;
  role: string;
}

export interface Query {
  me: User | null;
  user: User | null;
  products: Product[];
  product: Product | null;
  newsLatestPublished: News[];
  newsByProduct: News[];
  newsStatisticsByProductId: NewsStatistics;
  newsById: News;
}

export interface Token {
  token: string;
}

export interface Mutation {
  login: Token | null;
  createProduct: Product;
  updateProduct: Product;
  createNews: News;
  updateNews: News;
}

/* 

 */
export enum ProductCategory {
  api = "api",
  library = "library",
  framework = "framework",
  tool = "tool",
  plugin = "plugin",
  template = "template",
  other = "other",
}

/* 

 */
export enum ProductStatus {
  stable = "stable",
  beta = "beta",
  alpha = "alpha",
  deprecated = "deprecated",
  coming_soon = "coming_soon",
  planned = "planned",
  in_progress = "in_progress",
}

export interface Product {
  id: string;
  name: string;
  description: string;
  tagline: string;
  category: ProductCategory;
  status: ProductStatus;
  public: boolean;
  createdAt: string;
  updatedAt: string;
}

export interface ProductCreateInput {
  name: string;
  description: string;
  tagline: string;
  public: boolean;
  status: ProductStatus;
  category: ProductCategory;
  tags: string[] | null;
}

export interface ProductUpdateInput {
  name: string;
  description: string;
  tagline: string;
  public: boolean;
  status: ProductStatus;
  category: ProductCategory;
  tags: string[] | null;
}

/* 

 */
export enum NewsState {
  draft = "draft",
  published = "published",
  archived = "archived",
}

export interface News {
  id: string;
  title: string;
  state: NewsState;
  content: string;
  createdAt: string;
  updatedAt: string;
  publishedAt: string | null;
}

export interface NewsCreateInput {
  title: string;
  content: string;
  state: NewsState;
}

export interface NewsUpdateInput {
  title: string | null;
  content: string | null;
  state: NewsState | null;
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
  description: string | null;
  types: __Type[];
  queryType: __Type;
  mutationType: __Type | null;
  subscriptionType: __Type | null;
  directives: __Directive[];
}

/* 
The fundamental unit of any GraphQL Schema is the type. There are many kinds of types in GraphQL as represented by the `__TypeKind` enum.

Depending on the kind of a type, certain fields describe information about that type. Scalar types provide no information beyond a name, description and optional `specifiedByURL`, while Enum types provide their values. Object and Interface types provide the fields they describe. Abstract types, Union and Interface, provide the Object types possible at runtime. List and NonNull types compose other types.
 */
export interface __Type {
  kind: __TypeKind;
  name: string | null;
  description: string | null;
  specifiedByURL: string | null;
  fields: __Field[] | null;
  interfaces: __Type[] | null;
  possibleTypes: __Type[] | null;
  enumValues: __EnumValue[] | null;
  inputFields: __InputValue[] | null;
  ofType: __Type | null;
  isOneOf: boolean | null;
}

/* 
An enum describing what kind of type a given `__Type` is.
 */
export enum __TypeKind {
  SCALAR = "SCALAR",
  OBJECT = "OBJECT",
  INTERFACE = "INTERFACE",
  UNION = "UNION",
  ENUM = "ENUM",
  INPUT_OBJECT = "INPUT_OBJECT",
  LIST = "LIST",
  NON_NULL = "NON_NULL",
}

/* 
Object and Interface types are described by a list of Fields, each of which has a name, potentially a list of arguments, and a return type.
 */
export interface __Field {
  name: string;
  description: string | null;
  args: __InputValue[];
  type: __Type;
  isDeprecated: boolean;
  deprecationReason: string | null;
}

/* 
Arguments provided to Fields or Directives and the input fields of an InputObject are represented as Input Values which describe their type and optionally a default value.
 */
export interface __InputValue {
  name: string;
  description: string | null;
  type: __Type;
  defaultValue: string | null;
  isDeprecated: boolean;
  deprecationReason: string | null;
}

/* 
One possible value for a given Enum. Enum values are unique values, not a placeholder for a string or numeric value. However an Enum value is returned in a JSON response as a string.
 */
export interface __EnumValue {
  name: string;
  description: string | null;
  isDeprecated: boolean;
  deprecationReason: string | null;
}

/* 
A Directive provides a way to describe alternate runtime execution and type validation behavior in a GraphQL document.

In some cases, you need to provide options to alter GraphQL's execution behavior in ways field arguments will not suffice, such as conditionally including or skipping a field. Directives provide this by describing additional information to the executor.
 */
export interface __Directive {
  name: string;
  description: string | null;
  isRepeatable: boolean;
  locations: __DirectiveLocation[];
  args: __InputValue[];
}

/* 
A Directive can be adjacent to many parts of the GraphQL language, a __DirectiveLocation describes one such possible adjacencies.
 */
export enum __DirectiveLocation {
  QUERY = "QUERY",
  MUTATION = "MUTATION",
  SUBSCRIPTION = "SUBSCRIPTION",
  FIELD = "FIELD",
  FRAGMENT_DEFINITION = "FRAGMENT_DEFINITION",
  FRAGMENT_SPREAD = "FRAGMENT_SPREAD",
  INLINE_FRAGMENT = "INLINE_FRAGMENT",
  VARIABLE_DEFINITION = "VARIABLE_DEFINITION",
  SCHEMA = "SCHEMA",
  SCALAR = "SCALAR",
  OBJECT = "OBJECT",
  FIELD_DEFINITION = "FIELD_DEFINITION",
  ARGUMENT_DEFINITION = "ARGUMENT_DEFINITION",
  INTERFACE = "INTERFACE",
  UNION = "UNION",
  ENUM = "ENUM",
  ENUM_VALUE = "ENUM_VALUE",
  INPUT_OBJECT = "INPUT_OBJECT",
  INPUT_FIELD_DEFINITION = "INPUT_FIELD_DEFINITION",
}

