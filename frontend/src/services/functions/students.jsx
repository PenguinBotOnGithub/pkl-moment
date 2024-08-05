import { fetchData } from "..";

export const fetchStudents = async (page, size) => {
  return await fetchData(`/api/student?page=${page}&size=${size}`);
};

export const assignStudentToLetter = async (entryId, body) => {
  fetchData(`/api/letters/${entryId}/student/add`, {
    body: JSON.stringify(body),
    method: "POST",
  });
};

export const updateStudent = async (studentId, body) => {
  fetchData(`/api/student/${studentId}/update`, {
    body: JSON.stringify(body),
    method: "PATCH",
  });
};

export default { assignStudentToLetter };
