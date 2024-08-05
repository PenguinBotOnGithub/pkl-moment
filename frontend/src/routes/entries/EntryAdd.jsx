import React, { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import StudentEntryAddTable from "../../components/tables/entries/StudentEntryAddTable";
import host from "../../assets/strings/host";
import Cookies from "universal-cookie";
import Dropdown from "../../components/Dropdown";
import { fetchData } from "../../services";
import { assignStudentToLetter } from "../../services/functions/students";

function EntryAdd({ role }) {
  const cookies = new Cookies(null, { path: "/" });
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
  const endDateValue = ["6 Months", "1 Year"];

  useEffect(() => {
    const fetchDataWrapper = async (
      url,
      setter,
      transform = (data) => data
    ) => {
      try {
        const data = await fetchData(url);
        setter(transform(data.data.items));
      } catch (err) {
        alert(err);
        setter([]);
      }
    };

    const flattenStudentData = (students) => {
      return students.map((student) => ({
        id: student.id,
        name: student.name,
        grade: `${student.class.grade} ${student.class.department} ${student.class.number}`,
      }));
    };

    fetchDataWrapper(`/api/company?page=0&size=1000`, setCompany);
    fetchDataWrapper(`/api/student?page=0&size=1000`, setStudents, flattenStudentData);
    if (role !== "advisor") {
      fetchDataWrapper(`/api/user`, setAdvisers, (items) =>
        items.filter((user) => user.role === "coordinator")
      );
    }
  }, []);

  const addRow = (id, name, grade) => {
    setRows((prevRows) => [...prevRows, { id, name, grade }]);
  };

  const deleteRow = (index) => {
    setRows((prevRows) => prevRows.filter((_, i) => i !== index));
  };

  const searchStudent = (value, setVisibleStudents, items) => {
    const searchTerm = value.toLowerCase();
    const filteredItems = items.filter((item) =>
      item.name.toLowerCase().startsWith(searchTerm)
    );
    setVisibleStudents(filteredItems);
  };

  const addStudentsToEntry = async (entryId) => {
    for (const row of rows) {
      const body = {
        student_id: row.id,
      };

      const result = await assignStudentToLetter(entryId, body);

      if (!result || result.status !== "success") {
        console.error(`Failed to add student with ID ${row.id}`);
      }
    }
  };

  const handleOnSubmit = async () => {
    const selectedWave = cookies.get("selected-wave");

    if (role === "coordinator") {
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
      const response = await fetchData(`/api/letters/create`, {
        method: "POST",
        body: JSON.stringify(body),
      });

      if (response.status === "success") {
        await addStudentsToEntry(response.data.id);
        navigate("/admin/entries/0");
      } else {
        alert("Submission failed");
      }
    } catch (error) {
      alert("Something went wrong: " + error.message);
    }
  };

  const handleDateClick = (index) => {
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
          <Dropdown
            items={advisers}
            displayFields={["username"]}
            searchField={"username"}
            setSelectedValue={setSelectedAdvisers}
          />
        </div>
      )}
      <div className="w-full max-w-screen-sm relative flex-row flex gap-2 items-center">
        <label className={labelStyle}>Perusahaan</label>
        <Dropdown
          items={company}
          displayFields={["name", "address"]}
          searchField={"name"}
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
          {endDateValue.map((entry, index) => (
            <button
              key={index}
              role="tab"
              onClick={() => handleDateClick(index)}
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
        items={students}
      />
      <button
        className="btn btn-primary max-w-screen-sm w-full"
        onClick={handleOnSubmit}
      >
        Send
      </button>
      <button
        onClick={() => {
          console.log(students);
        }}
      >
        Debug
      </button>
    </div>
  );
}

export default EntryAdd;
