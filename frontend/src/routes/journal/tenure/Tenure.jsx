import React, { useEffect, useState } from "react";
import Cookies from "universal-cookie";
import host from "../../../assets/strings/host";
import Search from "../../../components/Search";
import { useNavigate, useParams } from "react-router-dom";
import StatisticStudent from "../../../components/count/StatisticStudent";
import { fetchData } from "../../../services";
import Dropdown from "../../../components/Dropdown";
import {
  fetchStudents,
  updateStudent,
} from "../../../services/functions/students";
import { fetchTenure } from "../../../services/functions/tenure";

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

  const fetchDataWrapper = async (url, setter, transform = (data) => data) => {
    try {
      const data = await fetchData(url);
      setter(transform(data.data.items));
    } catch (err) {
      alert(err);
      setter([]);
    }
  };

  useEffect(() => {
    fetchDataWrapper(`/api/class`, setClassData);
  }, []);

  const fetchDataForTenure = async () => {
    try {
      const response = await fetchTenure(page, 10);
      if (response.status != "success") {
        throw new Error(`HTTP error: Status ${response.status}`);
      }
      setPageData(response.data);
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
    fetchDataForTenure();
    console.log(data);
  }, [page]);

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
      fetchDataForTenure();
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
          fetchDataForTenure();
        }
      })
      .catch((err) => {
        alert("Something went wrong: " + err);
        fetchDataForTenure();
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
      <Search
        addOnClick={() => {
          navigate("/admin/entries/student/add");
        }}
      />
      <div className="overflow-x-auto">
        <table className="table bg-base-100 border-0 overflow-hidden rounded-lg">
          <thead className="bg-base-300">
            <tr className="border-0">
              <th className="pl-3 pb-2 pr-0 w-0">No</th>
              <th>Nama Siswa</th>
              <th>Pembimbing Sekolah</th>
              <th>Pembimbing Perusahaan</th>
            </tr>
          </thead>
          <tbody className="box-content">
            {data.map((row, index) => (
              <tr key={row.id} className="border-t-2 border-base-300">
                <td className="p-3 pb-2">{index + 1}</td>
                <td>
                  <span>{row.student}</span>
                </td>
                <td>
                  <span>{row.advisor_sch ? row.advisor_sch : <span className="opacity-50">null</span>}</span>
                </td>
                <td>
                  <span>{row.advisor_dudi ? row.advisor_dudi : <span className="opacity-50">null</span>}</span>
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
