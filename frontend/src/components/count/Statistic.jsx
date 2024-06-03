import React from "react";

function Statistic() {
  return (
    <div className="flex justify-between gap-2 ">
      <div className="overflow-hidden relative bg-base-100 p-4 rounded-lg flex flex-col items-start flex-1">
        Total Entri
        <div>
          <span className="text-4xl font-bold">17</span>
        </div>
        <span className="absolute -rotate-12 -right-10 -bottom-16 icon-size-164 material-symbols-rounded text-neutral">description</span>
      </div>
      <button className="overflow-hidden relative bg-base-100 p-4 rounded-lg flex flex-col items-start flex-1 hover:bg-base-300 ease-in-out duration-150">
        Total Perusahaan
        <div>
          <span className="text-4xl font-bold">12</span>
        </div>
        <span className="absolute -rotate-12 -right-10 -bottom-16 icon-size-164 material-symbols-rounded text-neutral">apartment</span>
      </button>
      <button className="overflow-hidden relative bg-base-100 p-4 rounded-lg flex flex-col items-start flex-1 hover:bg-base-300 ease-in-out duration-150">
        Total Siswa
        <div>
          <span className="text-4xl font-bold">5</span>
        </div>
        <span className="absolute -rotate-12 -right-10 -bottom-16 icon-size-164 material-symbols-rounded text-neutral">person</span>
      </button>
    </div>
  );
}

export default Statistic;
