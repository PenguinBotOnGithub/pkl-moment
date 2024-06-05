import React, { useState } from "react";
import { useParams } from "react-router-dom";
import getCurrentDate from "../../assets/strings/getCurrentDate";
import StudentEntryAddTable from "../../components/tables/StudentEntryAddTable";

function Entry() {
  let { id, entry } = useParams();
  const [rows, setRows] = useState([]);
  const verifikasi = true;

  function titleCase(str) {
    return str.toLowerCase().replace(/(^|\s)\S/g, (L) => L.toUpperCase());
  }

  const students = [
    { id: 1, name: "Student 1" },
    { id: 2, name: "Student 2" },
  ];

  const addRow = (name) => {
    setRows([...rows, { name }]);
  };

  const deleteRow = (index) => {
    const newRows = [...rows];
    newRows.splice(index, 1);
    setRows(newRows);
  };

  const searchStudent = (value, setVisibleStudents) => {
    // Dummy data for demonstration, replace with actual API call or logic
    const searchTerm = value.toLowerCase();
    const filteredStudents = students.filter((student) =>
      student.name.toLowerCase().includes(searchTerm)
    );
    setVisibleStudents(filteredStudents);
  };

  const [selectedRows, setSelectedRows] = useState([]);

  const handleSelectRow = (rowIndex) => {
    if (selectedRows.includes(rowIndex)) {
      setSelectedRows(selectedRows.filter((index) => index !== rowIndex));
    } else {
      setSelectedRows([...selectedRows, rowIndex]);
    }
  };

  return (
    <>
      <table className="table bg-base-100 border-0 overflow-hidden rounded-lg ">
        <thead className="bg-neutral">
          <tr className="border-0">
            <th>Pembimbing</th>
            <th>Jenis Entri</th>
            <th>Tanggal Permintaan</th>
            <th>Verifikasi</th>
            <th>Aksi</th>
          </tr>
        </thead>
        <tbody className="box-content">
          <tr>
            <td>Akber</td>
            <td>{titleCase(entry)}</td>
            <td>{getCurrentDate("/")}</td>
            <td>
              {verifikasi ? (
                <p className="opacity-60">Terverifikasi</p>
              ) : (
                <button className="btn btn-success btn-xs">Verifikasi</button>
              )}
            </td>
            <td className="gap-2 flex flex-row">
              {verifikasi && (
                <button className="btn btn-warning btn-xs">Export</button>
              )}
              <button className="btn btn-error btn-xs">Delete</button>
            </td>
          </tr>
        </tbody>
      </table>
      Perusahaan
      <table className="table bg-base-100 border-0 overflow-hidden rounded-lg ">
        <thead className="bg-neutral">
          <tr className="border-0">
            <th>Nama Perusahaan</th>
            <th>Alamat</th>
            <th>Aksi</th>
          </tr>
        </thead>
        <tbody className="box-content">
          <tr>
            <td>Google .Inc</td>
            <td>kudus</td>
            <td className="gap-2 flex flex-row">
              <button className="btn btn-warning btn-xs">Ganti</button>
            </td>
          </tr>
        </tbody>
      </table>
      Siswa
      <StudentEntryAddTable
        rows={rows}
        onAddRow={addRow}
        onDeleteRow={deleteRow}
        onSearchStudent={searchStudent}
      />
      Tanda Tangan
      <table className="table bg-base-100 border-0 overflow-hidden rounded-lg ">
        <thead className="bg-neutral">
          <tr className="border-0">
            <th>Nama</th>
            <th>Jabatan</th>
            <th>Di Dokumen</th>
          </tr>
        </thead>
        <tbody className="box-content">
          <tr className="border-t-2 border-neutral">
            <td>Pak Agus</td>
            <td>Jabatan</td>
            <td className="p-3 pb-2">
              <label className="swap opacity-60">
                <input
                  type="checkbox"
                  onChange={() => handleSelectRow(0)}
                  checked={selectedRows.includes(0)}
                />
                <span className="swap-off material-symbols-rounded">
                  check_box_outline_blank
                </span>
                <span className="swap-on material-symbols-rounded">
                  check_box
                </span>
              </label>
            </td>
          </tr>
          <tr className="border-t-2 border-neutral">
            <td>Pak Agus</td>
            <td>Jabatan</td>
            <td className="p-3 pb-2">
              <label className="swap opacity-60">
                <input
                  type="checkbox"
                  onChange={() => handleSelectRow(1)}
                  checked={selectedRows.includes(1)}
                />
                <span className="swap-off material-symbols-rounded">
                  check_box_outline_blank
                </span>
                <span className="swap-on material-symbols-rounded">
                  check_box
                </span>
              </label>
            </td>
          </tr>
        </tbody>
      </table>
    </>
  );
}

export default Entry;
