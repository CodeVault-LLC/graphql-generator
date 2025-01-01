import { User, Product, News, NewsStatistics, Token, ProductCreateInput, ProductUpdateInput, NewsCreateInput, NewsUpdateInput } from './gpl.d';
import { meQuery, userQuery, productsQuery, productQuery, newsLatestPublishedQuery, newsByProductQuery, newsStatisticsByProductIdQuery, newsByIdQuery, loginQuery, createProductQuery, updateProductQuery, createNewsQuery, updateNewsQuery } from './queries';export const requestMe = async (selection: Partial<Record<keyof User, boolean>>, ) => {
  const fields = Object.entries(selection)
    .filter(([_, include]) => include)
    .map(([key]) => key)
    .join("\n");

  if (!fields) throw new Error('No fields selected for query.');
  

  let query = meQuery.replace('{{fields}}', fields);
	

  const response = await graphqlRequest(query) as { me: User };
  return response.me;
};

export const requestUser = async (selection: Partial<Record<keyof User, boolean>>, args: { id: string }) => {
  const fields = Object.entries(selection)
    .filter(([_, include]) => include)
    .map(([key]) => key)
    .join("\n");

  if (!fields) throw new Error('No fields selected for query.');
  if (!args.id) throw new Error('id is required.');


  let query = userQuery.replace('{{fields}}', fields);
	const isEnumField = (field: string): boolean => {
 return [
].includes(field);
};
query = query.replace('{{args.id}}', `"${args.id}"`);


  const response = await graphqlRequest(query) as { user: User };
  return response.user;
};

export const requestProducts = async (selection: Partial<Record<keyof Product, boolean>>, ) => {
  const fields = Object.entries(selection)
    .filter(([_, include]) => include)
    .map(([key]) => key)
    .join("\n");

  if (!fields) throw new Error('No fields selected for query.');
  

  let query = productsQuery.replace('{{fields}}', fields);
	

  const response = await graphqlRequest(query) as { products: Product };
  return response.products;
};

export const requestProduct = async (selection: Partial<Record<keyof Product, boolean>>, args: { id: string }) => {
  const fields = Object.entries(selection)
    .filter(([_, include]) => include)
    .map(([key]) => key)
    .join("\n");

  if (!fields) throw new Error('No fields selected for query.');
  if (!args.id) throw new Error('id is required.');


  let query = productQuery.replace('{{fields}}', fields);
	const isEnumField = (field: string): boolean => {
 return [
].includes(field);
};
query = query.replace('{{args.id}}', `"${args.id}"`);


  const response = await graphqlRequest(query) as { product: Product };
  return response.product;
};

export const requestNewsLatestPublished = async (selection: Partial<Record<keyof News, boolean>>, ) => {
  const fields = Object.entries(selection)
    .filter(([_, include]) => include)
    .map(([key]) => key)
    .join("\n");

  if (!fields) throw new Error('No fields selected for query.');
  

  let query = newsLatestPublishedQuery.replace('{{fields}}', fields);
	

  const response = await graphqlRequest(query) as { newsLatestPublished: News };
  return response.newsLatestPublished;
};

export const requestNewsByProduct = async (selection: Partial<Record<keyof News, boolean>>, args: { productId: string }) => {
  const fields = Object.entries(selection)
    .filter(([_, include]) => include)
    .map(([key]) => key)
    .join("\n");

  if (!fields) throw new Error('No fields selected for query.');
  if (!args.productId) throw new Error('productId is required.');


  let query = newsByProductQuery.replace('{{fields}}', fields);
	const isEnumField = (field: string): boolean => {
 return [
].includes(field);
};
query = query.replace('{{args.productId}}', `"${args.productId}"`);


  const response = await graphqlRequest(query) as { newsByProduct: News };
  return response.newsByProduct;
};

export const requestNewsStatisticsByProductId = async (selection: Partial<Record<keyof NewsStatistics, boolean>>, args: { productId: string }) => {
  const fields = Object.entries(selection)
    .filter(([_, include]) => include)
    .map(([key]) => key)
    .join("\n");

  if (!fields) throw new Error('No fields selected for query.');
  if (!args.productId) throw new Error('productId is required.');


  let query = newsStatisticsByProductIdQuery.replace('{{fields}}', fields);
	const isEnumField = (field: string): boolean => {
 return [
].includes(field);
};
query = query.replace('{{args.productId}}', `"${args.productId}"`);


  const response = await graphqlRequest(query) as { newsStatisticsByProductId: NewsStatistics };
  return response.newsStatisticsByProductId;
};

export const requestNewsById = async (selection: Partial<Record<keyof News, boolean>>, args: { id: string }) => {
  const fields = Object.entries(selection)
    .filter(([_, include]) => include)
    .map(([key]) => key)
    .join("\n");

  if (!fields) throw new Error('No fields selected for query.');
  if (!args.id) throw new Error('id is required.');


  let query = newsByIdQuery.replace('{{fields}}', fields);
	const isEnumField = (field: string): boolean => {
 return [
].includes(field);
};
query = query.replace('{{args.id}}', `"${args.id}"`);


  const response = await graphqlRequest(query) as { newsById: News };
  return response.newsById;
};

