import { fetchData } from "..";

export const fetchLetters = async (page, size) => {
  return await fetchData(`/api/letters?page=${page}&size=${size}`);
};

export const deleteEntry = async (id) => {
  return await fetchData(`/api/letters/${id}/delete`, {
    method: 'DELETE',
  });
};

export const exportEntry = async (id, lettertype) => {
  try {
    const response = fetchData(`/api/letters/${id}/pdf/${lettertype}`);
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

export default { fetchLetters, deleteEntry, exportEntry };