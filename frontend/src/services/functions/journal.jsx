import { fetchData } from "..";

export const fetchJournal = async (page, size) => {
  return await fetchData(`/api/journal?page=${page}&size=${size}`);
};

export const deleteJournal = async (id) => {
  return await fetchData(`/api/journal/${id}/delete`, {
    method: "DELETE",
  });
};

export const verifyJournal = async (id, role) => {
  try {
    const response = fetchData(`/api/journal/${id}/verify/${role}`, {
      method: "PATCH",
    });
    return response;
  } catch (err) {
    console.log(err);
  }
};

export default { fetchJournal, deleteJournal, verifyJournal };