export const requestLogin = async (selection: Partial<Record<keyof Token, boolean>>, args: { email: string, password: string }) => {
  const fields = Object.entries(selection)
    .filter(([_, include]) => include)
    .map(([key]) => key)
    .join("\n");

  if (!fields) throw new Error('No fields selected for query.');
  if (!args.email) throw new Error('email is required.');
if (!args.password) throw new Error('password is required.');


  let query = loginQuery.replace('{{fields}}', fields);
	const isEnumField = (field: string): boolean => {
 return [
].includes(field);
};
query = query.replace('{{args.email}}', `"${args.email}"`);
query = query.replace('{{args.password}}', `"${args.password}"`);


  const response = await graphqlRequest(query) as { login: Token };
  return response.login;
};

export const requestCreateProduct = async (selection: Partial<Record<keyof Product, boolean>>, args: { data: ProductCreateInput | null }) => {
  const fields = Object.entries(selection)
    .filter(([_, include]) => include)
    .map(([key]) => key)
    .join("\n");

  if (!fields) throw new Error('No fields selected for query.');
  if (!args.data) throw new Error('data is required.');


  let query = createProductQuery.replace('{{fields}}', fields);
	const isEnumField = (field: string): boolean => {
 return [
].includes(field);
};
const dataFields = Object.entries(args.data)
  .map(([key, value]) => {
    if (isEnumField(key)) {
      return `${key}: ${value}`;
    }
    return `${key}: ${JSON.stringify(value)}`;
  })
  .join(", ");query = query.replace("{{args.data}}", dataFields);


  const response = await graphqlRequest(query) as { createProduct: Product };
  return response.createProduct;
};

export const requestUpdateProduct = async (selection: Partial<Record<keyof Product, boolean>>, args: { id: string, data: ProductUpdateInput | null }) => {
  const fields = Object.entries(selection)
    .filter(([_, include]) => include)
    .map(([key]) => key)
    .join("\n");

  if (!fields) throw new Error('No fields selected for query.');
  if (!args.id) throw new Error('id is required.');
if (!args.data) throw new Error('data is required.');


  let query = updateProductQuery.replace('{{fields}}', fields);
	const isEnumField = (field: string): boolean => {
 return [
].includes(field);
};
query = query.replace('{{args.id}}', `"${args.id}"`);
const dataFields = Object.entries(args.data)
  .map(([key, value]) => {
    if (isEnumField(key)) {
      return `${key}: ${value}`;
    }
    return `${key}: ${JSON.stringify(value)}`;
  })
  .join(", ");query = query.replace("{{args.data}}", dataFields);


  const response = await graphqlRequest(query) as { updateProduct: Product };
  return response.updateProduct;
};

export const requestCreateNews = async (selection: Partial<Record<keyof News, boolean>>, args: { productId: string, data: NewsCreateInput }) => {
  const fields = Object.entries(selection)
    .filter(([_, include]) => include)
    .map(([key]) => key)
    .join("\n");

  if (!fields) throw new Error('No fields selected for query.');
  if (!args.productId) throw new Error('productId is required.');
if (!args.data) throw new Error('data is required.');


  let query = createNewsQuery.replace('{{fields}}', fields);
	const isEnumField = (field: string): boolean => {
 return [
].includes(field);
};
query = query.replace('{{args.productId}}', `"${args.productId}"`);
const dataFields = Object.entries(args.data)
  .map(([key, value]) => {
    if (isEnumField(key)) {
      return `${key}: ${value}`;
    }
    return `${key}: ${JSON.stringify(value)}`;
  })
  .join(", ");query = query.replace("{{args.data}}", dataFields);


  const response = await graphqlRequest(query) as { createNews: News };
  return response.createNews;
};

export const requestUpdateNews = async (selection: Partial<Record<keyof News, boolean>>, args: { id: string, data: NewsUpdateInput }) => {
  const fields = Object.entries(selection)
    .filter(([_, include]) => include)
    .map(([key]) => key)
    .join("\n");

  if (!fields) throw new Error('No fields selected for query.');
  if (!args.id) throw new Error('id is required.');
if (!args.data) throw new Error('data is required.');


  let query = updateNewsQuery.replace('{{fields}}', fields);
	const isEnumField = (field: string): boolean => {
 return [
].includes(field);
};
query = query.replace('{{args.id}}', `"${args.id}"`);
const dataFields = Object.entries(args.data)
  .map(([key, value]) => {
    if (isEnumField(key)) {
      return `${key}: ${value}`;
    }
    return `${key}: ${JSON.stringify(value)}`;
  })
  .join(", ");query = query.replace("{{args.data}}", dataFields);


  const response = await graphqlRequest(query) as { updateNews: News };
  return response.updateNews;
};

export const isFieldEnum = (field: string): boolean => {
  return [
    'ProductCategory',
    'ProductStatus',
    'NewsState',
    '__TypeKind',
    '__DirectiveLocation',
  ].includes(field);
};
