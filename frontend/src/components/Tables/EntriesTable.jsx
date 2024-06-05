import React, { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";

function EntriesTable() {
  const navigate = useNavigate();

  const newData1 = [
    { id:0, adviser: 'Cy Ganderton', company: 'Google .inc', created_at: '23/05/2024', verified: true },
    { id:1, adviser: 'Cy Ganderton', company: 'Google .inc', created_at: '23/05/2024', verified: false },
    { id:2, adviser: 'Cy Ganderton', company: 'Google .inc', created_at: '23/05/2024', verified: true }
  ];
  const newData2 = [
    { id:0, adviser: 'Ridho Jago', company: 'Google .inc', created_at: '23/05/2024', verified: false },
    { id:1, adviser: 'Ridho Jago', company: 'Google .inc', created_at: '23/05/2024', verified: true },
    { id:2, adviser: 'Ridho Jago', company: 'Google .inc', created_at: '23/05/2024', verified: true }
  ];
  const newData3 = [
    { id:0, adviser: 'Rawrr', company: 'Google .inc', created_at: '23/05/2024', verified: true },
    { id:1, adviser: 'Rawrr', company: 'Google .inc', created_at: '23/05/2024', verified: true },
    { id:2, adviser: 'Rawrr', company: 'Google .inc', created_at: '23/05/2024', verified: false }
  ];

  const [selectedRows, setSelectedRows] = useState([]);
  const [currentEntry, setCurrentEntry] = useState(0);
  const entryValue = ["permohonan", "pengantaran", "penjemputan"];
  //? 0 = "permohonan" | 1 = "pengantaran" | 2 = "penjemputan"

  const [data, setData] = useState(newData1);

  function handleSelectRow (rowIndex) {
    if (selectedRows.includes(rowIndex)) {
      setSelectedRows(selectedRows.filter((index) => index !== rowIndex));
    } else {
      setSelectedRows([...selectedRows, rowIndex]);
    }
  };

  function handleSelectTab(index){
    setCurrentEntry(index);
    const newData = [newData1, newData2, newData3];
    setData(newData[index]);
  }

  return (
    <div className="flex flex-col gap-2">
      <div className="flex justify-between items-center gap-2">
        <div role="tablist" className= "tabs-boxed p-0 bg-base-100 gap-2 flex flex-row flex-nowrap">
          <button role="tab" onClick={() => handleSelectTab(0)} className={`tab hover:bg-base-300 ease-in-out duration-150 ${currentEntry === 0 && `tab-active`}`}>Permohonan</button>
          <button role="tab" onClick={() => handleSelectTab(1)} className={`tab hover:bg-base-300 ease-in-out duration-150 ${currentEntry === 1 && `tab-active`}`}>Pengantaran</button>
          <button role="tab" onClick={() => handleSelectTab(2)} className={`tab hover:bg-base-300 ease-in-out duration-150 ${currentEntry === 2 && `tab-active`}`}>Penjemputan</button>
        </div>
        <div className="flex gap-2">
          <button
            className={`btn btn-success btn-sm text-black ${
              selectedRows.length === 0 ? "opacity-50 cursor-not-allowed" : ""
            }`}
            disabled={selectedRows.length === 0}
          >
            Verifikasi {<span className="hidden lg:block">yang terpilih</span>}
          </button>
          <button
            className={`btn btn-warning btn-sm text-black ${
              selectedRows.length === 0 ? "opacity-50 cursor-not-allowed" : ""
            }`}
            disabled={selectedRows.length === 0}
          >
            Export {<span className="hidden lg:block">yang terpilih</span>}
          </button>
          <button
            className={`btn btn-error btn-sm text-black ${
              selectedRows.length === 0 ? "opacity-50 cursor-not-allowed" : ""
            }`}
            disabled={selectedRows.length === 0}
          >
            Delete {<span className="hidden lg:block">yang terpilih</span>}
          </button>
        </div>
      </div>
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
              <td>{row.adviser}</td>
              <td>{row.company}</td>
              <td>{row.created_at}</td>
              <td>{row.verified ? <p className="opacity-60">Terverifikasi</p> : <button className="btn btn-success btn-xs">Verifikasi</button>}</td>
              <td className="flex flex-row flex-nowrap gap-2">
                <button className="btn btn-info btn-xs rounded-lg" onClick={()=>{navigate(`/admin/entries/${entryValue[currentEntry]}/${row.id}`)}}>
                  Detail
                </button>
                {row.verified && <button className="btn btn-warning btn-xs">Export</button>}
              </td>
            </tr>
            
          ))}
        </tbody>
      </table>
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
    </div>
  );
}

export default EntriesTable;
