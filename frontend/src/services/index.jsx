import Cookies from "universal-cookie";
import host from "../assets/strings/host";

const cookies = new Cookies();
const token = cookies.get("access-token");

const BASE_URL = host;

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

export const login = async (credentials) => {
  return await fetchData("/api/auth/login", {
    method: "POST",
    body: JSON.stringify(credentials),
  });
};

export default { fetchData, login };
