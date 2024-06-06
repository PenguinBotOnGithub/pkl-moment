import React, { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import StudentEntryAddTable from "../../components/tables/StudentEntryAddTable";
import CompanyDropdown from "../../components/dropdowns/CompanyDropdown";
import AdviserDropdown from "../../components/dropdowns/AdviserDropdown";
import host from "../../assets/strings/host";
import Cookies from "universal-cookie";

function EntryAdd() {
  const cookies = new Cookies(null, { path: "/" });
  const token = cookies.get("access-token");
  const { id, entry } = useParams();
  const labelStyle = "max-w-36 min-w-36 overflow-hidden";
  const [rows, setRows] = useState([]);
  const [rowId, setRowId] = useState([]);
  const [company, setCompany] = useState([]);
  const [students, setStudents] = useState([]);
  const [loading, setLoading] = useState([true, true, true]);
  const navigate = useNavigate();

  //example dummies. please fetch it for me

  const advisers = [
    { id: 1, username: "Ahmed" },
    { id: 2, username: "Akber" },
  ];

  // const company = [
  //   { id: 1, company: "Google .Inc" },
  //   { id: 2, company: "Microsoft" },
  // ];
  const fetchDataForCompanies = async () => {
    try {
      const response = await fetch(`${host}/api/company?page=0&size=1000`, {
        headers: {
          Authorization: token,
        },
      });
      if (!response.ok) {
        throw new Error(`HTTP error: Status ${response.status}`);
      }
      let companiesData = await response.json();
      setCompany(companiesData.data.items);
    } catch (err) {
      alert("something went wrong");
      setCompany([]);
    } finally {
    }
  };

  // const students = [
  //   { id: 1, name: "Student 1" },
  //   { id: 2, name: "Student 2" },
  // ];
  const fetchDataForStudents = async () => {
    try {
      const response = await fetch(`${host}/api/student?page=0&size=1000`, {
        headers: {
          Authorization: token,
        },
      });
      if (!response.ok) {
        throw new Error(`HTTP error: Status ${response.status}`);
      }
      let studentsData = await response.json();
      setStudents(studentsData.data.items);
    } catch (err) {
      alert("something went wrong");
      setStudents([]);
    } finally {
    }
  };

  const addRow = (id, name) => {
    setRows([...rows, { id, name }]);
    setRowId([...rowId, id]);
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

  function handleOnSubmit() {
    //Logic to get form data and post it is here

    navigate("/admin/entries");
  }

  useEffect(() => {
    fetchDataForCompanies();
    fetchDataForStudents();
  }, []);

  return (
    <div className="flex-col flex gap-2 items-center">
      <div className="w-full max-w-screen-sm flex-row flex gap-2 items-center">
        <label className={labelStyle}>Pembimbing</label>
        <AdviserDropdown value={advisers} />
      </div>
      <div className="w-full max-w-screen-sm relative flex-row flex gap-2 items-center">
        <label className={labelStyle}>Perusahaan</label>
        <CompanyDropdown value={company} />
      </div>
      <div className="w-full max-w-screen-sm flex-row flex gap-2 items-center">
        <label className={labelStyle}>Tanggal Berangkat</label>
        <input type="date" className="input w-full" />
      </div>
      <div className="w-full max-w-screen-sm flex-row flex gap-2 items-center">
        <label className={labelStyle}>Tanggal Kembali</label>
        <input type="date" className="input w-full" />
      </div>
      <StudentEntryAddTable
        rows={rows}
        onAddRow={addRow}
        onDeleteRow={deleteRow}
        onSearchStudent={searchStudent}
        isMaxWidth={true}
      />
      <button className="btn btn-primary max-w-screen-sm w-full" onClick={handleOnSubmit}>Send</button>
      {/* <button className="btn btn-neutral" onClick={()=>{console.log(company);console.log(students)}}>Send</button> */}
    </div>
  );
}

export default EntryAdd;
