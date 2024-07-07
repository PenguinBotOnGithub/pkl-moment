import { useState, useEffect } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { fetchEntries } from "../../services";

const useEntryAddController = () => {
  const navigate = useNavigate();
  const { entry } = useParams();
  const token = ""; // Move token handling here if needed

  const [company, setCompany] = useState([]);
  const [advisers, setAdvisers] = useState([]);
  const [students, setStudents] = useState([]);

  useEffect(() => {
    // Fetch data on component mount
    fetchEntries("company", 0, 1000).then((data) => setCompany(data));
    fetchEntries("student", 0, 1000).then((data) => setStudents(data));

    // Example conditionally fetching advisers
    if (token) {
      fetchEntries("user", 0, 1000).then((data) =>
        setAdvisers(data.filter((user) => user.role === "advisor"))
      );
    }
  }, []);

  const handleOnSubmit = async (selectedCompany, selectedAdvisers, startDate, endDate) => {
    // Handle submission logic here
    // Example:
    try {
      const response = await fetch(`${host}/api/${entry}/create`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: token ? `Bearer ${token}` : '',
        },
        body: JSON.stringify({
          company_id: selectedCompany,
          adviser_id: selectedAdvisers,
          start_date: startDate,
          end_date: endDate
        }),
      });

      const result = await response.json();
      if (result.status === "success") {
        navigate("/admin/entries/0");
      } else {
        alert("Submission failed");
      }
    } catch (error) {
      alert("Something went wrong: " + error.message);
    }
  };

  return {
    company,
    advisers,
    students,
    handleOnSubmit,
  };
};

export default useEntryAddController;
