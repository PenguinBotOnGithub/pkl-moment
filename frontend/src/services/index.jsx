import Cookies from "universal-cookie";
import host from "../assets/strings/host";

const cookies = new Cookies();
const token = cookies.get("access-token");

const BASE_URL = host;

export const fetchData = async (url, options = {}) => {
  try {

    const headers = {
      "Content-Type": "application/json", // Default Content-Type
      Authorization: token ? `Bearer ${token}` : "",
      ...options.headers, // Merge additional headers
    };

    // If options.headers contains a Content-Type, override the default
    if (options.headers && options.headers["Content-Type"]) {
      headers["Content-Type"] = options.headers["Content-Type"];
    }

    const response = await fetch(`${BASE_URL}${url}`, {
      ...options,
      headers,
    });

    if (!response.ok) {
      throw new Error(`HTTP error! Status: ${response.status}`);
    }
    return await response.json();
  } catch (error) {
    console.error("Fetch error:", error);
    throw error;
  }
};

export const fetchDataWrapper = async (
  url,
  setter,
  transform = (data) => data
) => {
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
