import Cookies from "universal-cookie";

const cookies = new Cookies();
const token = cookies.get("access-token");

const BASE_URL = 'http://127.0.0.1:8000';

export const fetchData = async (url, options = {}) => {
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

export const fetchDataWrapper = async (url, setter, transform = (data) => data) => {
  try {
    const data = await fetchData(url);
    setter(transform(data.data.items));
  } catch (err) {
    alert(err);
    setter([]);
  }
};

export const login = async (credentials) => {
  return await fetchData("/api/auth/login", {
    method: "POST",
    body: JSON.stringify(credentials),
  });
};

export default { fetchData, login, fetchDataWrapper };