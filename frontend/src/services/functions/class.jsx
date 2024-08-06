import { fetchData } from "..";

export const classTo = async (entryId, body) => {
  try {
    const response = await fetchData(`/api/class/create`, {
      body: JSON.stringify(body),
      method: "POST",
    });

    if (response.status !== "success") {
      alert(`Failed to create class with ID ${entryId}`);
    }
    return response;
  } catch (error) {
    alert(`Something went wrong: ${error.message}`);
    return null;
  }
};

export default { classTo };
