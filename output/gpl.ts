import { useQuery, useMutation } from '@tanstack/react-query';
import { User, Token, ProductCategory, ProductStatus, Product, NewsState, News, NewsStatistics, } from './gpl.d';

export const useMe = (selection: Partial<Record<keyof User, boolean>>, ) => {
  return useQuery<User>({
    queryKey: ['Me', selection],
    queryFn: async () => {
      return await requestMe(selection);
    },
  });
};

export const useUser = (selection: Partial<Record<keyof User, boolean>>, args: { id: string }) => {
  return useQuery<User>({
    queryKey: ['User', selection],
    queryFn: async () => {
      return await requestUser(selection, args);
    },
  });
};

export const useProducts = (selection: Partial<Record<keyof Product, boolean>>, ) => {
  return useQuery<Product>({
    queryKey: ['Products', selection],
    queryFn: async () => {
      return await requestProducts(selection);
    },
  });
};

export const useProduct = (selection: Partial<Record<keyof Product, boolean>>, args: { id: string }) => {
  return useQuery<Product>({
    queryKey: ['Product', selection],
    queryFn: async () => {
      return await requestProduct(selection, args);
    },
  });
};

export const useNewsByProduct = (selection: Partial<Record<keyof News, boolean>>, args: { productId: string }) => {
  return useQuery<News>({
    queryKey: ['NewsByProduct', selection],
    queryFn: async () => {
      return await requestNewsByProduct(selection, args);
    },
  });
};

export const useNewsStatisticsByProductId = (selection: Partial<Record<keyof NewsStatistics, boolean>>, args: { productId: string }) => {
  return useQuery<NewsStatistics>({
    queryKey: ['NewsStatisticsByProductId', selection],
    queryFn: async () => {
      return await requestNewsStatisticsByProductId(selection, args);
    },
  });
};

export const useNewsById = (selection: Partial<Record<keyof News, boolean>>, args: { id: string }) => {
  return useQuery<News>({
    queryKey: ['NewsById', selection],
    queryFn: async () => {
      return await requestNewsById(selection, args);
    },
  });
};

export const useLogin = (selection: Partial<Record<keyof Token, boolean>>, args: { email: string, password: string }) => {
  return useMutation<Token>({
    queryKey: ['Login', selection],
    queryFn: async () => {
      return await requestLogin(selection, args);
    },
  });
};

export const useCreateProduct = (selection: Partial<Record<keyof Product, boolean>>, args: { data: any }) => {
  return useMutation<Product>({
    queryKey: ['CreateProduct', selection],
    queryFn: async () => {
      return await requestCreateProduct(selection, args);
    },
  });
};

export const useUpdateProduct = (selection: Partial<Record<keyof Product, boolean>>, args: { id: string, data: any }) => {
  return useMutation<Product>({
    queryKey: ['UpdateProduct', selection],
    queryFn: async () => {
      return await requestUpdateProduct(selection, args);
    },
  });
};

export const useCreateNews = (selection: Partial<Record<keyof News, boolean>>, args: { productId: string, data: any }) => {
  return useMutation<News>({
    queryKey: ['CreateNews', selection],
    queryFn: async () => {
      return await requestCreateNews(selection, args);
    },
  });
};

export const useUpdateNews = (selection: Partial<Record<keyof News, boolean>>, args: { id: string, data: any }) => {
  return useMutation<News>({
    queryKey: ['UpdateNews', selection],
    queryFn: async () => {
      return await requestUpdateNews(selection, args);
    },
  });
};

