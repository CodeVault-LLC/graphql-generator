export const meQuery = `query Me  {
  me {
    {{fields}}
  }
}`;

export const userQuery = `query User (id: "{{args.id}}") {
  user(id: "{{args.id}}") {
    {{fields}}
  }
}`;

export const productsQuery = `query Products  {
  products {
    {{fields}}
  }
}`;

export const productQuery = `query Product (id: "{{args.id}}") {
  product(id: "{{args.id}}") {
    {{fields}}
  }
}`;

export const newsByProductQuery = `query NewsByProduct (productId: "{{args.productId}}") {
  newsByProduct(productId: "{{args.productId}}") {
    {{fields}}
  }
}`;

export const newsStatisticsByProductIdQuery = `query NewsStatisticsByProductId (productId: "{{args.productId}}") {
  newsStatisticsByProductId(productId: "{{args.productId}}") {
    {{fields}}
  }
}`;

export const newsByIdQuery = `query NewsById (id: "{{args.id}}") {
  newsById(id: "{{args.id}}") {
    {{fields}}
  }
}`;

export const loginQuery = `mutation Login (email: "{{args.email}}", password: "{{args.password}}") {
  login(email: "{{args.email}}", password: "{{args.password}}") {
    {{fields}}
  }
}`;

export const createProductQuery = `mutation CreateProduct (data: "{{args.data}}") {
  createProduct(data: "{{args.data}}") {
    {{fields}}
  }
}`;

export const updateProductQuery = `mutation UpdateProduct (id: "{{args.id}}", data: "{{args.data}}") {
  updateProduct(id: "{{args.id}}", data: "{{args.data}}") {
    {{fields}}
  }
}`;

export const createNewsQuery = `mutation CreateNews (productId: "{{args.productId}}", data: "{{args.data}}") {
  createNews(productId: "{{args.productId}}", data: "{{args.data}}") {
    {{fields}}
  }
}`;

export const updateNewsQuery = `mutation UpdateNews (id: "{{args.id}}", data: "{{args.data}}") {
  updateNews(id: "{{args.id}}", data: "{{args.data}}") {
    {{fields}}
  }
}`;

