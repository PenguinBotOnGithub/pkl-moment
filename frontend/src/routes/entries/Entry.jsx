import React, { useEffect, useState } from "react";
import { useParams, useNavigate } from "react-router-dom";
import getCurrentDate from "../../assets/strings/getCurrentDate";
import StudentEntryAddTable from "../../components/tables/entries/StudentEntryAddTable";
import Cookies from "universal-cookie";
import host from "../../assets/strings/host"; // Import the host URL
import { matchSorter } from "match-sorter";
import { exportLetter } from "../../services/functions/letters";

function Entry() {
  let { id, entry } = useParams();
  const [rows, setRows] = useState([]);
  const [data, setData] = useState();
  const [dataEntryStudent, setDataEntryStudent] = useState([]);
  const [dataAllStudent, setDataAllStudent] = useState([]);
  const [verifikasi, setVerifikasi] = useState(true);
  const cookies = new Cookies();
  const role = cookies.get("role");
  const token = cookies.get("access-token");
  const [isStudentListChanged, setIsStudentListChanged] = useState(false);
  const [selectedRows, setSelectedRows] = useState([]);
  const navigate = useNavigate();

  const [error, setError] = useState(null);
  const [loading, setLoading] = useState(true);

  const [isOpen, setIsOpen] = useState(false);

  const onExport = async (index) => {
    exportLetter(index);
  };

  const fetchDataForEntry = async () => {
    try {
      const response = await fetch(`${host}/api/letters/${id}`, {
        headers: {
          Authorization: token,
        },
      });
      if (!response.ok) {
        throw new Error(`HTTP error: Status ${response.status}`);
      }
      let entryData = await response.json();
      setData(entryData.data);
      setVerifikasi(entryData.data.verified);
      console.log(entryData.data);
    } catch (err) {
      console.log("Error fetching data: " + err);
      setData([]);
    } finally {
      setLoading(false);
    }
  };

  const fetchDataForEntryStudents = async () => {
    try {
      const response = await fetch(`${host}/api/letters/${id}/student`, {
        headers: {
          Authorization: token,
        },
      });
      if (!response.ok) {
        throw new Error(`HTTP error: Status ${response.status}`);
      }
      let studentsData = await response.json();
      setDataEntryStudent(studentsData.data);
      setRows(studentsData.data);
      console.log(rows);
    } catch (err) {
      console.log("Error fetching data: " + err);
      setDataEntryStudent([]);
    }
  };

  const fetchAllStudents = async () => {
    try {
      const response = await fetch(`${host}/api/student?page=0&size=1000`, {
        headers: {
          Authorization: token,
        },
      });
      if (!response.ok) {
        throw new Error(`HTTP error: Status ${response.status}`);
      }
      let allStudentsData = await response.json();
      setDataAllStudent(allStudentsData.data.items);
    } catch (err) {
      console.log("Error fetching data: " + err);
      setDataAllStudent([]);
    }
  };

  const onVerify = async () => {
    try {
      const response = await fetch(`${host}/api/letters/${id}/verify`, {
        headers: {
          Authorization: token,
        },
        method: "PATCH",
      });
      if (!response.ok) {
        throw new Error(`HTTP error: Status ${response.status}`);
      }
      await response.json();
      setVerifikasi(true);
      fetchDataForEntry();
    } catch (err) {
      console.log("Error verifying data: " + err);
    }
  };

  const onDelete = async () => {
    try {
      const response = await fetch(`${host}/api/letters/${id}/delete`, {
        headers: {
          Authorization: token,
        },
        method: "DELETE",
      });
      if (!response.ok) {
        throw new Error(`HTTP error: Status ${response.status}`);
      }
      await response.json();
      navigate("/admin/entries");
    } catch (err) {
      console.log("Error deleting data: " + err);
    }
  };

  useEffect(() => {
    fetchDataForEntry();
    fetchDataForEntryStudents();
    fetchAllStudents();
  }, []);

  const addRow = (id, name, grade) => {
    setRows((prevRows) => [...prevRows, { id, name, grade }]);
    setIsStudentListChanged(true);
  };

  const deleteRow = (index) => {
    setRows((prevRows) => prevRows.filter((_, i) => i !== index));
    setIsStudentListChanged(true);
  };

  const searchStudent = (value, setVisibleStudents) => {
    const searchTerm = value.toLowerCase();
    const filteredStudents = matchSorter(dataAllStudent, searchTerm, {
      threshold: matchSorter.rankings.STARTS_WITH,
      keys: ["name"],
    });
    setVisibleStudents(filteredStudents);
  };

  const handleConfirmEdit = async () => {
    if (isStudentListChanged) {
      const currentStudentIds = dataEntryStudent.map((student) => student.id);
      const newStudentIds = rows.map((student) => student.id);

      const studentsToAdd = rows.filter(
        (student) => !currentStudentIds.includes(student.id)
      );
      const studentsToDelete = dataEntryStudent.filter(
        (student) => !newStudentIds.includes(student.id)
      );

      try {
        // Add new students
        for (const student of studentsToAdd) {
          await fetch(`${host}/api/letters/${id}/student/add`, {
            method: "POST",
            headers: {
              "Content-Type": "application/json",
              Authorization: token,
            },
            body: JSON.stringify({ student_id: student.id }),
          });
        }

        // Delete removed students
        for (const student of studentsToDelete) {
          await fetch(
            `${host}/api/letters/${id}/student/${student.id}/remove`,
            {
              method: "DELETE",
              headers: {
                Authorization: token,
              },
            }
          );
        }

        // Fetch updated student data
        fetchDataForEntryStudents();

        // refresh entry data
        fetchDataForEntry();
        setIsStudentListChanged(false);
      } catch (error) {
        console.log("Error confirming edit: " + error.message);
      }
    }
  };

  const handleCancelEdit = async () => {
    if (isStudentListChanged) {
      fetchDataForEntryStudents();
      setIsStudentListChanged(false);
    }
  };

  const handleSelectRow = (rowIndex) => {
    if (selectedRows.includes(rowIndex)) {
      setSelectedRows(selectedRows.filter((index) => index !== rowIndex));
    } else {
      setSelectedRows([...selectedRows, rowIndex]);
    }
  };

  return (
    <>
      <table className="table bg-base-100 border-0 overflow-hidden rounded-box ">
        <thead className="bg-base-300">
          <tr className="border-0">
            <th>Pembimbing</th>
            <th>Tanggal Permintaan</th>
            <th>Verifikasi</th>
            <th>Aksi</th>
          </tr>
        </thead>
        <tbody className="box-content">
          <tr>
            <td>{data?.user?.username || "N/A"}</td>
            <td>{data?.created_at}</td>
            <td>
              {verifikasi ? (
                <p className="opacity-60">Terverifikasi</p>
              ) : (
                <p>Belum Terverifikasi</p>
              )}
            </td>
            <td className="gap-2 flex flex-row">
              <button
                className="btn btn-error btn-xs mr-10"
                onClick={() =>
                  document
                    .getElementById("delete_confirmation_modal")
                    .showModal()
                }
              >
                Delete
              </button>
              {verifikasi && (
                <div className="dropdown dropdown-end fixed ml-14">
                  <button
                    className="btn btn-warning btn-xs"
                    onClick={() => {
                      isOpen ? setIsOpen(false) : setIsOpen(true);
                    }}
                  >
                    Export
                  </button>
                  {isOpen && (
                    <div className="dropdown-content flex flex-row bg-base-300 rounded-box z-[100] p-2 shadow-2xl overflow-y-auto">
                      <div
                        className="cursor-pointer btn btn-sm btn-ghost"
                        onMouseDown={() => exportLetter(id, "permohonan")}
                      >
                        Permohonan
                      </div>
                      <div
                        className="cursor-pointer btn btn-sm btn-ghost"
                        onMouseDown={() => exportLetter(id, "pengantaran")}
                      >
                        Pengantaran
                      </div>
                      <div
                        className="cursor-pointer btn btn-sm btn-ghost"
                        onMouseDown={() => exportLetter(id, "penjemputan")}
                      >
                        Penjemputan
                      </div>
                    </div>
                  )}
                </div>
              )}
            </td>
          </tr>
        </tbody>
      </table>
      <h2>Perusahaan</h2>
      <table className="table bg-base-100 border-0 overflow-hidden rounded-box z-[1]">
        <thead className="bg-base-300">
          <tr className="border-0">
            <th>Nama Perusahaan</th>
            <th>Alamat</th>
            <th>Aksi</th>
          </tr>
        </thead>
        <tbody className="box-content">
          <tr>
            <td>{data?.company?.name || "N/A"}</td>
            <td>{data?.company?.address || "N/A"}</td>
            <td className="gap-2 flex flex-row">
              <button className="btn btn-warning btn-xs">Ganti</button>
            </td>
          </tr>
        </tbody>
      </table>
      <h2>Siswa</h2>
      <StudentEntryAddTable
        rows={rows}
        onAddRow={addRow}
        onDeleteRow={deleteRow}
        onSearchStudent={searchStudent}
        student={dataAllStudent}
      />

      <div className="flex flex-row gap-2">
        <button
          className={`btn btn-${
            isStudentListChanged ? "success" : "disabled"
          } flex-1 rounded-btn btn-sm`}
          onClick={handleConfirmEdit}
        >
          <span>Confirm Edit</span>
        </button>
        <button
          className={`btn btn-${
            isStudentListChanged ? "error" : "disabled"
          } flex-1 rounded-btn btn-sm`}
          onClick={handleCancelEdit}
        >
          <span>Cancel Edit</span>
        </button>
      </div>

      {role != "advisor" && (
        <>
          <h2>Tanda Tangan</h2>
          <table className="table bg-base-100 border-0 overflow-hidden rounded-box ">
            <thead className="bg-base-300">
              <tr className="border-0">
                <th>Nama</th>
                <th>Jabatan</th>
                <th>Di Dokumen</th>
              </tr>
            </thead>
            <tbody className="box-content">
              <tr className="border-t-2 border-base-300">
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
              <tr className="border-t-2 border-base-300">
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
      {role !== "advisor" && !verifikasi && (
        <button
          className="btn btn-success rounded-box btn-md"
          onClick={() =>
            document.getElementById("verify_confirmation_modal").showModal()
          }
        >
          <span>Verifikasi</span>
        </button>
      )}
      <dialog id="verify_confirmation_modal" className="modal">
        <div className="modal-box">
          <h3 className="font-bold text-lg text-error">Warning!</h3>
          <p className="pt-4">
            Are you sure you want to
            <span className="text-success"> verify?</span>
          </p>
          This action is <span className="text-error">irreversible</span>
          <div className="modal-action">
            <form method="dialog">
              <button className="btn btn-success" onClick={onVerify}>
                Yes
              </button>
              <button className="btn ml-2">Cancel</button>
            </form>
          </div>
        </div>
      </dialog>
      <dialog id="delete_confirmation_modal" className="modal">
        <div className="modal-box">
          <h3 className="font-bold text-lg text-error">Warning!</h3>
          <p className="pt-4">
            Are you sure you want to
            <span className="text-error"> delete?</span>
          </p>
          This action is <span className="text-error">irreversible</span>
          <div className="modal-action">
            <form method="dialog">
              <button className="btn btn-error" onClick={onDelete}>
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
