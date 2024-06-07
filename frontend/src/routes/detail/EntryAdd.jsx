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
  const [company, setCompany] = useState([]);
  const [advisers, setAdvisers] = useState([]);
  const [students, setStudents] = useState([]);
  const navigate = useNavigate();

  const [selectedCompany, setSelectedCompany] = useState();
  const [selectedAdvisers, setSelectedAdvisers] = useState();
  const [startDate, setStartDate] = useState("");
  const [endDate, setEndDate] = useState("");

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
    }
  };

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
    }
  };

  const fetchDataForAdvisers = async () => {
    try {
      const response = await fetch(`${host}/api/user`, {
        headers: {
          Authorization: token,
        },
      });
      if (!response.ok) {
        throw new Error(`HTTP error: Status ${response.status}`);
      }
      let usersData = await response.json();
      const advisersData = usersData.data.items.filter(
        (user) => user.role === "advisor"
      );
      setAdvisers(advisersData);
    } catch (err) {
      alert("something went wrong");
      setAdvisers([]);
    }
  };

  const addRow = (id, name) => {
    setRows([...rows, { id, name }]);
  };

  const deleteRow = (index) => {
    const newRows = [...rows];
    newRows.splice(index, 1);
    setRows(newRows);
  };

  const searchStudent = (value, setVisibleStudents) => {
    const searchTerm = value.toLowerCase();
    const filteredStudents = students.filter((student) =>
      student.name.toLowerCase().includes(searchTerm)
    );
    setVisibleStudents(filteredStudents);
  };

  const handleOnSubmit = async () => {
    console.log(selectedCompany, selectedAdvisers, startDate, endDate);
    console.log(rows);
    const selectedWave = cookies.get("selected-wave");
    
    // Create the base object for the request body
    let body = {
      user_id: selectedAdvisers,
      company_id: selectedCompany,
      wave_id: selectedWave,
      end_date: endDate,
    };
  
    // Conditionally add start_date if entry is not "penarikan"
    if (entry !== "penarikan") {
      body.start_date = startDate;
    }
  
    try {
      const response = await fetch(`${host}/api/${entry}/create`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: token,
        },
        body: JSON.stringify(body),
      });
      
      const result = await response.json();
      if (result.status === "success") {
        navigate("/admin/entries");
      } else {
        alert("Submission failed");
      }
    } catch (error) {
      alert("Something went wrong: " + error.message);
    }
  };
  

  const handleSubmit = async (formData) => {
    console.log("Form data submitted:", formData);
    await fetch(`${host}/api/auth/register`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: token,
      },
      body: JSON.stringify({
        username: formData.username,
        password: formData.password,
        role: formData.role,
      }),
    })
      .then((response) => response.json())
      .then((result) => {
        if (result.status === "success") {
          
          // navigate("/admin/users");
        }
      })
      .catch(() => {
        alert("Something went wrong");
      });
  };

  useEffect(() => {
    fetchDataForCompanies();
    fetchDataForStudents();
    fetchDataForAdvisers();
  }, []);

  return (
    <div className="flex-col flex gap-2 items-center">
      <div className="w-full max-w-screen-sm flex-row flex gap-2 items-center">
        <label className={labelStyle}>Pembimbing</label>
        <AdviserDropdown
          value={advisers}
          setSelectedValue={setSelectedAdvisers}
        />
      </div>
      <div className="w-full max-w-screen-sm relative flex-row flex gap-2 items-center">
        <label className={labelStyle}>Perusahaan</label>
        <CompanyDropdown
          value={company}
          setSelectedValue={setSelectedCompany}
        />
      </div>
      {entry != "penarikan" && (
        <div className="w-full max-w-screen-sm flex-row flex gap-2 items-center">
          <label className={labelStyle}>Tanggal Berangkat</label>
          <input
            type="date"
            className="input w-full"
            value={startDate}
            onChange={(e) => setStartDate(e.target.value)}
          />
        </div>
      )}
      <div className="w-full max-w-screen-sm flex-row flex gap-2 items-center">
        <label className={labelStyle}>Tanggal Kembali</label>
        <input
          type="date"
          className="input w-full"
          value={endDate}
          onChange={(e) => setEndDate(e.target.value)}
        />
      </div>
      <StudentEntryAddTable
        rows={rows}
        onAddRow={addRow}
        onDeleteRow={deleteRow}
        onSearchStudent={searchStudent}
        isMaxWidth={true}
      />
      <button
        className="btn btn-primary max-w-screen-sm w-full"
        onClick={handleOnSubmit}
      >
        Send
      </button>
    </div>
  );
}

export default EntryAdd;
