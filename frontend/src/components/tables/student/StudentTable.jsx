import React, { useEffect, useState } from "react";
import Cookies from "universal-cookie";
import { fetchData } from "../../../services";

function StudentTable() {
  const cookies = new Cookies(null, { path: "/" });
  const token = cookies.get("access-token");
  const [data, setData] = useState([]);
  const [isDataEdited, setIsDataEdited] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  const fetchDataForStudents = async () => {
    try {
      const response = await fetchData(`/api/student?page=0`);
      setData(response.data.items);
      setIsDataEdited(response.data.items.map(() => false));
      setError(null);
    } catch (err) {
      setError(err.message);
      setData([]);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchDataForStudents();
  }, []);

  const deleteStudent = async (index) => {
    try {
      await fetchData(`/api/student/${index}/delete`, {
        method: "DELETE",
      });
      setError(null);
      fetchDataForStudents();
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
    // Implement the logic to save the changes to the server
    console.log("Form data submitted:", data[index]);
    await fetchData(`/api/student/${id}/update`, {
      method: "PATCH",
      body: JSON.stringify({
        name: data[index].name,
        class: data[index].class,
        nis: data[index].nis,
      }),
    })
      .then((result) => {
        if (result.status === "success") {
          fetchDataForStudents();
        }
      })
      .catch(() => {
        alert("Something went wrong");
        fetchDataForStudents();
      });

    const newIsDataEdited = [...isDataEdited];
    newIsDataEdited[index] = false;
    setIsDataEdited(newIsDataEdited);
  };

  const cancelChanges = () => {
    fetchDataForStudents();
  };

  return (
    <div className="overflow-x-auto">
      <table className="table bg-base-100 border-0 overflow-hidden rounded-lg">
        <thead className="bg-base-300">
          <tr className="border-0">
            <th>No</th>
            <th>Nama Siswa</th>
            <th>Kelas</th>
            <th>NIS</th>
            <th>Aksi</th>
          </tr>
        </thead>
        <tbody className="box-content">
          {data.map((row, index) => (
            <tr key={row.id} className="border-t-2 border-base-300">
              <td>{index + 1}</td>
              <td>
                <input
                  type="text"
                  value={row.name}
                  className="w-full"
                  style={{
                    backgroundColor: "transparent",
                    border: "none",
                    outline: "none",
                  }}
                  onChange={(e) =>
                    handleInputChange(index, "name", e.target.value)
                  }
                />
              </td>
              <td>
                <input
                  type="text"
                  value={row.class}
                  className="w-full"
                  style={{
                    backgroundColor: "transparent",
                    border: "none",
                    outline: "none",
                  }}
                  onChange={(e) =>
                    handleInputChange(index, "class", e.target.value)
                  }
                />
              </td>
              <td>
                <input
                  type="text"
                  value={row.nis}
                  className="w-full"
                  style={{
                    backgroundColor: "transparent",
                    border: "none",
                    outline: "none",
                  }}
                  onChange={(e) =>
                    handleInputChange(index, "nis", e.target.value)
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
                      onClick={() => cancelChanges()}
                    >
                      Cancel
                    </button>
                  </>
                )}
                <button
                  className="btn btn-error btn-xs rounded-lg"
                  onClick={() => deleteStudent(row.id)}
                >
                  Delete
                </button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
      <div className="flex justify-center items-center gap-2 mt-4">
        <button className="flex-none btn bg-base-100">
          <span className="material-symbols-rounded icon-size-20">
            arrow_back
          </span>
        </button>
        <div className="join flex gap-2">
          <button className="join-item btn">1</button>
          <button className="join-item btn">2</button>
          <button className="join-item btn opacity-50">...</button>
          <button className="join-item btn">99</button>
          <button className="join-item btn">100</button>
        </div>
        <button className="flex-none btn bg-base-100">
          <span className="material-symbols-rounded icon-size-20">
            arrow_forward
          </span>
        </button>
      </div>
    </div>
  );
}

export default StudentTable;
