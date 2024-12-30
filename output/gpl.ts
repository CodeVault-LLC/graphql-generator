import { requestMe, requestUser, requestProducts, requestProduct, requestNewsByProduct, requestNewsStatisticsByProductId, requestNewsById, requestLogin, requestCreateProduct, requestUpdateProduct, requestCreateNews, requestUpdateNews } from './resources';
import { useQuery, useMutation } from '@tanstack/react-query';
import { User, Product, News, NewsStatistics, Token } from './gpl.d';export const useMe = (selection: Partial<Record<keyof User, boolean>>, ) => {
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

export const useLogin = (selection: Partial<Record<keyof Token, boolean>>) => {
  return useMutation<Token, unknown, { email: string, password: string }>({
    mutationKey: ['Login', selection],
    mutationFn: async (args) => {
      return await requestLogin(selection, args);
    },
  });
};

export const useCreateProduct = (selection: Partial<Record<keyof Product, boolean>>) => {
  return useMutation<Product, unknown, { data: any }>({
    mutationKey: ['CreateProduct', selection],
    mutationFn: async (args) => {
      return await requestCreateProduct(selection, args);
    },
  });
};

export const useUpdateProduct = (selection: Partial<Record<keyof Product, boolean>>) => {
  return useMutation<Product, unknown, { id: string, data: any }>({
    mutationKey: ['UpdateProduct', selection],
    mutationFn: async (args) => {
      return await requestUpdateProduct(selection, args);
    },
  });
};

export const useCreateNews = (selection: Partial<Record<keyof News, boolean>>) => {
  return useMutation<News, unknown, { productId: string, data: any }>({
    mutationKey: ['CreateNews', selection],
    mutationFn: async (args) => {
      return await requestCreateNews(selection, args);
    },
  });
};

export const useUpdateNews = (selection: Partial<Record<keyof News, boolean>>) => {
  return useMutation<News, unknown, { id: string, data: any }>({
    mutationKey: ['UpdateNews', selection],
    mutationFn: async (args) => {
      return await requestUpdateNews(selection, args);
    },
  });
};

