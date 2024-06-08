import React, { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import getCurrentDate from "../../assets/strings/getCurrentDate";
import StudentEntryAddTable from "../../components/tables/StudentEntryAddTable";
import Cookies from "universal-cookie";

function Entry() {
  let { id, entry } = useParams();
  const [rows, setRows] = useState([]);
  const [data, setData] = useState();
  const [verifikasi, setVerifikasi] = useState(false);
  const cookies = new Cookies();
  const role = cookies.get("role");
  const token = cookies.get("access-token");

  const fetchDataForEntry = async () => {
    try {
      const response = await fetch(`${host}/api/${entry}/${id}`, {
        headers: {
          Authorization: token,
        },
      });
      if (!response.ok) {
        throw new Error(`HTTP error: Status ${response.status}`);
      }
      let entryData = await response.json();
      setData(entryData.data);
      setIsDataEdited(entryData.data.map(() => false));
      console.log(data);
    } catch (err) {
      console.log("Error fetching data: " + err);
      setData([]);
    } finally {
    }
  };

  const onVerify = async () => {
    try {
      const response = await fetch(
        `${host}/api/${entryValue[currentEntry]}/${id}/verify`,
        {
          headers: {
            Authorization: token,
          },
          method: "PATCH",
        }
      );
      if (!response.ok) {
        throw new Error(`HTTP error: Status ${response.status}`);
      }
      await response.json();
      setError(null);
      fetchDataForEntry(entryValue[currentEntry]);
      setVerifikasi(true);
    } catch (err) {
      setError(err.message);
    }
  };

  function titleCase(str) {
    return str.toLowerCase().replace(/(^|\s)\S/g, (L) => L.toUpperCase());
  }

  const students = [
    { id: 1, name: "Aan Kurniawan", grade: "11 PPLG-1" },
    { id: 2, name: "Aaron Ikhwan Saputra", grade: "11 PPLG-1" },
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

  useEffect(() => {
    fetchDataForEntry();
  }, []);

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
            <td>Pak Agus</td>
            <td>{titleCase(entry)}</td>
            <td>{getCurrentDate("/")}</td>
            <td>
              {verifikasi ? (
                <p className="opacity-60">Terverifikasi</p>
              ) : (
                <p>Verifikasi</p>
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
        rows={students}
        onAddRow={addRow}
        onDeleteRow={deleteRow}
        onSearchStudent={searchStudent}
      />
      {role != "advisor" && (
        <>
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
      )}
      {role != "adviser" && !verifikasi && (
        <button
          className="btn btn-success rounded-lg btn-md"
          onClick={() =>
            document.getElementById("logout_confirmation_modal").showModal()
          }
        >
          <span>Verifikasi</span>
        </button>
      )}
      <dialog id="logout_confirmation_modal" className="modal">
        <div className="modal-box">
          <h3 className="font-bold text-lg text-error">Warning!</h3>
          <p className="pt-4">Are you sure you want to <span className="text-success">verify</span>?</p>
          <p>This action is irrevesable</p>
          <div className="modal-action">
            <form method="dialog">
              {/* if there is a button in form, it will close the modal */}
              <button
                className="btn text-error"
                onClick={() => setVerifikasi(true)}
              >
                Yes
              </button>
              <button className="btn ml-2">Cancel</button>
            </form>
          </div>
        </div>
      </dialog>
    </>
  );
}

export default Entry;
