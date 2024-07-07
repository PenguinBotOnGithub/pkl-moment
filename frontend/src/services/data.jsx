import fetchData from './index';

export const getUsers = async () => {
  return await fetchData('/users');
};

export const getUserById = async (userId) => {
  return await fetchData(`/users/${userId}`);
};
