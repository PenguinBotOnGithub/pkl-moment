import { fetchData } from "..";

export const fetchTenure = async (page, size) => {
  return await fetchData(`/api/tenure?page=${page}&size=${size}`);
};

export const updateTenure = async (body) => {
  return await fetchData(`/api/tenure/update`, { body: JSON.stringify(body) });
};

export default { fetchTenure, updateTenure };
