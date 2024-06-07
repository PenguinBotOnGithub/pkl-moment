import React, { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import host from "../../assets/strings/host";
import Cookies from "universal-cookie";

function Statistic() {
  const navigate = useNavigate();
  const [dataWave, setDataWave] = useState("");
  const cookies = new Cookies(null, { path: "/" });
  const token = cookies.get("access-token");

  const fetchWaveData = async () => {
    try {
      const response = await fetch(`${host}/api/wave?page=0&size=1000`, {
        headers: {
          Authorization: token,
        },
      });
      if (!response.ok) {
        throw new Error(`HTTP error: Status ${response.status}`);
      }
      const waveData = await response.json();
      const selectedWaveId = cookies.get("selected-wave");
      const wave = waveData.data.items.find(element => element.id === parseInt(selectedWaveId));

      if (wave) {
        setDataWave(`${new Date(wave.start_date).getFullYear()}/${new Date(wave.end_date).getFullYear()}`);
      } else {
        setDataWave("No wave selected");
      }
    } catch (err) {
      alert("Something went wrong: " + err);
      setDataWave("");
    }
  };

  useEffect(() => {
    fetchWaveData();
  }, []);

  return (
    <div className="flex justify-between gap-2">
      <div className="overflow-hidden relative bg-base-100 p-4 rounded-lg flex flex-col items-start flex-1">
        <span className="z-10">Total Entri</span>
        <span className="z-10 text-4xl font-bold">17</span>
        <span className="absolute -rotate-12 -right-10 -bottom-16 icon-size-164 material-symbols-rounded text-neutral">
          description
        </span>
      </div>
      <button
        className="overflow-hidden relative bg-base-100 p-4 rounded-lg flex flex-col items-start flex-1 hover:bg-base-300 ease-in-out duration-150"
        onClick={() => navigate("/admin/entries/company")}
      >
        <span className="z-10">Total Perusahaan</span>
        <span className="z-10 text-4xl font-bold">1200000</span>
        <span className="absolute -rotate-12 -right-10 -bottom-16 icon-size-164 material-symbols-rounded text-neutral">
          apartment
        </span>
      </button>
      <button
        className="overflow-hidden relative bg-base-100 p-4 rounded-lg flex flex-col items-start flex-1 hover:bg-base-300 ease-in-out duration-150"
        onClick={() => navigate("/admin/entries/student")}
      >
        <span className="z-10">Total Siswa</span>
        <span className="z-10 text-4xl font-bold">5</span>
        <span className="absolute -rotate-12 -right-10 -bottom-16 icon-size-164 material-symbols-rounded text-neutral">
          person
        </span>
      </button>
      <button
        className="bg-base-100 p-4 rounded-lg flex flex-col justify-center items-center flex-0 hover:bg-base-300 ease-in-out duration-150"
        onClick={() => navigate("/admin/entries/wave/0")}
      >
        Gelombang
        <span className="z-10 text-2xl font-bold">{dataWave}</span>
      </button>
    </div>
  );
}

export default Statistic;
