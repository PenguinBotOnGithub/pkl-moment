import React, { useState } from "react";

function DashboardTable({ data }) {
  const [selectedRows, setSelectedRows] = useState([]);

  const handleSelectRow = (rowIndex) => {
    if (selectedRows.includes(rowIndex)) {
      setSelectedRows(selectedRows.filter((index) => index !== rowIndex));
    } else {
      setSelectedRows([...selectedRows, rowIndex]);
    }
  };

  return (
    <div className="overflow-x-auto">
      <div className="flex justify-between items-center mb-2">
        <span className="text-lg">Permintaan Verifikasi</span>
        <div className="flex gap-2 ">
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
          <button className="btn btn-info btn-sm text-black">
            <span>Lihat lebih</span>
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
            <th>Pembimbing</th>
            <th>Jenis Entri</th>
            <th>Tanggal Permintaan</th>
            <th>Data Input</th>
            <th>Verifikasi</th>
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
              <td>{row.pembimbing}</td>
              <td>{row.jenisEntri}</td>
              <td>{row.tanggalPermintaan}</td>
              <td>
                <button className="btn btn-info btn-xs rounded-lg mr-2">
                  Detail
                </button>
              </td>
              <td>{row.verifikasi ? <p className="opacity-60">Terverifikasi</p> : <button className="btn btn-success btn-xs">Verifikasi</button>}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export default DashboardTable;
