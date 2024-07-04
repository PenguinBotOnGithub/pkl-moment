import React, { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import Cookies from "universal-cookie";
import host from "../assets/strings/host";
import Statistic from "../components/count/Statistic";

function SearchEntry() {
  const navigate = useNavigate();
  const cookies = new Cookies();
  const role = cookies.get("role");
  const token = cookies.get("access-token");

  const [selectedRows, setSelectedRows] = useState([]);
  const [currentEntry, setCurrentEntry] = useState(0);
  const entryValue = ["permohonan", "pengantaran", "penarikan"];
  const [data, setData] = useState([]);
  const [isDataEdited, setIsDataEdited] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [dataWave, setDataWave] = useState("");

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
      const response = await fetch(`${host}/api/${entryType}?page=0`, {
        headers: {
          Authorization: token,
        },
      });
      if (!response.ok) {
        throw new Error(`HTTP error: Status ${response.status}`);
      }
      let entryData = await response.json();
      setData(entryData.data.items);
      setIsDataEdited(entryData.data.items.map(() => false));
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
  }, [currentEntry]);

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
    try {
      const response = await fetch(
        `${host}/api/${entryValue[currentEntry]}/${index}/pdf`,
        {
          headers: {
            "Content-Type": "application/json",
            Authorization: token,
          },
          method: "POST",
          body: JSON.stringify({
            signature_1_id: 1,
            signature_2_id: 2,
          }),
        }
      );
      let bin = [];
      for await (const chunk of response.body) {
        console.log("dfdfd");
        console.log(chunk);
        bin.push(chunk);
      }
      let blob = new Blob(bin, { type: "application/pdf" });
      console.log(await blob.arrayBuffer());
      downloadBlob(blob, "pdfff.pdf");
      setError(null);
      fetchDataForEntry(entryValue[currentEntry]);
    } catch (err) {
      setError(err.message);
    }
  };

  return (
    <>
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
          <div className="flex gap-2 items-center">
            Cari Berdasarkan:
            <details className="dropdown dropdown-bottom dropdown-end">
              <summary className="m-1 btn btn-sm btn-neutral">Company</summary>
              <ul className="p-2 shadow menu dropdown-content z-[1] bg-base-100 rounded-box w-52">
                <li>
                  <button onClick={() => {navigate("")}} className="btn btn-sm btn-ghost">Google .Inc</button>
                  <button onClick={() => {navigate("/admin/entries/searchdummy")}} className="btn btn-sm btn-ghost">Microsoft Corporation</button>
                </li>
              </ul>
            </details>
            <button className="btn btn-neutral btn-sm" onClick={() => {navigate("/admin/entries/search/siswa")}}>Siswa</button>
          </div>
        </div>
        {loading ? (
          <div>Loading...</div>
        ) : (
          <table className="table bg-base-100 border-0 overflow-hidden rounded-lg">
            <thead className="bg-neutral">
              <tr className="border-0">
                <th className="pl-3 pb-2 pr-0 w-0">
                  <label className="swap">
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
                  </label>
                </th>
                <th>Pembimbing</th>
                <th>Perusahaan</th>
                <th>Tanggal Permintaan</th>
                <th>Verifikasi</th>
                <th>Aksi</th>
              </tr>
            </thead>
            <tbody className="box-content">
              {data.map((row, index) => (
                <tr key={row.id} className="border-t-2 border-neutral">
                  <td className="p-3 pb-2">
                    <label className="swap opacity-60">
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
                    </label>
                  </td>
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
                      className="btn btn-info btn-xs rounded-lg"
                      onClick={() => {
                        navigate(
                          `/admin/entries/${entryValue[currentEntry]}/${row.id}`
                        );
                      }}
                    >
                      Detail
                    </button>
                    <button
                      className="btn btn-error btn-xs rounded-lg"
                      onClick={() => deleteEntry(row.id)}
                    >
                      Delete
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
        <div className="flex justify-center items-center gap-2 mt-4">
          <button className="flex-none btn bg-base-100">
            <span className="material-symbols-rounded icon-size-20">
              arrow_back
            </span>
          </button>
          <div className="join flex gap-2">
            <button className="join-item btn">1</button>
            <button className="join-item btn">2</button>
            <button className="join-item btn opacity-50">...</button>
            <button className="join-item btn">99</button>
            <button className="join-item btn">100</button>
          </div>
          <button className="flex-none btn bg-base-100">
            <span className="material-symbols-rounded icon-size-20">
              arrow_forward
            </span>
          </button>
        </div>
        <button
          onClick={() => {
            console.log(data);
          }}
        >
          debug
        </button>
      </div>
    </>
  );
}

export default SearchEntry;
