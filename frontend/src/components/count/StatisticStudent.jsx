import React, { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import Cookies from "universal-cookie";
import host from "../../assets/strings/host";

function StatisticStudent({ entryCount }) {
  const navigate = useNavigate();
  const cookies = new Cookies(null, { path: "/" });
  const token = cookies.get("access-token");
  const [classData, setClassData] = useState([]);
  const [departmentData, setDepartmentData] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  const fetchDataForClasses = async () => {
    try {
      const response = await fetchData(`${host}/api/class?page=0`, {
        headers: {
          Authorization: token,
        },
      });
      if (!response.ok) {
        throw new Error(`HTTP error: Status ${response.status}`);
      }
      let classesData = await response.json();
      setClassData(classesData.data);
      setError(null);
    } catch (err) {
      setError(err.message);
      setClassData([]);
    } finally {
      setLoading(false);
    }
  };

  const fetchDataForDepartments = async () => {
    try {
      const response = await fetch(`${host}/api/department?page=0`, {
        headers: {
          Authorization: token,
        },
      });
      if (!response.ok) {
        throw new Error(`HTTP error: Status ${response.status}`);
      }
      let departmentsData = await response.json();
      setDepartmentData(departmentsData.data);
      setError(null);
    } catch (err) {
      setError(err.message);
      setDepartmentData([]);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchDataForClasses();
    fetchDataForDepartments();
  }, []);

  return (
    <div className="flex gap-2">
      <div className="overflow-hidden relative bg-base-100 p-4 rounded-box flex flex-col items-start flex-1">
        <span className="z-10 text-left">Total Siswa</span>
        <span className="z-10 text-4xl font-bold">{entryCount}</span>
        <span className="absolute -rotate-12 -right-10 -bottom-16 icon-size-164 material-symbols-rounded text-primary opacity-20">
          group
        </span>
      </div>
      <button
        className="overflow-hidden relative bg-base-100 p-4 rounded-box flex flex-col items-start flex-1 hover:bg-base-300 ease-in-out duration-150"
        onClick={() => navigate("/admin/entries/student/classes")}
      >
        <span className="z-10 text-left">Total Kelas</span>
        <span className="z-10 text-4xl font-bold">
          {classData.total_items}
        </span>
        <span className="absolute -rotate-12 -right-10 -bottom-16 icon-size-164 material-symbols-rounded text-primary opacity-20">
          school
        </span>
      </button>
      <button
        className="overflow-hidden relative bg-base-100 p-4 rounded-box flex flex-col items-start flex-1 hover:bg-base-300 ease-in-out duration-150"
        onClick={() => navigate("/admin/entries/student/department")}
      >
        <span className="z-10 text-left">Total Department</span>
        <span className="z-10 text-4xl font-bold">
          {departmentData.total_items}
        </span>
        <span className="absolute -rotate-12 -right-10 -bottom-16 icon-size-164 material-symbols-rounded text-primary opacity-20">
          manufacturing
        </span>
      </button>
      <button className="bg-base-100 p-4 rounded-box flex flex-col justify-center items-center flex-0">
        Tahun ajaran
        <span className="z-10 text-2xl font-bold">2023/2024</span>
      </button>
    </div>
  );
}

export default StatisticStudent;
