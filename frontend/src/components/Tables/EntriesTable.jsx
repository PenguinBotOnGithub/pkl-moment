import React, { useState } from "react";

function EntriesTable() {
  const [selectedRows, setSelectedRows] = useState([]);

  const handleSelectRow = (rowIndex) => {
    if (selectedRows.includes(rowIndex)) {
      setSelectedRows(selectedRows.filter((index) => index !== rowIndex));
    } else {
      setSelectedRows([...selectedRows, rowIndex]);
    }
  };

  const data = [
    {
      id: 1,
      pembimbing: "Cy Ganderton",
      jenisEntri: "Quality Control Specialist",
      tanggalPermintaan: "Blue",
      verifikasi: "Terverifikasi",
    },
    {
      id: 2,
      pembimbing: "Cy Ganderton",
      jenisEntri: "Quality Control Specialist",
      tanggalPermintaan: "Blue",
      verifikasi: "Terverifikasi",
    },
    {
      id: 3,
      pembimbing: "Cy Ganderton",
      jenisEntri: "Quality Control Specialist",
      tanggalPermintaan: "Blue",
      verifikasi: "Terverifikasi",
    },
  ];

  return (
    <div className="overflow-x-auto">
      <div className="flex justify-between items-center mb-2">
        <span className="text-lg">Permintaan Verifikasi</span>
        <div className="flex gap-2">
          <button
            className={`btn btn-success btn-sm text-black ${
              selectedRows.length === 0 ? "opacity-50 cursor-not-allowed" : ""
            }`}
            disabled={selectedRows.length === 0}
          >
            <span>Verifikasi yang terpilih</span>
          </button>
          <button
            className={`btn btn-warning btn-sm text-black ${
              selectedRows.length === 0 ? "opacity-50 cursor-not-allowed" : ""
            }`}
            disabled={selectedRows.length === 0}
          >
            <span>Export yang terpilih</span>
          </button>
          <button
            className={`btn btn-error btn-sm text-black ${
              selectedRows.length === 0 ? "opacity-50 cursor-not-allowed" : ""
            }`}
            disabled={selectedRows.length === 0}
          >
            <span>Delete yang terpilih</span>
          </button>
        </div>
      </div>
      <table className="table bg-base-100 border-0 overflow-hidden rounded-lg ">
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
            <th>No</th>
            <th>Pembimbing</th>
            <th>Jenis Entri</th>
            <th>Tanggal Permintaan</th>
            <th>Data Input</th>
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
              <th>{row.id}</th>
              <td>{row.pembimbing}</td>
              <td>{row.jenisEntri}</td>
              <td>{row.tanggalPermintaan}</td>
              <td>
                <button className="btn btn-info btn-xs rounded-lg mr-2">
                  Detail
                </button>
              </td>
              <td className="">{row.verifikasi}</td>
              <td>
                <button className="btn btn-error btn-xs rounded-lg mr-2">
                  Delete
                </button>
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
