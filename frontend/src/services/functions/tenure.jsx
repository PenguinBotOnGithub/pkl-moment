import { fetchData } from "..";

export const fetchTenure = async (page, size) => {
  return await fetchData(`/api/tenure?page=${page}&size=${size}`);
};

export const updateTenure = async (tenureId,body) => {
  return await fetchData(`/api/tenure/${tenureId}/advisor/add`, {
    body: JSON.stringify(body),
    method: "PATCH",
  });
};

export default { fetchTenure, updateTenure };
