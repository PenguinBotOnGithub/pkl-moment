import React, { useEffect, useState } from "react";
import Cookies from "universal-cookie";
import Search from "../../../components/Search";
import { useNavigate } from "react-router-dom";
import { fetchData, fetchDataWrapper } from "../../../services";
import Dropdown from "../../../components/Dropdown";

function Class() {
  const cookies = new Cookies(null, { path: "/" });
  const token = cookies.get("access-token");
  const role = cookies.get("role");
  const [data, setData] = useState([]);
  const [departmentData, setDepartmentData] = useState([]);
  const [isDataEdited, setIsDataEdited] = useState([]);
  const navigate = useNavigate();
  const [pageData, setPageData] = useState();
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  const parseClassString = (classString) => {
    const [grade, departmentWithNumber] = classString.split(" ");
    const [department, number] = departmentWithNumber.split("-");
    return { grade, department, number };
  };

  const fetchDataForClasses = async () => {
    try {
      const response = await fetchData(`/api/class?page=0&size=10`);
      const items = response.data.items.map((item) => ({
        id: item.id,
        ...parseClassString(item.class),
      }));
      setPageData(response.data);
      setData(items);
      setIsDataEdited(items.map(() => false));
      setError(null);
    } catch (err) {
      setError(err.message);
      setData([]);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchDataForClasses();
    fetchDataWrapper(`/api/department`, setDepartmentData);
  }, []);

  const deleteClass = async (id) => {
    try {
      await fetchData(`/api/class/${id}/delete`, {
        method: "DELETE",
      });
      setError(null);
      fetchDataForClasses();
    } catch (err) {
      setError(err.message);
    }
  };

  const handleInputChange = (index, field, value) => {
    const newData = [...data];
    newData[index][field] = value;
    setData(newData);

    const newIsDataEdited = [...isDataEdited];
    newIsDataEdited[index] = true;
    setIsDataEdited(newIsDataEdited);
  };

  const saveChanges = async (index, id) => {
    console.log("Form data submitted:", data[index]);
    try {
      const response = await fetchData(`/api/class/${id}/update`, {
        method: "PATCH",
        body: JSON.stringify({
          grade: parseInt(data[index].grade),
          department_id: parseInt(data[index].department),
          number: parseInt(data[index].number),
        }),
      });

      if (response.status === "success") {
        fetchDataForClasses();
      } else {
        alert("Failed to save changes");
      }
    } catch (err) {
      alert(`Something went wrong: ${err.message}`);
    } finally {
      const newIsDataEdited = [...isDataEdited];
      newIsDataEdited[index] = false;
      setIsDataEdited(newIsDataEdited);
    }
  };

  const handlePageChange = (page) => {
    // Implement the page change logic
  };

  return (
    <>
      <Search
        addOnClick={role === "secretary" ? () => navigate("/admin/entries/student/class/add") : null}
      />

      <div className="overflow-x-auto">
        <table className="table bg-base-100 border-0 overflow-hidden rounded-lg">
          <thead className="bg-base-300">
            <tr className="border-0">
              <th>No</th>
              <th>Grade</th>
              <th>Department</th>
              <th>Number</th>
              {role === "secretary" && <th>Action</th>}
            </tr>
          </thead>
          <tbody className="box-content">
            {data.map((row, index) => (
              <tr key={row.id} className="border-t-2 border-base-300">
                <td>{index + 1}</td>
                <td>
                  <input
                    type="text"
                    value={row.grade}
                    className="w-full"
                    style={{
                      backgroundColor: "transparent",
                      border: "none",
                      outline: "none",
                    }}
                    onChange={(e) =>
                      handleInputChange(index, "grade", e.target.value)
                    }
                    disabled={role !== "secretary"}
                  />
                </td>
                <td>
                  {role === "secretary" ? (
                    <Dropdown
                      size="sm"
                      items={departmentData}
                      displayFields={["name"]}
                      searchField="name"
                      setSelectedValue={(selectedValue) =>
                        handleInputChange(index, "department", selectedValue)
                      }
                      defaultValue={row.department}
                    />
                  ) : (
                    row.department
                  )}
                </td>
                <td>
                  <input
                    type="text"
                    value={row.number}
                    className="w-full"
                    style={{
                      backgroundColor: "transparent",
                      border: "none",
                      outline: "none",
                    }}
                    onChange={(e) =>
                      handleInputChange(index, "number", e.target.value)
                    }
                    disabled={role !== "secretary"}
                  />
                </td>
                {role === "secretary" && (
                  <td>
                    {isDataEdited[index] && (
                      <>
                        <button
                          className="btn btn-success btn-xs rounded-lg mr-2"
                          onClick={() => saveChanges(index, row.id)}
                        >
                          Save
                        </button>
                        <button
                          className="btn btn-warning btn-xs rounded-lg mr-2"
                          onClick={() => fetchDataForClasses()}
                        >
                          Cancel
                        </button>
                      </>
                    )}
                    <button
                      className="btn btn-error btn-xs rounded-lg"
                      onClick={() => deleteClass(row.id)}
                    >
                      Delete
                    </button>
                  </td>
                )}
              </tr>
            ))}
          </tbody>
        </table>
        {pageData && (
          <div className="flex justify-center items-center gap-2 mt-2">
            <button
              className="flex-none btn bg-base-100"
              onClick={() => handlePageChange(pageData.page - 1)}
              disabled={pageData.page === 0}
            >
              <span className="material-symbols-rounded icon-size-20">
                arrow_back
              </span>
            </button>
            <div className="join flex">
              {[...Array(pageData.num_pages)].map((_, index) => (
                <button
                  key={index}
                  className={`join-item btn ${
                    pageData.page === index
                      ? "bg-primary text-primary-content"
                      : "bg-base-100"
                  }`}
                  onClick={() => handlePageChange(index)}
                >
                  {index + 1}
                </button>
              ))}
            </div>
            <button
              className="flex-none btn bg-base-100"
              onClick={() => handlePageChange(pageData.page + 1)}
              disabled={pageData.page === pageData.num_pages - 1}
            >
              <span className="material-symbols-rounded icon-size-20">
                arrow_forward
              </span>
            </button>
          </div>
        )}
      </div>
      <button onClick={() => console.log(data)}>debug</button>
    </>
  );
}

export default Class;