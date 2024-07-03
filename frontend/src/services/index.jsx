import Cookies from "universal-cookie";

const cookies = new Cookies();
const token = cookies.get("access-token");

const BASE_URL = 'http://127.0.0.1:8000';

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

export const exportEntry = async (entryType, id) => {
  const response = await fetch(`${BASE_URL}/api/${entryType}/${id}/pdf`, {
    headers: { Authorization: token ? `Bearer ${token}` : '' },
  });

  if (!response.ok) throw new Error(`HTTP error! Status: ${response.status}`);

  const blob = await response.blob();
  return blob;
};

export default { login, fetchEntries, deleteEntry, exportEntry };
