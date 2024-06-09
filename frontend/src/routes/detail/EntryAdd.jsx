import React, { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import StudentEntryAddTable from "../../components/tables/StudentEntryAddTable";
import CompanyDropdown from "../../components/dropdowns/CompanyDropdown";
import AdviserDropdown from "../../components/dropdowns/AdviserDropdown";
import host from "../../assets/strings/host";
import Cookies from "universal-cookie";

function EntryAdd() {
  const cookies = new Cookies(null, { path: "/" });
  const role = cookies.get("role");
  const token = cookies.get("access-token");
  const userId = cookies.get("user-id");
  const { entry } = useParams();
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
  const [currentEndDate, setCurrentEndDate] = useState(0);
  const entryValue = ["6 Bulan", "1 Tahun"];

  const fetchData = async (url, setter, transform = (data) => data) => {
    try {
      const response = await fetch(url, {
        headers: {
          Authorization: token,
        },
      });
      if (!response.ok) {
        throw new Error(`HTTP error: Status ${response.status}`);
      }
      const data = await response.json();
      setter(transform(data.data.items));
    } catch (err) {
      alert("Something went wrong");
      setter([]);
    }
  };

  useEffect(() => {
    fetchData(`${host}/api/company?page=0&size=1000`, setCompany);
    fetchData(`${host}/api/student?page=0&size=1000`, setStudents);
    if (role !== "advisor") {
      fetchData(`${host}/api/user`, setAdvisers, (items) =>
        items.filter((user) => user.role === "advisor")
      );
    }
  }, []);

  const addRow = (id, name, grade) => {
    setRows((prevRows) => [...prevRows, { id, name, grade }]);
  };

  const deleteRow = (index) => {
    setRows((prevRows) => prevRows.filter((_, i) => i !== index));
  };

  const searchStudent = (value, setVisibleStudents) => {
    const searchTerm = value.toLowerCase();
    const filteredStudents = matchSorter(value, searchTerm, { threshold: matchSorter.rankings.STARTS_WITH, keys: ['name'] });
    setVisibleStudents(filteredStudents);
  };

  const addStudentsToEntry = async (entryId) => {
    for (const row of rows) {
      const body = {
        student_id: row.id,
      };

      try {
        const response = await fetch(`${host}/api/${entry}/${entryId}/student/add`, {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
            Authorization: token,
          },
          body: JSON.stringify(body),
        });

        const result = await response.json();
        if (result.status !== "success") {
          alert(`Failed to add student ${row.name}`);
        }
      } catch (error) {
        alert(`Something went wrong: ${error.message}`);
      }
    }
  };

  const handleOnSubmit = async () => {
    const selectedWave = cookies.get("selected-wave");

    if (role === "advisor") {
      setSelectedAdvisers(userId);
    }

    const body = {
      user_id: selectedAdvisers,
      company_id: selectedCompany,
      wave_id: selectedWave,
      end_date: endDate,
      ...(entry !== "penarikan" && { start_date: startDate }),
    };

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
        const entryId = result.data.id;
        await addStudentsToEntry(entryId);
        navigate("/admin/entries/0");
      } else {
        alert("Submission failed");
      }
    } catch (error) {
      alert("Something went wrong: " + error.message);
    }
  };

  const handleEntryClick = (index) => {
    if (startDate) {
      const newEndDate = new Date(startDate);
      newEndDate.setMonth(newEndDate.getMonth() + (index === 0 ? 6 : 12));
      setEndDate(newEndDate.toISOString().split("T")[0]);
      setCurrentEndDate(index);
    } else {
      setCurrentEndDate(index);
    }
  };

  return (
    <div className="flex-col flex gap-2 items-center">
      {role !== "advisor" && (
        <div className="w-full max-w-screen-sm flex-row flex gap-2 items-center">
          <label className={labelStyle}>Pembimbing</label>
          <AdviserDropdown
            value={advisers}
            setSelectedValue={setSelectedAdvisers}
          />
        </div>
      )}
      <div className="w-full max-w-screen-sm relative flex-row flex gap-2 items-center">
        <label className={labelStyle}>Perusahaan</label>
        <CompanyDropdown
          value={company}
          setSelectedValue={setSelectedCompany}
        />
      </div>
      {entry !== "penarikan" && (
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
        <div
          role="tablist"
          className="tabs-boxed p-0 bg-base-100 gap-2 flex flex-row flex-nowrap"
        >
          {entryValue.map((entry, index) => (
            <button
              key={index}
              role="tab"
              onClick={() => handleEntryClick(index)}
              className={`tab hover:bg-base-300 ease-in-out duration-150 ${
                currentEndDate === index ? "tab-active" : ""
              }`}
            >
              {entry}
            </button>
          ))}
        </div>
        <span>{endDate}</span>
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
