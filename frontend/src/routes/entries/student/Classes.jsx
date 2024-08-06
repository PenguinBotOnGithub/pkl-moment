import React, { useEffect, useState } from "react";
import Cookies from "universal-cookie";
import host from "../../../assets/strings/host";
import Search from "../../../components/Search";
import { useNavigate } from "react-router-dom";
import { fetchData } from "../../../services";

function Classes() {
  const cookies = new Cookies(null, { path: "/" });
  const token = cookies.get("access-token");
  const [data, setData] = useState([]);
  const [isDataEdited, setIsDataEdited] = useState([]);
  const navigate = useNavigate();
  const [pageData, setPageData] = useState();

  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  const fetchDataForClasses = async () => {
    try {
      const response = await fetch(`${host}/api/class?page=0&size=10`, {
        headers: {
          Authorization: token,
        },
      });
      if (!response.ok) {
        throw new Error(`HTTP error: Status ${response.status}`);
      }
      let classesData = await response.json();
      setPageData(classesData.data);
      setData(classesData.data.items);
      setIsDataEdited(classesData.data.items.map(() => false));
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
  }, []);

  const deleteClass = async (id) => {
    try {
      const response = await fetch(`${host}/api/class/${id}`, {
        headers: {
          Authorization: token,
        },
        method: "DELETE",
      });
      if (!response.ok) {
        throw new Error(`HTTP error: Status ${response.status}`);
      }
      await response.json();
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
    try {
      const response = await fetch(`${host}/api/class/${id}`, {
        method: "PATCH",
        headers: {
          Authorization: token,
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          grade: data[index].grade,
          department: data[index].department,
          number: data[index].number,
        }),
      });

      if (!response.ok) {
        throw new Error(`HTTP error: Status ${response.status}`);
      }

      let result = await response.json();

      if (result.status === "success") {
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
      <Search addOnClick={() => navigate("/admin/entries/student/classes/add")} />

      <div className="overflow-x-auto">
        <table className="table bg-base-100 border-0 overflow-hidden rounded-lg">
          <thead className="bg-base-300">
            <tr className="border-0">
              <th>No</th>
              <th>Grade</th>
              <th>Department</th>
              <th>Number</th>
              <th>Action</th>
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
                  />
                </td>
                <td>
                  <input
                    type="text"
                    value={row.department}
                    className="w-full"
                    style={{
                      backgroundColor: "transparent",
                      border: "none",
                      outline: "none",
                    }}
                    onChange={(e) =>
                      handleInputChange(index, "department", e.target.value)
                    }
                  />
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
                  />
                </td>
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
    </>
  );
}

export default Classes;
