export const meQuery = `query Me {
  me {
    {{fields}}
  }
}`;

export const userQuery = `query User {
  user(id: "{{args.id}}") {
    {{fields}}
  }
}`;

export const productsQuery = `query Products {
  products {
    {{fields}}
  }
}`;

export const productQuery = `query Product {
  product(id: "{{args.id}}") {
    {{fields}}
  }
}`;

export const newsByProductQuery = `query NewsByProduct {
  newsByProduct(productId: "{{args.productId}}") {
    {{fields}}
  }
}`;

export const newsStatisticsByProductIdQuery = `query NewsStatisticsByProductId {
  newsStatisticsByProductId(productId: "{{args.productId}}") {
    {{fields}}
  }
}`;

export const newsByIdQuery = `query NewsById {
  newsById(id: "{{args.id}}") {
    {{fields}}
  }
}`;

export const loginQuery = `mutation Login {
  login(email: "{{args.email}}", password: "{{args.password}}") {
    {{fields}}
  }
}`;

export const createProductQuery = `mutation CreateProduct {
  createProduct(data: "{{args.data}}") {
    {{fields}}
  }
}`;

export const updateProductQuery = `mutation UpdateProduct {
  updateProduct(id: "{{args.id}}", data: "{{args.data}}") {
    {{fields}}
  }
}`;

export const createNewsQuery = `mutation CreateNews {
  createNews(productId: "{{args.productId}}", data: "{{args.data}}") {
    {{fields}}
  }
}`;

export const updateNewsQuery = `mutation UpdateNews {
  updateNews(id: "{{args.id}}", data: "{{args.data}}") {
    {{fields}}
  }
}`;

