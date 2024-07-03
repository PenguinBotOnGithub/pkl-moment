import React, { useState } from "react";
import host from "../../../assets/strings/host";
import Cookies from "universal-cookie";
import { useNavigate } from "react-router-dom";

function WaveAddTable() {
  const [rows, setRows] = useState([{ start_date: "", end_date: "" }]);
  const cookies = new Cookies(null, { path: "/" });
  const token = cookies.get("access-token");
  const navigate = useNavigate();

  const handleSubmit = async (formData) => {
    console.log("Form data submitted:", formData);
    let res = await fetch(`${host}/api/wave/create`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: token,
      },
      body: JSON.stringify({
        start_date: formData.start_date,
        end_date: formData.end_date,
      }),
    });

    let json = await res.json();

    if (json.status === "success") {
      navigate("/admin/entries/wave/0");
    }
  };

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

  function execBulkPost() {
    rows.forEach((row) => {
      if (row.start_date.trim() == "" || row.end_date.trim() == "") {
        alert("Please fill every data before submitting");
      } else {
        handleSubmit(row);
      }
    });
  }

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
                <button
                  className="btn btn-error btn-xs rounded-lg mr-2"
                  onClick={() => deleteRow(index)}
                >
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
        <button className="btn btn-success btn-sm" onClick={execBulkPost}>
          Kirim
        </button>
      </div>
    </div>
  );
}

export default WaveAddTable;
