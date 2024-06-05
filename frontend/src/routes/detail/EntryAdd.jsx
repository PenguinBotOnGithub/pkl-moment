import React, { useState } from "react";
import { useParams } from "react-router-dom";
import StudentEntryAddTable from "../../components/tables/StudentEntryAddTable";
import CompanyDropdown from "../../components/dropdowns/CompanyDropdown";
import AdviserDropdown from "../../components/dropdowns/AdviserDropdown";

function EntryAdd() {
  const { id, entry } = useParams();
  const labelStyle = "max-w-36 min-w-36 overflow-hidden";
  const [rows, setRows] = useState([]);

  const advisers = [
    { id: 1, name: "Ahmed" },
    { id: 2, name: "Akber" },
  ];

  const company = [
    { id: 1, company: "Google .Inc" },
    { id: 2, company: "Microsoft" },
  ];

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

  return (
    <form className="flex-col flex gap-2 items-center">
      <div className="w-full max-w-screen-sm flex-row flex gap-2 items-center">
        <label className={labelStyle}>Pembimbing</label>
        <AdviserDropdown value={advisers}/>
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
      />
      <button className="btn btn-primary max-w-screen-sm w-full">Send</button>
    </form>
  );
}

export default EntryAdd;
