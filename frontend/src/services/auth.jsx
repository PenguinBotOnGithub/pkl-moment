import fetchData from './index';

export const login = async (credentials) => {
  return await fetchData('/auth/login', {
    method: 'POST',
    body: JSON.stringify(credentials),
  });
};

export const signup = async (userData) => {
  return await fetchData('/auth/register', {
    method: 'POST',
    body: JSON.stringify(userData),
  });
};
