import { fetchData } from "..";

export const fetchTenure = async (page, size) => {
  return await fetchData(`/api/tenure?page=${page}&size=${size}`);
};

export default { fetchTenure };
