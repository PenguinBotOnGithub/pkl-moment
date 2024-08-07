import { fetchData } from "..";

export const classTo = async (body) => {
  fetchData(`/api/class/create`, {
    body: JSON.stringify(body),
    method: "POST",
  });
};

export const updateClass = async (classId, body) => {
  fetchData(`/api/class/${classId}/update`, {
    body: JSON.stringify(body),
    method: "PATCH",
  });
};

export default { classTo };
