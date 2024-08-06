import { fetchData } from "..";

export const fetchJournal = async (page, size) => {
  return await fetchData(`/api/journal?page=${page}&size=${size}`);
};

export const deleteJournal = async (id) => {
  return await fetchData(`/api/journal/${id}/delete`, {
    method: 'DELETE',
  });
};

export const exportJournal = async (id, lettertype) => {
  try {
    const response = fetchData(`/api/journal/${id}/pdf/${lettertype}`);
    let bin = [];
    for await (const chunk of response) {
      bin.push(chunk);
    }
    let blob = new Blob(bin, { type: "application/pdf" });
    downloadBlob(blob, "pkl.pdf");
  } catch (err) {
    console.log(err);
  }
};

export default { fetchJournal, deleteJournal, exportJournal };