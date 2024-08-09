import React, { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import Cookies from "universal-cookie";
import { fetchData } from "../../services";

function Statistic({ entryCount }) {
  const navigate = useNavigate();
  const cookies = new Cookies(null, { path: "/" });
  const token = cookies.get("access-token");
  const [companyData, setCompanyData] = useState([]);
  const [studentData, setStudentData] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  const fetchDataForStudents = async () => {
    try {
      const response = await fetchData(`/api/student?page=0&size=0`);
      if (response.status !== "success") {
        throw new Error(`HTTP error: Status ${response.status}`);
      }
      setStudentData(response.data);
      setError(null);
    } catch (err) {
      setError(err.message);
      setStudentData([]);
    } finally {
      setLoading(false);
    }
  };

  const fetchDataForCompanies = async () => {
    try {
      const response = await fetchData(`/api/company?page=0&size=0`);
      if (response.status !== "success") {
        throw new Error(`HTTP error: Status ${response.status}`);
      }
      setCompanyData(response.data);
      setError(null);
    } catch (err) {
      setError(err.message);
      setCompanyData([]);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchDataForCompanies();
    fetchDataForStudents();
  }, []);

  return (
    <div className="flex gap-2">
      <div className="overflow-hidden relative bg-base-100 p-4 rounded-box flex flex-col items-start flex-1">
        <span className="z-10 text-left">Total Entri</span>
        <span className="z-10 text-4xl font-bold">{entryCount}</span>
        <span className="absolute -rotate-12 -right-10 -bottom-16 icon-size-164 material-symbols-rounded text-primary opacity-20">
          description
        </span>
      </div>
      <button
        className="overflow-hidden relative bg-base-100 p-4 rounded-box flex flex-col items-start flex-1 hover:bg-base-300 ease-in-out duration-150"
        onClick={() => navigate("/admin/entries/company")}
      >
        <span className="z-10 text-left">Total Perusahaan</span>
        <span className="z-10 text-4xl font-bold">
          {companyData.total_items}
        </span>
        <span className="absolute -rotate-12 -right-10 -bottom-16 icon-size-164 material-symbols-rounded text-primary opacity-20">
          apartment
        </span>
      </button>
      <button
        className="overflow-hidden relative bg-base-100 p-4 rounded-box flex flex-col items-start flex-1 hover:bg-base-300 ease-in-out duration-150"
        onClick={() => navigate("/admin/entries/student")}
      >
        <span className="z-10 text-left">Total Siswa</span>
        <span className="z-10 text-4xl font-bold">
          {studentData.total_items}
        </span>
        <span className="absolute -rotate-12 -right-10 -bottom-16 icon-size-164 material-symbols-rounded text-primary opacity-20">
          person
        </span>
      </button>
      <button className="bg-base-100 p-4 rounded-box flex flex-col justify-center items-center flex-0">
        Tahun ajaran
        <span className="z-10 text-2xl font-bold">2023/2024</span>
      </button>
    </div>
  );
}

export default Statistic;
