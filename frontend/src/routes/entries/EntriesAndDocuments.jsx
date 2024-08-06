import React, { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import Cookies from "universal-cookie";
import Statistic from "../../components/count/Statistic";
import { fetchLetters } from "../../services/functions/letters";
import Search from "../../components/Search";

function EntriesAndDocument() {
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
    fetchDataForEntry();
  }, [page]);

  function handlePageChange(index) {
    navigate(`/admin/entries/${index}`);
  }

  return (
    <>
      <Search addOnClick={() => navigate(`/admin/entries/add`)} />
      <Statistic entryCount={pageData && pageData.total_items} />
      <div className="flex flex-col gap-2">
<<<<<<< HEAD
        <div className="flex justify-between items-center gap-2">
          <div
            role="tablist"
            className="tabs-boxed p-0 bg-base-100 gap-2 flex flex-row flex-nowrap"
          >
            {entryValue.map((entry, index) => (
              <button
                key={index}
                role="tab"
                onClick={() => handleSelectTab(index)}
                className={`tab hover:bg-base-300 ease-in-out duration-150 ${
                  currentEntry === index && `tab-active`
                }`}
              >
                {entry.charAt(0).toUpperCase() + entry.slice(1)}
              </button>
            ))}
          </div>
<<<<<<< HEAD
<<<<<<< HEAD
=======
          {/* <div className="flex gap-2">
            <button
              className={`btn btn-warning btn-sm text-black ${
                selectedRows.length === 0 ? "opacity-50 cursor-not-allowed" : ""
              }`}
              disabled={selectedRows.length === 0}
            >
              Export{<span className="hidden lg:block"> yang terpilih</span>}
            </button>
            <button
              className={`btn btn-error btn-sm text-black ${
                selectedRows.length === 0 ? "opacity-50 cursor-not-allowed" : ""
              }`}
              disabled={selectedRows.length === 0}
              onClick={() =>
                selectedRows.forEach((rowIndex) =>
                  deleteEntry(data[rowIndex].id)
                )
              }
            >
              Delete{<span className="hidden lg:block"> yang terpilih</span>}
            </button>
          </div> */}
>>>>>>> 49af8b3 (frontend/refactor: [AS] delete entries table and move to entries and document)
=======
>>>>>>> 291a5c7 (frontend/feat: [AS] themes, breadcrumb, simple pkl icon, UI fix, move company add table to company add)
        </div>
=======
>>>>>>> 249d054 (frontend/feat: [AS] Dropdown overhaul, search add UI fix, change user add table required from advisor to coordinator, entries and document cleaner code, entry add cleaner code, move journal to journal folder, index service export fetchData)
        {loading ? (
          <div>Loading...</div>
        ) : data.length === 0 ? (
          <div>No data</div>
        ) : (
<<<<<<< HEAD
<<<<<<< HEAD
          <table className="table bg-base-100 border-0 overflow-hidden rounded-box">
            <thead className="bg-base-300">
              <tr className="border-0">
                <th className="pl-3 pb-2 pr-0 w-0">No</th>
=======
          <table className="table bg-base-100 border-0 overflow-hidden rounded-lg">
            <thead className="bg-neutral">
              <tr className="border-0">
                <th className="pl-3 pb-2 pr-0 w-0">
                  {/* <label className="swap">
                    <input
                      type="checkbox"
                      onChange={(e) => {
                        if (e.target.checked) {
                          setSelectedRows(data.map((_, index) => index));
                        } else {
                          setSelectedRows([]);
                        }
                      }}
                      checked={selectedRows.length === data.length}
                    />
                    <span className="swap-off material-symbols-rounded">
                      check_box_outline_blank
                    </span>
                    <span className="swap-on material-symbols-rounded">
                      check_box
                    </span>
                  </label> */}
                  No
                </th>
>>>>>>> 49af8b3 (frontend/refactor: [AS] delete entries table and move to entries and document)
=======
          <table className="table bg-base-100 border-0 overflow-hidden rounded-box">
            <thead className="bg-base-300">
              <tr className="border-0">
                <th className="pl-3 pb-2 pr-0 w-0">No</th>
>>>>>>> 291a5c7 (frontend/feat: [AS] themes, breadcrumb, simple pkl icon, UI fix, move company add table to company add)
                <th>Pembimbing</th>
                <th>Perusahaan</th>
                <th>Tanggal Permintaan</th>
                <th>Verifikasi</th>
                <th>Aksi</th>
              </tr>
            </thead>
            <tbody className="box-content">
              {data.map((row, index) => (
<<<<<<< HEAD
<<<<<<< HEAD
                <tr key={row.id} className="border-t-2 border-base-300">
                  <td className="p-3 pb-2">{index + 1}</td>
=======
                <tr key={row.id} className="border-t-2 border-neutral">
                  <td className="p-3 pb-2">
                    {/* <label className="swap opacity-60">
                      <input
                        type="checkbox"
                        onChange={() => handleSelectRow(index)}
                        checked={selectedRows.includes(index)}
                      />
                      <span className="swap-off material-symbols-rounded">
                        check_box_outline_blank
                      </span>
                      <span className="swap-on material-symbols-rounded">
                        check_box
                      </span>
                    </label> */}
                    {index + 1}
                  </td>
>>>>>>> 49af8b3 (frontend/refactor: [AS] delete entries table and move to entries and document)
=======
                <tr key={row.id} className="border-t-2 border-base-300">
                  <td className="p-3 pb-2">{index + 1}</td>
>>>>>>> 291a5c7 (frontend/feat: [AS] themes, breadcrumb, simple pkl icon, UI fix, move company add table to company add)
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
<<<<<<< HEAD
<<<<<<< HEAD
                      className="btn btn-info btn-xs"
=======
                      className="btn btn-info btn-xs rounded-lg"
>>>>>>> 49af8b3 (frontend/refactor: [AS] delete entries table and move to entries and document)
=======
                      className="btn btn-info btn-xs"
>>>>>>> 291a5c7 (frontend/feat: [AS] themes, breadcrumb, simple pkl icon, UI fix, move company add table to company add)
                      onClick={() => {
                        navigate(
                          `/admin/entries/detail/${row.id}`
                        );
                      }}
                    >
                      Detail
                    </button>
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
=======
                    {/* <button
                      className="btn btn-error btn-xs rounded-lg"
                      onClick={() => deleteEntry(row.id)}
                    >
                      Delete
                    </button> */}
>>>>>>> 49af8b3 (frontend/refactor: [AS] delete entries table and move to entries and document)
=======
>>>>>>> 291a5c7 (frontend/feat: [AS] themes, breadcrumb, simple pkl icon, UI fix, move company add table to company add)
                    {row.verified && (
                      <button
                        className="btn btn-warning btn-xs"
                        onClick={() => onExport(row.id)}
                      >
                        Export
                      </button>
                    )}
=======
>>>>>>> 4e1c186 (frontend/feat: [AS] app change from entry type to simply detail, statistic student adjustment, settings theme removed, entries and document simpler code, entry fix export, entry add simpler code, student class edit (buggy) and pagination, simpler student service code, deleted unused codes)
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}

        {pageData && (
<<<<<<< HEAD
<<<<<<< HEAD
          <div className="flex justify-center items-center gap-2">
=======
          <div className="flex justify-center items-center gap-2 mt-4">
>>>>>>> 49af8b3 (frontend/refactor: [AS] delete entries table and move to entries and document)
=======
          <div className="flex justify-center items-center gap-2">
>>>>>>> 291a5c7 (frontend/feat: [AS] themes, breadcrumb, simple pkl icon, UI fix, move company add table to company add)
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
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
                    pageData.page === index ? "bg-primary text-primary-content" : "bg-base-100"
=======
                    pageData.page === index ? "bg-primary text-base-300" : ""
>>>>>>> 49af8b3 (frontend/refactor: [AS] delete entries table and move to entries and document)
=======
                    pageData.page === index ? "bg-primary text-primary-content" : "bg-base-100"
>>>>>>> 291a5c7 (frontend/feat: [AS] themes, breadcrumb, simple pkl icon, UI fix, move company add table to company add)
=======
                    pageData.page === index
                      ? "bg-primary text-primary-content"
                      : "bg-base-100"
>>>>>>> 249d054 (frontend/feat: [AS] Dropdown overhaul, search add UI fix, change user add table required from advisor to coordinator, entries and document cleaner code, entry add cleaner code, move journal to journal folder, index service export fetchData)
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

export default EntriesAndDocument;
