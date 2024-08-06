import React, { useEffect, useState } from "react";
import Cookies from "universal-cookie";
import host from "../../../assets/strings/host";
import Search from "../../../components/Search";
import { useNavigate } from "react-router-dom";

function Department() {
  const cookies = new Cookies(null, { path: "/" });
  const token = cookies.get("access-token");
  const [data, setData] = useState([]);
  const navigate = useNavigate();
  const [pageData, setPageData] = useState();
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [isModalVisible, setIsModalVisible] = useState(false);
  const [newDepartmentName, setNewDepartmentName] = useState("");

  const fetchDataForDepartments = async (page = 0) => {
    try {
      setLoading(true);
      const response = await fetch(`${host}/api/department?page=${page}&size=10`, {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });
      if (!response.ok) {
        throw new Error(`HTTP error: Status ${response.status}`);
      }
      const departmentsData = await response.json();
      setPageData(departmentsData.data);
      setData(departmentsData.data.items);
      setError(null);
    } catch (err) {
      setError(err.message);
      setData([]);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchDataForDepartments();
  }, []);

  const deleteDepartment = async (id) => {
    try {
      const response = await fetch(`${host}/api/department/${id}`, {
        method: "DELETE",
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });
      if (!response.ok) {
        throw new Error(`HTTP error: Status ${response.status}`);
      }
      await response.json();
      setError(null);
      fetchDataForDepartments(pageData.page);
    } catch (err) {
      setError(err.message);
    }
  };

  const handlePageChange = (page) => {
    fetchDataForDepartments(page);
  };

  const addDepartment = async () => {
    if (!newDepartmentName.trim()) {
      setError("Department name cannot be empty.");
      return;
    }
    try {
      const response = await fetch(`${host}/api/department/create`, {
        method: "POST",
        headers: {
          Authorization: `Bearer ${token}`,
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ name: newDepartmentName }),
      });
      if (!response.ok) {
        const errorText = await response.text();
        throw new Error(`HTTP error: Status ${response.status}, Message: ${errorText}`);
      }
      const newDepartment = await response.json();
      setError(null);
      setData((prevData) => [...prevData, newDepartment.data]);
      setNewDepartmentName("");
      setIsModalVisible(false);
    } catch (err) {
      console.error(err.message);  // Log error to the console
      setError(err.message);
    }
  };

  return (
    <>
      <Search addOnClick={() => setIsModalVisible(true)} />

      {isModalVisible && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex justify-center items-center">
          <div className="bg-white p-5 rounded-lg relative w-80">
            <button
              type="button"
              className="btn btn-sm btn-circle btn-ghost absolute right-2 top-2"
              onClick={() => setIsModalVisible(false)}
            >
              <span className="material-symbols-rounded">close</span>
            </button>
            <h3 className="text-lg font-bold mb-4">Add New Department</h3>
            <input
              type="text"
              value={newDepartmentName}
              onChange={(e) => setNewDepartmentName(e.target.value)}
              className="input input-bordered w-full mb-4"
              placeholder="Department Name"
              required
            />
            <button className="btn btn-primary w-full" onClick={addDepartment}>
              Save
            </button>
          </div>
        </div>
      )}

      {loading ? (
        <div>Loading...</div>
      ) : (
        <>
          {error && <div className="text-red-500">{error}</div>}
          <div className="overflow-x-auto">
            <table className="table bg-base-100 border-0 overflow-hidden rounded-lg">
              <thead className="bg-base-300">
                <tr className="border-0">
                  <th>No</th>
                  <th>Jurusan</th>
                  <th>Action</th>
                </tr>
              </thead>
              <tbody className="box-content">
                {data.map((row, index) => (
                  <tr key={row.id} className="border-t-2 border-base-300">
                    <td>{index + 1}</td>
                    <td>{row.name}</td>
                    <td>
                      <button
                        className="btn btn-error btn-xs rounded-lg"
                        onClick={() => deleteDepartment(row.id)}
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
      )}
    </>
  );
}

export default Department;
