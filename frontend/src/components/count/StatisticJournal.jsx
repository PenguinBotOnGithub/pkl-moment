import React, { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import Cookies from "universal-cookie";
import { fetchData } from "../../services";

function StatisticJournal({ entryCount }) {
  const navigate = useNavigate();
  const cookies = new Cookies(null, { path: "/" });
  const token = cookies.get("access-token");
  const [tenureData, setTenureData] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  const fetchDataForTenure = async () => {
    try {
      const response = await fetchData(`/api/tenure?page=0&size=0`);
      setTenureData(response.data);
      setError(null);
    } catch (err) {
      setError(err.message);
      setTenureData([]);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchDataForTenure();
  }, []);

  return (
    <div className="flex gap-2">
      <div className="overflow-hidden relative bg-base-100 p-4 rounded-box flex flex-col items-start flex-1">
        <span className="z-10 text-left">Total jurnal</span>
        <span className="z-10 text-4xl font-bold">{entryCount}</span>
        <span className="absolute -rotate-12 -right-10 -bottom-16 icon-size-164 material-symbols-rounded text-primary opacity-20">
          description
        </span>
      </div>
      <button
        className="overflow-hidden relative bg-base-100 p-4 rounded-box flex flex-col items-start flex-1 hover:bg-base-300 ease-in-out duration-150"
        onClick={() => navigate("/admin/journal/tenure")}
      >
        <span className="z-10 text-left">Total siswa yang PKL</span>
        <span className="z-10 text-4xl font-bold">
          {tenureData.total_items}
        </span>
        <span className="absolute -rotate-12 -right-10 -bottom-16 icon-size-164 material-symbols-rounded text-primary opacity-20">
          supervisor_account
        </span>
      </button>
      <button className="bg-base-100 p-4 rounded-box flex flex-col justify-center items-center flex-0">
        Tahun ajaran
        <span className="z-10 text-2xl font-bold">2023/2024</span>
      </button>
    </div>
  );
}

export default StatisticJournal;
