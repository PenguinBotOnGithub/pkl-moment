import React, { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import StudentEntryAddTable from "../../components/tables/entries/StudentEntryAddTable";
import Cookies from "universal-cookie";
import Dropdown from "../../components/Dropdown";
import { fetchData, fetchDataWrapper } from "../../services";
import { assignStudentToLetter } from "../../services/functions/students";
import host from "../../assets/strings/host";

function JournalAdd({ role }) {
  const cookies = new Cookies(null, { path: "/" });
  const userId = cookies.get("user-id");
  const labelStyle = "w-28 flex-none overflow-hidden";
  const [fileUrl, setFileUrl] = useState();
  const [file, setFile] = useState();

  const navigate = useNavigate();

  const [student, setStudent] = useState([]);
  const [selectedStudent, setSelectedStudent] = useState();
  const [date, setDate] = useState("");
  const [startTime, setStartTime] = useState("");
  const [endTime, setEndTime] = useState("");
  const [activity, setActivity] = useState("");
  const [division, setDivision] = useState("");

  useEffect(() => {
    if (role == "secretary") {
      fetchDataWrapper(`/api/tenure`, setStudent);
    }
  }, []);

  const handleOnSubmit = async () => {
    try {
      let tenureId;

      // Step 1: Determine the tenure ID
      if (role == "secretary") {
        // Use selectedStudent as tenure ID
        if (!selectedStudent) {
          alert("Please select a student.");
          return;
        }
        tenureId = selectedStudent; // Adjust based on actual structure of selectedStudent
        console.log(tenureId);
      } else {
        // Fetch tenure ID from API
        const tenureResponse = await fetchData(`/api/tenure/my`);
        if (!tenureResponse || !tenureResponse.data) {
          alert("Failed to fetch tenure ID");
          return;
        }
        tenureId = tenureResponse.data[0].id;
        console.log(tenureId);
      }

      // Step 2: Upload the photo if a file is selected
      let img_url = "";
      console.log(`woyyy ${file}`);
      if (file) {
        let bin = await file.arrayBuffer();
        console.log(bin);

        const photoResponse = await fetchData(`/api/journal/photo`, {
          headers: {
            "Content-Type": "image/png",
          },
          method: "POST",
          body: file,
        });
        console.log(photoResponse);

        if (photoResponse && photoResponse.status === "success") {
          img_url = host + photoResponse.data; // Adjust based on actual response structure
        } else {
          alert("Failed to upload photo");
          return;
        }
      }

      // Step 3: Prepare the journal entry data
      const body = {
        tenure_id: tenureId,
        division: division,
        entry_date: date,
        start_time: startTime,
        end_time: endTime,
        activity: activity,
        img_url: img_url || "", // Use the uploaded image URL or fallback to empty string
      };

      // Step 4: Post the journal entry
      const journalResponse = await fetchData(`/api/journal/create`, {
        method: "POST",
        body: JSON.stringify(body),
      });

      if (journalResponse.status === "success") {
        navigate("/admin/journal/0");
      } else {
        alert("Submission failed");
      }
    } catch (error) {
      alert("Something went wrong: " + error.message);
    }
  };

  return (
    <div className="flex-col flex gap-2 items-center">
      {role == "secretary" && (
        <div className="w-full max-w-screen-sm flex-row flex gap-2 items-center">
          <label className={labelStyle}>Siswa</label>
          <Dropdown
            items={student}
            displayFields={["student"]}
            searchField={"student"}
            setSelectedValue={setSelectedStudent}
          />
        </div>
      )}
      <div className="w-full max-w-screen-sm flex-row flex gap-2 items-center">
        <label className={labelStyle}>Foto Kegiatan</label>
        <input
          type="file"
          className="file-input w-full"
          onChange={(e) => {
            setFileUrl(URL.createObjectURL(e.target.files[0]));
            setFile(e.target.files[0]);
          }}
        />
      </div>
      {fileUrl && (
        <div className="w-full max-w-screen-sm flex-row flex gap-2 items-center">
          <img
            src={fileUrl}
            alt=""
            className="object-cover flex-1 rounded-btn"
          />
        </div>
      )}
      <div className="w-full max-w-screen-sm flex-row flex gap-2 items-center">
        <label className={labelStyle}>Tanggal Kegiatan</label>
        <button
          className="btn btn-neutral grow-[0.2] text-lg"
          onClick={() => {
            let currentDate = new Date().toJSON().slice(0, 10);
            setDate(`${currentDate}`);
          }}
        >
          Hari Ini
        </button>
        <input
          type="date"
          className="input text-center grow-[2]"
          value={date}
          onChange={(e) => setDate(e.target.value)}
        />
      </div>
      <div className="w-full max-w-screen-sm flex-row flex gap-2 items-center">
        <label className={labelStyle}>Waktu Kegiatan</label>
        <input
          type="time"
          className="input w-full text-center"
          value={startTime}
          onChange={(e) => setStartTime(e.target.value)}
        />
        <span>-</span>
        <input
          type="time"
          className="input w-full text-center"
          value={endTime}
          onChange={(e) => setEndTime(e.target.value)}
        />
      </div>
      <div className="w-full max-w-screen-sm flex-row flex gap-2 items-center">
        <label className={labelStyle}>Kegiatan</label>
        <input
          type="text"
          className="input w-full"
          value={activity}
          onChange={(e) => setActivity(e.target.value)}
        />
      </div>
      <div className="w-full max-w-screen-sm relative flex-row flex gap-2 items-center">
        <label className={labelStyle}>Divisi</label>
        <input
          type="text"
          className="input w-full"
          value={division}
          onChange={(e) => setDivision(e.target.value)}
        />
      </div>
      <button
        className="btn btn-primary max-w-screen-sm w-full"
        onClick={handleOnSubmit}
      >
        Send
      </button>
      <button
        onClick={() => {
          console.log(date);
        }}
      >
        Debug
      </button>
    </div>
  );
}

export default JournalAdd;
