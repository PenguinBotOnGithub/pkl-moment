import React, { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import Cookies from "universal-cookie";
import host from "../../../assets/strings/host";
import Dropdown from "../../../components/Dropdown";
import { fetchData } from "../../../services";
import { fetchDepartment } from "../../../services/functions/department";

function ClassAdd() {
  const [rows, setRows] = useState([{ grade: "", department: "", number: "" }]);
  const [departments, setDepartments] = useState([]);
  const cookies = new Cookies();
  const token = cookies.get("access-token");
  const navigate = useNavigate();

  useEffect(() => {
    const fetchDataWrapper = async (
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

    fetchDataWrapper(`/api/department?page=0&size=1000`, setDepartments);
  }, []);

  const handleSubmit = async (formData) => {
    try {
      const response = await fetchData(`/api/class/create`, {
        method: "POST",
        body: JSON.stringify({
          grade: parseInt(formData.grade),
          department_id: formData.department,
          number: parseInt(formData.number),
        }),
      });
      if (response.status === "success") {
        navigate("/admin/entries/student/class");
      } else {
        alert("Submission failed");
      }
    } catch (error) {
      alert("Something went wrong");
    }
  };

  const execBulkPost = async () => {
    console.log(rows);
    for (const row of rows) {
      if (
        row.grade.trim() === "" ||
        row.department === undefined ||
        row.number.trim() === ""
      ) {
        alert("Please fill every data before submitting");
        return;
      } else {
        await handleSubmit(row);
      }
    }
  };

  const handleInputChange = (index, event) => {
    const { name, value } = event.target;
    const newRows = [...rows];
    newRows[index][name] = value;
    setRows(newRows);
  };

  const handleDropdownChange = (index, selectedValue) => {
    const newRows = [...rows];
    newRows[index].department = selectedValue;
    setRows(newRows);
  };

  const addRow = () => {
    setRows([...rows, { grade: "", department: "", number: "" }]);
  };

  const deleteRow = (index) => {
    const newRows = [...rows];
    newRows.splice(index, 1);
    setRows(newRows);
  };

  return (
    <div className="overflow-x-auto">
      <table className="table bg-base-100 border-0 overflow-hidden rounded-lg">
        <thead className="bg-base-300">
          <tr className="border-0">
            <th className="w-0">No</th>
            <th>Grade</th>
            <th>Department</th>
            <th>Class Number</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody className="box-content">
          {rows.map((row, index) => (
            <tr key={index} className="border-t-2 border-base-300">
              <td>{index + 1}</td>
              <td>
                <input
                  type="text"
                  name="grade"
                  value={row.grade}
                  onChange={(event) => handleInputChange(index, event)}
                  style={{
                    backgroundColor: "transparent",
                    border: "none",
                    outline: "none",
                  }}
                />
              </td>
              <td>
                <Dropdown
                  size="sm"
                  items={departments}
                  displayFields={["name"]}
                  searchField="name"
                  setSelectedValue={(selectedValue) =>
                    handleDropdownChange(index, selectedValue)
                  }
                />
              </td>
              <td>
                <input
                  type="text"
                  name="number"
                  value={row.number}
                  onChange={(event) => handleInputChange(index, event)}
                  style={{
                    backgroundColor: "transparent",
                    border: "none",
                    outline: "none",
                  }}
                />
              </td>
              <td>
                <button
                  className="btn btn-error btn-xs rounded-lg mr-2"
                  onClick={() => deleteRow(index)}
                >
                  Delete
                </button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>

      <div className="flex justify-end mt-2 gap-2">
        <div className="btn btn-neutral btn-sm" onClick={addRow}>
          Add Row
        </div>
        <div className="btn btn-success btn-sm" onClick={execBulkPost}>
          Submit
        </div>
      </div>
      
    </div>
  );
}

export default ClassAdd;
