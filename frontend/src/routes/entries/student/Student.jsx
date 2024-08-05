import React, { useEffect, useState } from "react";
import Cookies from "universal-cookie";
import host from "../../../assets/strings/host";
import Search from "../../../components/Search";
import { useNavigate } from "react-router-dom";
import StatisticStudent from "../../../components/count/StatisticStudent";
import { fetchData } from "../../../services";
import Dropdown from "../../../components/Dropdown";
import { updateStudent } from "../../../services/functions/students";

function Student() {
  const cookies = new Cookies(null, { path: "/" });
  const token = cookies.get("access-token");
  const [data, setData] = useState([]);
  const [classData, setClassData] = useState([]);
  const [isDataEdited, setIsDataEdited] = useState([]);
  const navigate = useNavigate();
  const [pageData, setPageData] = useState();

  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    const fetchDataWrapper = async (url, setter, transform = (data) => data) => {
      try {
        const data = await fetchData(url);
        setter(transform(data.data.items));
      } catch (err) {
        alert(err);
        setter([]);
      }
    };

    fetchDataWrapper(`/api/class`, setClassData);
  }, []);

  const fetchDataForStudents = async () => {
    try {
      const response = await fetch(`${host}/api/student?page=0&size=1`, {
        headers: {
          Authorization: token,
        },
      });
      if (!response.ok) {
        throw new Error(`HTTP error: Status ${response.status}`);
      }
      let studentsData = await response.json();
      setPageData(studentsData.data);
      setData(studentsData.data.items);
      setIsDataEdited(studentsData.data.items.map(() => false));
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
      const response = await fetch(`${host}/api/student/${index}/delete`, {
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
    console.log("Form data submitted:", data[index]);
    await fetchData(`/api/student/${id}/update`, {
      method: "PATCH",
      body: JSON.stringify({
        name: data[index].name,
        nis: data[index].nis,
      }),
    })
      .then((result) => {
        if (result.status === "success") {
          fetchDataForStudents();
        }
      })
      .catch((err) => {
        alert("Something went wrong: "+err);
        fetchDataForStudents();
      });

    const newIsDataEdited = [...isDataEdited];
    newIsDataEdited[index] = false;
    setIsDataEdited(newIsDataEdited);
  };

  return (
    <>
      <Search addOnClick={() => {navigate("/admin/entries/student/add")}} />
      <StatisticStudent entryCount={data.total_items}/>      
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
                <Dropdown
                  size="sm"
                  items={classData}
                  displayFields={["grade", "department", "number"]}
                  searchField="department"
                  setSelectedValue={(selectedValue) =>
                    updateStudent(index+1, {class_id: selectedValue})
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
                        onClick={() => {fetchDataForStudents()}}
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

export default Student;
