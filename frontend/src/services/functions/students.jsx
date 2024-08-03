import { fetchData } from "..";

export const assignStudentToLetter = async (entryId, body) => {
  try {
    const response = await fetchData(`/api/letters/${entryId}/student/add`, {
      body: JSON.stringify(body),
      method: "POST",
    });

    if (response.status !== "success") {
      alert(`Failed to add student with ID ${body.student_id}`);
    }
    return response;
  } catch (error) {
    alert(`Something went wrong: ${error.message}`);
    return null;
  }
};

export default { assignStudentToLetter };
