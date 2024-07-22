import Cookies from "universal-cookie";
import host from "../assets/strings/host";

const cookies = new Cookies();
const token = cookies.get("access-token");

const BASE_URL = host;

const fetchData = async (url, options = {}) => {
  try {
    const response = await fetch(`${BASE_URL}${url}`, {
      ...options,
      headers: {
        'Content-Type': 'application/json',
        'Authorization': token ? `Bearer ${token}` : '',
        ...options.headers,
      },
    });
    if (!response.ok) {
      throw new Error(`HTTP error! Status: ${response.status}`);
    }
    return await response.json();
  } catch (error) {
    console.error('Fetch error:', error);
    throw error;
  }
};

export const login = async (formData) => {
  return await fetchData('/api/auth/login', {
    method: 'POST',
    body: JSON.stringify(formData),
  });
};

export const fetchEntries = async (entryType, page, size) => {
  return await fetchData(`/api/${entryType}?page=${page}&size=${size}`);
};

export const deleteEntry = async (entryType, id) => {
  return await fetchData(`/api/${entryType}/${id}/delete`, {
    method: 'DELETE',
  });
};

export const exportEntry = async (entryType, index) => {
  try {
    const response = await fetch(
      `${host}/api/${entryType}/${index}/pdf`,
      {
        headers: {
          "Content-Type": "application/json",
          Authorization: token,
        },
        method: "GET",
      }
    );
    let bin = [];
    for await (const chunk of response.body) {
      bin.push(chunk);
    }
    let blob = new Blob(bin, { type: "application/pdf" });
    downloadBlob(blob, "pkl.pdf");
  } catch (err) {
    console.log(err);
  }
};

export default { login, fetchEntries, deleteEntry, exportEntry };
