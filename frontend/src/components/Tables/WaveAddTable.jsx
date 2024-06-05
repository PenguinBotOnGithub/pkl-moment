import React, { useState } from "react";

function WaveAddTable() {
  const [rows, setRows] = useState([{ start_date: "", end_date: "" }]);

  const handleInputChange = (index, event) => {
    const { name, value } = event.target;
    const newRows = [...rows];
    newRows[index][name] = value;
    setRows(newRows);
  };

  const addRow = () => {
    setRows([...rows, { start_date: "", end_date: "" }]);
  };

  const deleteRow = (index) => {
    const newRows = [...rows];
    newRows.splice(index, 1);
    setRows(newRows);
  };

  return (
    <div className="overflow-x-auto">
      <table className="table bg-base-100 border-0 overflow-hidden rounded-lg">
        <thead className="bg-neutral">
          <tr className="border-0">
            <th className="w-0">No</th>
            <th>Tanggal Mulai</th>
            <th>Tanggal Berakhir</th>
            <th>Aksi</th>
          </tr>
        </thead>
        <tbody className="box-content">
          {rows.map((row, index) => (
            <tr key={index} className="border-t-2 border-neutral ">
              <td>{index + 1}</td>
              <td>
                <input
                  type="date"
                  name="start_date"
                  value={row.start_date}
                  onChange={(event) => handleInputChange(index, event)}
                  style={{
                    backgroundColor: "transparent",
                    border: "none",
                    outline: "none",
                  }}
                />
              </td>
              <td>
                <input
                  type="date"
                  name="end_date"
                  value={row.end_date}
                  onChange={(event) => handleInputChange(index, event)}
                  className="neutral-content"
                  style={{
                    backgroundColor: "transparent",
                    border: "none",
                    outline: "none",
                  }}
                />
              </td>
              <td>
              <button className="btn btn-error btn-xs rounded-lg mr-2" onClick={() => deleteRow(index)}>
                  Delete
                </button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>

      <div className="flex justify-end mt-2 gap-2">
        <button className="btn btn-neutral btn-sm" onClick={addRow}>
          Tambah Baris
        </button>
        <button className="btn btn-success btn-sm">Kirim</button>
      </div>
    </div>
  );
}

export default WaveAddTable;
