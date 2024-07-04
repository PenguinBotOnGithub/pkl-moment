import React, { useEffect, useState } from "react";
import host from "../../assets/strings/host";
import Cookies from "universal-cookie";
import { useNavigate, useParams } from "react-router-dom";

function WaveTable() {
  const [data, setData] = useState([]);
  const [pageData, setPageData] = useState();
  const { page } = useParams();
  const navigate = useNavigate();
  const cookies = new Cookies(null, { path: "/" });
  const token = cookies.get("access-token");
  const [selectedId, setSelectedId] = useState();

  const fetchWaveData = async () => {
    try {
      const response = await fetch(`${host}/api/wave?page=${page}&size=10`, {
        headers: {
          Authorization: token,
        },
      });
      if (!response.ok) {
        throw new Error(`HTTP error: Status ${response.status}`);
      }
      const waveData = await response.json();
      setData(waveData.data.items.map((wave, index) => ({
        ...wave,
        waveyear: `${new Date(wave.start_date).getFullYear()}/${new Date(wave.end_date).getFullYear()}`,
        start_date: new Date(wave.start_date).toLocaleDateString('id-ID', { day: 'numeric', month: 'long', year: 'numeric' }),
        end_date: new Date(wave.end_date).toLocaleDateString('id-ID', { day: 'numeric', month: 'long', year: 'numeric' })
      })));
      setPageData(waveData.data);
    } catch (err) {
      alert("Something went wrong: " + err);
      setData([]);
    }
  };

  const handleSelect = (id) => {
    cookies.set("selected-wave", id);
    setSelectedId(id);
  };

  useEffect(() => {
    fetchWaveData();
    setSelectedId(cookies.get("selected-wave"));
  }, [page]);

  const handlePageChange = (newPage) => {
    navigate(`/admin/entries/wave/${newPage}`);
  };

  const onDelete = async (index) => {
    try {
      const response = await fetch(`${host}/api/wave/${index}/delete`, {
        method: "DELETE",
        headers: {
          Authorization: token,
        },
      });
      if (!response.ok) {
        throw new Error(`HTTP error: Status ${response.status}`);
      }
      fetchWaveData();
      cookies.remove("selected-wave");
    } catch (err) {
      alert("something went wrong:" + err);
    } finally {
    }
  };

  return (
    <div className="overflow-x-auto">
      <table className="table bg-base-100 border-0 overflow-hidden rounded-lg">
        <thead className="bg-neutral">
          <tr className="border-0">
            <th className="w-0">No</th>
            <th>Tahun Pembelajaran</th>
            <th>Start Date</th>
            <th>End Date</th>
            <th>Terpilih</th>
            <th>Aksi</th>
          </tr>
        </thead>
        <tbody className="box-content">
          {data.map((row, index) => (
            <tr key={row.id} className="border-t-2 border-neutral">
              <td>{index + 1}</td>
              <td>{row.waveyear}</td>
              <td>{row.start_date}</td>
              <td>{row.end_date}</td>
              <td>
                {selectedId === row.id ? (
                  <span className="opacity-60">Terpilih</span>
                ) : (
                  <button
                    className="btn btn-success btn-xs rounded-lg mr-2"
                    onClick={() => handleSelect(row.id)}
                  >
                    Pilih
                  </button>
                )}
              </td>
              <td>
                <button className="btn btn-info btn-xs rounded-lg mr-2">
                  Edit
                </button>
                <button className="btn btn-error btn-xs rounded-lg" onClick={()=>{onDelete(row.id)}}>
                  Delete
                </button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
      {pageData && (
        <div className="flex justify-center items-center gap-2 mt-4">
          <button
            className="flex-none btn bg-base-100"
            onClick={() => handlePageChange(pageData.page - 1)}
            disabled={pageData.page === 1}
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
                  pageData.page === index ? "btn-neutral" : ""
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
            disabled={pageData.page === pageData.num_pages}
          >
            <span className="material-symbols-rounded icon-size-20">
              arrow_forward
            </span>
          </button>
        </div>
      )}
    </div>
  );
}

export default WaveTable;
