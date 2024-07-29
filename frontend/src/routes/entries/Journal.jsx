import React, { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import Cookies from "universal-cookie";
import Statistic from "../../components/count/StatisticJournal";
import { exportEntry, fetchEntries } from "../../services";
import Search from "../../components/Search";
import host from "../../assets/strings/host";

function Journal() {
  const navigate = useNavigate();
  const cookies = new Cookies();
  const role = cookies.get("role");
  const token = cookies.get("access-token");
  const max_item = cookies.get("max-item");

  const [selectedRows, setSelectedRows] = useState([]);
  const [currentEntry, setCurrentEntry] = useState(0);
  const entryValue = ["Terverivikasi", "Belum Terverivikasi", "Dari siswa..", "Dari siswa yang dibimbing.."];
  const [pageData, setPageData] = useState();
  const [data, setData] = useState([]);
  const [isDataEdited, setIsDataEdited] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [dataWave, setDataWave] = useState("");
  const { page } = useParams();

  // const fetchWaveData = async () => {
  //   try {
  //     const response = await fetch(`${host}/api/wave?page=0&size=1000`, {
  //       headers: {
  //         Authorization: token,
  //       },
  //     });
  //     if (!response.ok) {
  //       throw new Error(`HTTP error: Status ${response.status}`);
  //     }
  //     const waveData = await response.json();
  //     const selectedWaveId = cookies.get("selected-wave");
  //     const wave = waveData.data.items.find(
  //       (element) => element.id === parseInt(selectedWaveId)
  //     );

  //     if (wave) {
  //       setDataWave(
  //         `${new Date(wave.start_date).getFullYear()}/${new Date(
  //           wave.end_date
  //         ).getFullYear()}`
  //       );
  //     } else {
  //       setDataWave("No wave selected");
  //     }
  //   } catch (err) {
  //     alert("Something went wrong: " + err);
  //     setDataWave("");
  //   }
  // };

  const fetchDataForEntry = async (entryType) => {
    setLoading(true);
    try {
      const entryData = await fetchEntries(entryType, page, max_item);
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
    fetchDataForEntry(entryValue[currentEntry]);
  }, [currentEntry, page]);

  const deleteEntry = async (id) => {
    try {
      const response = await fetch(
        `${host}/api/${entryValue[currentEntry]}/${id}/delete`,
        {
          headers: {
            Authorization: token,
          },
          method: "DELETE",
        }
      );
      if (!response.ok) {
        throw new Error(`HTTP error: Status ${response.status}`);
      }
      await response.json();
      setError(null);
      fetchDataForEntry(entryValue[currentEntry]);
    } catch (err) {
      setError(err.message);
    }
  };

  const handleInputChange = (index, field, value) => {
    const newData = [...data];
    newData[index][field] = value;
    setData(newData);
  };

  function handleSelectRow(rowIndex) {
    if (selectedRows.includes(rowIndex)) {
      setSelectedRows(selectedRows.filter((index) => index !== rowIndex));
    } else {
      setSelectedRows([...selectedRows, rowIndex]);
    }
  }

  function handleSelectTab(index) {
    setCurrentEntry(index);
  }

  function onAddHandle() {
    navigate(`/admin/journal/${entryValue[currentEntry]}/add`);
  }

  function downloadBlob(blob, name = "file") {
    // Convert your blob into a Blob URL (a special url that points to an object in the browser's memory)
    const blobUrl = URL.createObjectURL(blob);

    // Create a link element
    const link = document.createElement("a");

    // Set link's href to point to the Blob URL
    link.href = blobUrl;
    link.download = name;

    // Append link to the body
    document.body.appendChild(link);

    // Dispatch click event on the link
    // This is necessary as link.click() does not work on the latest firefox
    link.dispatchEvent(
      new MouseEvent("click", {
        bubbles: true,
        cancelable: true,
        view: window,
      })
    );

    // Remove link from body
    document.body.removeChild(link);
  }

  const onExport = async (index) => {
    exportEntry();
  };

  function handlePageChange(index) {
    navigate(`/admin/journal/${index}`);
  }

  return (
    <>
      <Search addOnClick={onAddHandle} />
      <Statistic entryCount={pageData && pageData.total_items} />
      <div className="flex flex-col gap-2">
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
        </div>
        {loading ? (
          <div>Loading...</div>
        ) : (
          <table className="table bg-base-100 border-0 overflow-hidden rounded-box">
            <thead className="bg-base-300">
              <tr className="border-0">
                <th className="pl-3 pb-2 pr-0 w-0">No</th>
                <th>Siswa</th>
                <th>Pembimbing</th>
                <th>Mulai-Selesai Pukul</th>
                <th>Kegiatan</th>
                <th>terverifikasi</th>
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
                          `/admin/journal/${entryValue[currentEntry]}/${row.id}`
                        );
                      }}
                    >
                      Detail
                    </button>
                    {row.verified && (
                      <button
                        className="btn btn-warning btn-xs"
                        onClick={() => onExport(row.id)}
                      >
                        Export
                      </button>
                    )}
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
            {/* <div className="join flex">
              {[...Array(pageData.num_pages)].map((_, index) => (
                <button
                  key={index}
                  className={`join-item btn ${
                    pageData.page === index ? "bg-primary text-primary-content" : "bg-base-100"
                  }`}
                  onClick={() => handlePageChange(index)}
                >
                  {index + 1}
                </button>
              ))}
            </div> */}
            {/* <button
              className="flex-none btn bg-base-100"
              onClick={() => handlePageChange(pageData.page + 1)}
              disabled={pageData.page === pageData.num_pages - 1}
            >
              <span className="material-symbols-rounded icon-size-20">
                arrow_forward
              </span>
            </button> */}
          </div>
        )}
      </div>
    </>
  );
}

export default Journal;
