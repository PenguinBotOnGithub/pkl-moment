import { fetchData } from "..";

export const department = async (body) => {
  fetchData(`/api/department/create`, {
    body: JSON.stringify(body),
    method: "POST",
  });
};

export default { department };