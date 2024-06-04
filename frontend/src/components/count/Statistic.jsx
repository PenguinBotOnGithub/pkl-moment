import React from "react";
import { useNavigate } from "react-router-dom";

function Statistic() {
  const navigate = useNavigate();

  const handleCompanyButtonClick = () => {
    navigate("/admin/entries/company");
  };

  const handleStudentButtonClick = () => {
    navigate("/admin/entries/student");
  };

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
        onClick={handleCompanyButtonClick}
      >
        <span className="z-10">Total Perusahaan</span>
        <span className="z-10 text-4xl font-bold">1200000</span>
        <span className="absolute -rotate-12 -right-10 -bottom-16 icon-size-164 material-symbols-rounded text-neutral">
          apartment
        </span>
      </button>
      <button
        className="overflow-hidden relative bg-base-100 p-4 rounded-lg flex flex-col items-start flex-1 hover:bg-base-300 ease-in-out duration-150"
        onClick={handleStudentButtonClick}
      >
        <span className="z-10">Total Siswa</span>
        <span className="z-10 text-4xl font-bold">5</span>
        <span className="absolute -rotate-12 -right-10 -bottom-16 icon-size-164 material-symbols-rounded text-neutral">
          person
        </span>
      </button>
      <button
        className="bg-base-100 p-4 rounded-lg flex flex-col justify-center items-center flex-0 hover:bg-base-300 ease-in-out duration-150"
        onClick={() => {}}
      >
        Gelombang
        <span className="z-10 text-2xl font-bold">2023/2024</span>
      </button>
    </div>
  );
}

export default Statistic;
