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

export const updateDepartment = async (departmentId, body) => {
  fetchData(`/api/class/${departmentId}/update`, {
    body: JSON.stringify(body),
    method: "PATCH",
  });
};

export default { addDepartment, fetchDepartment, updateDepartment };