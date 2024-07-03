import React, { useState } from "react";
import { useNavigate } from "react-router-dom";
import Cookies from "universal-cookie";
import host from "../../../assets/strings/host";

function CompanyAddTable() {
  const [rows, setRows] = useState([{ name: "", address: "" }]);
  const cookies = new Cookies(null, { path: "/" });
  const token = cookies.get("access-token");
  const navigate = useNavigate();

  const handleSubmit = async (formData) => {
    console.log("Form data submitted:", formData);
    await fetch(`${host}/api/company/create`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: token,
      },
      body: JSON.stringify({
        name: formData.name,
        address: formData.address,
      }),
    })
      .then((response) => response.json())
      .then((result) => {
        if (result.status === "success") {
          navigate("/admin/entries/company");
        }
      })
      .catch(() => {
        alert("Please check your companies information.");
      });
  };

  function execBulkPost() {
    rows.forEach((row) => {
      if (row.name.trim() == "" || row.address.trim() == "") {
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
    setRows([...rows, { name: "", address: "" }]);
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
            <th>Nama Perusahaan</th>
            <th>Alamat</th>
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
                  style={{
                    backgroundColor: "transparent",
                    border: "none",
                    outline: "none",
                  }}
                  required
                />
              </td>
              <td>
                <input
                  type="text"
                  name="address"
                  value={row.address}
                  onChange={(event) => handleInputChange(index, event)}
                  style={{
                    backgroundColor: "transparent",
                    border: "none",
                    outline: "none",
                  }}
                  required
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

export default CompanyAddTable;
