import React, { useState } from "react";
import { useNavigate } from "react-router-dom";
import Cookies from "universal-cookie";
import host from "../../assets/strings/host";

function StudentAddTable() {
  const [rows, setRows] = useState([{ name: "", class: "", nis:"" }]);
  const cookies = new Cookies(null, { path: "/" });
  const token = cookies.get("access-token");
  const navigate = useNavigate();

  const handleSubmit = async (formData) => {
    console.log("Form data submitted:", formData);
    await fetch(`${host}/api/student/create`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: token,
      },
      body: JSON.stringify({
        name: formData.name,
        class: formData.class,
        nis: formData.nis,
      }),
    })
      .then((response) => response.json())
      .then((result) => {
        if (result.status === "success") {
          navigate("/admin/entries/student");
        }
      })
      .catch(() => {
        alert("Something went wrong");
      });
  };

  function execBulkPost() {
    rows.forEach((row) => {
      if (row.name.trim() == "" || row.class.trim() == "" || row.nis.trim() == "") {
        alert("Please fill every data before submitting");
      } else {
        handleSubmit(row);
      }
    });
  }

  const handleInputChange = (index, event) => {
    const { name, value } = event.target;
    const newRows = [...rows];
    newRows[index][name] = value;
    setRows(newRows);
  };

  const addRow = () => {
    setRows([...rows, { name: "", class: "", nis:"" }]);
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
                  name="class"
                  value={row.class}
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
        <div className="btn btn-success btn-sm" onClick={execBulkPost}>Kirim</div>
      </div>
    </div>
  );
}

export default StudentAddTable;
