import React, { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import Cookies from "universal-cookie";
import StatisticJournal from "../../components/count/StatisticJournal";
import Search from "../../components/Search";
import { fetchJournal } from "../../services/functions/journal";

function Journal() {
  const navigate = useNavigate();
  const cookies = new Cookies();
  const max_item = cookies.get("max-item");

  const [pageData, setPageData] = useState();
  const [data, setData] = useState([]);
  const [selectedJournal, setSelectedJournal] = useState();
  const [isDataEdited, setIsDataEdited] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const { page } = useParams();

  const fetchDataForEntry = async () => {
    setLoading(true);
    try {
      const entryData = await fetchJournal(page, max_item);
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
      <Search addOnClick={() => navigate(`/admin/journal/add`)} />
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
                <th>Siswa</th>
                <th>Perusahaan</th>
                <th>Divisi</th>
                <th>Tanggal Dikirim</th>
                <th>Verifikasi</th>
                <th>Aksi</th>
              </tr>
            </thead>
            <tbody className="box-content">
              {data.map((row, index) => (
                <tr key={row.id} className="border-t-2 border-base-300">
                  <td className="p-3 pb-2">{index + 1}</td>
                  <td>{row.student}</td>
                  <td>{row.company}</td>
                  <td>{row.division}</td>
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
                      className="btn btn-info btn-xs rounded-lg mr-2"
                      onClick={() => {
                        setSelectedJournal(index);
                        console.log(index);
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

        {selectedJournal != undefined && (
          <dialog open className="modal">
            <div className="modal-box">
              <img
                src={data[selectedJournal].img_url}
                className="w-full object-cover rounded-box"
                alt=""
              />
              <button
                type="button"
                className="btn btn-sm btn-circle btn-ghost bg-base-100 hover:bg-base-200 absolute right-2 top-2"
                onClick={() => setSelectedJournal(null)}
              >
                <span className="material-symbols-rounded">close</span>
              </button>
              <div className="flex gap-2 w-full max-w-xl">
                <div className="bg-base-100 py-4 rounded-btn flex flex-col items-start flex-1">
                  <div className="flex flex-row items-center mb-4">
                    <span className="material-symbols-rounded icon-size-24">
                      photo_camera
                    </span>
                    <span className="ml-2 text-lg font-bold">
                      {data[selectedJournal].activity}
                    </span>
                  </div>
                  <div className="grid grid-cols-2 w-full rounded-box overflow-hidden">
                    <div className="p-2 bg-base-200">Siswa</div>
                    <div className="p-2 bg-base-300">
                      {data[selectedJournal].student}
                    </div>
                    <div className="p-2 bg-base-300">Perusahaan</div>
                    <div className="p-2 bg-base-200">
                      {data[selectedJournal].company}
                    </div>
                    <div className="p-2 bg-base-200">Divisi</div>
                    <div className="p-2 bg-base-300">
                      {data[selectedJournal].division}
                    </div>
                    <div className="p-2 bg-base-300">Jam Mulai</div>
                    <div className="p-2 bg-base-200">
                      {data[selectedJournal].start_time}
                    </div>
                    <div className="p-2 bg-base-200">Jam Berakhir</div>
                    <div className="p-2 bg-base-300">
                      {data[selectedJournal].end_time}
                    </div>
                    <div className="p-2 bg-base-300">Tanggal Dikirim</div>
                    <div className="p-2 bg-base-200">
                      {new Date(
                        data[selectedJournal].created_at
                      ).toLocaleDateString()}
                    </div>
                  </div>
                </div>
              </div>
              <button type="submit" className="btn btn-success w-full">
                Verify
              </button>
            </div>
          </dialog>
        )}
      </div>
    </>
  );
}

export default Journal;
