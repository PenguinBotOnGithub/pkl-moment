import React, { useEffect, useState } from "react";
import Cookies from "universal-cookie";
import Search from "../../../components/Search";
import { useNavigate, useParams } from "react-router-dom";
import StatisticStudent from "../../../components/count/StatisticStudent";
import { fetchData, fetchDataWrapper } from "../../../services";
import Dropdown from "../../../components/Dropdown";
import { fetchStudents, updateStudent } from "../../../services/functions/students";

function Student() {
  const cookies = new Cookies(null, { path: "/" });
  const token = cookies.get("access-token");
  const [data, setData] = useState([]);
  const [classData, setClassData] = useState([]);
  const [isDataEdited, setIsDataEdited] = useState([]);
  const navigate = useNavigate();
  const [pageData, setPageData] = useState();
  const { page } = useParams();

  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    fetchDataWrapper(`/api/class`, setClassData);
  }, []);

  useEffect(() => {
    fetchDataForStudents();
  }, [page]);

  const fetchDataForStudents = async () => {
    try {
      const response = await fetchStudents(page, 10);
      if (response.status != "success") {
        throw new Error(`HTTP error: Status ${response.status}`);
      }
      setPageData(response.data);
      setData(response.data.items);
      setIsDataEdited(response.data.items.map(() => false));
      setError(null);
      console.log(pageData);
    } catch (err) {
      setError(err.message);
      setData([]);
    } finally {
      setLoading(false);
    }
  };

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

  function handlePageChange(index) {
    navigate(`/admin/entries/student/${index}`);
  }

  return (
    <>
      <Search addOnClick={() => {navigate("/admin/entries/student/add")}} />
      <StatisticStudent entryCount={data.length}/>      
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
                  displayFields={["class"]}
                  searchField="class"
                  setSelectedValue={(selectedValue) =>
                    updateStudent(row.id, {class_id: selectedValue})
                  }
                  defaultValue={row.class.grade + " " + row.class.department + "-" + row.class.number}
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
