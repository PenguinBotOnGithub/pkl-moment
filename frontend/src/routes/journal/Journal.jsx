import React, { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import Cookies from "universal-cookie";
import StatisticJournal from "../../components/count/StatisticJournal";
import { fetchLetters } from "../../services/functions/letters";
import Search from "../../components/Search";

function Journal() {
  const navigate = useNavigate();
  const cookies = new Cookies();
  const max_item = cookies.get("max-item");

  const [currentEntry, setCurrentEntry] = useState(0);
  const entryValue = ["permohonan", "pengantaran", "penarikan"];
  const [pageData, setPageData] = useState();
  const [data, setData] = useState([]);
  const [isDataEdited, setIsDataEdited] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const { page } = useParams();

  const fetchDataForEntry = async () => {
    setLoading(true);
    try {
      const entryData = await fetchLetters(page, max_item);
      setData(entryData.data.items);
      setIsDataEdited(entryData.data.items.map(() => false));
      setPageData(entryData.data);
      setError(null);
    } catch (err) {
      console.log("Error fetching data: " + err);
      setError(err.message);
      setData([]);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    console.log(data);
    fetchDataForEntry();
  }, [page]);

  function handlePageChange(index) {
    navigate(`/admin/entries/${index}`);
  }

  return (
    <>
      <Search addOnClick={() => navigate(`/admin/entries/add`)} />
      <StatisticJournal entryCount={pageData && pageData.total_items} />
      <div className="flex flex-col gap-2">
        {loading ? (
          <div>Loading...</div>
        ) : data.length === 0 ? (
          <div>No data</div>
        ) : (
          <table className="table bg-base-100 border-0 overflow-hidden rounded-box">
            <thead className="bg-base-300">
              <tr className="border-0">
                <th className="pl-3 pb-2 pr-0 w-0">No</th>
                <th>Pembimbing</th>
                <th>Perusahaan</th>
                <th>Tanggal Permintaan</th>
                <th>Verifikasi</th>
                <th>Aksi</th>
              </tr>
            </thead>
            <tbody className="box-content">
              {data.map((row, index) => (
                <tr key={row.id} className="border-t-2 border-base-300">
                  <td className="p-3 pb-2">{index + 1}</td>
                  <td>{row.user}</td>
                  <td>{row.company}</td>
                  <td>{new Date(row.created_at).toLocaleDateString()}</td>
                  <td>
                    {row.verified ? (
                      <span className="opacity-60">Terverifikasi</span>
                    ) : (
                      <span>Belum Terversifikasi</span>
                    )}
                  </td>
                  <td className="flex flex-row flex-nowrap gap-2">
                    <button
                      className="btn btn-info btn-xs"
                      onClick={() => {
                        navigate(
                          `/admin/entries/detail/${row.id}`
                        );
                      }}
                    >
                      Detail
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}

        {pageData && (
          <div className="flex justify-center items-center gap-2">
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

export default Journal;
