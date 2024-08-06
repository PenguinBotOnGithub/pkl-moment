import { fetchData } from "..";

export const addDepartment = async (body) => {
  fetchData(`/api/department/create`, {
    body: JSON.stringify(body),
    method: "POST",
  });
};

export const fetchDepartment = async () => {
  fetchData(`/api/department`);
};

export default { addDepartment, fetchDepartment };