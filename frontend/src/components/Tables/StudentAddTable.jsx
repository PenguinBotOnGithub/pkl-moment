import React, { useState } from "react";

function StudentAddTable() {
  const [rows, setRows] = useState([{ name: "", grade: "", nis:"" }]);

  const handleInputChange = (index, event) => {
    const { name, value } = event.target;
    const newRows = [...rows];
    newRows[index][name] = value;
    setRows(newRows);
  };

  const addRow = () => {
    setRows([...rows, { name: "", grade: "", nis:"" }]);
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
            <th>Nama Siswa</th>
            <th>Kelas</th>
            <th>NIS</th>
            <th>Aksi</th>
          </tr>
        </thead>
        <tbody className="box-content">
          {rows.map((row, index) => (
            <tr key={index} className="border-t-2 border-neutral ">
              <td>{index + 1}</td>
              <td>
                <input
                  type="text"
                  name="name"
                  value={row.name}
                  onChange={(event) => handleInputChange(index, event)}
                  style={{ backgroundColor: 'transparent', border: 'none', outline: 'none' }}
                />
              </td>
              <td>
                <input
                  type="text"
                  name="grade"
                  value={row.grade}
                  onChange={(event) => handleInputChange(index, event)}
                  style={{ backgroundColor: 'transparent', border: 'none', outline: 'none' }}
                />
              </td>
              <td>
                <input
                  type="text"
                  name="nis"
                  value={row.nis}
                  onChange={(event) => handleInputChange(index, event)}
                  style={{ backgroundColor: 'transparent', border: 'none', outline: 'none' }}
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
        <div className="btn btn-neutral btn-sm" onClick={addRow}>Tambah Baris</div>
        <div className="btn btn-success btn-sm">Kirim</div>
      </div>
    </div>
  );
}

export default StudentAddTable;
