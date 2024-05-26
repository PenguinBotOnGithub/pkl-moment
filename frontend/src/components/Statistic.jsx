import React from 'react';

function Statistic() {
  return (
    <div className="flex justify-between gap-2 ">
      <div className="bg-base-100 text-white p-4 rounded-lg flex flex-col items-start flex-1">
        <div>
          <span className="text-blue-500">Total Entri</span>
        </div>
        <div>
          <span className="text-4xl font-bold">17</span>
        </div>
      </div>
      <div className="bg-base-100 text-white p-4 rounded-lg flex flex-col items-start flex-1">
        <div>
          <span className="text-green-500">Entri Terverifikasi</span>
        </div>
        <div>
          <span className="text-4xl font-bold">12</span>
        </div>
      </div>
      <div className="bg-base-100 text-white p-4 rounded-lg flex flex-col items-start flex-1">
        <div>
          <span className="text-yellow-500">Entri Belum Terverifikasi</span>
        </div>
        <div>
          <span className="text-4xl font-bold">5</span>
        </div>
      </div>
    </div>
  );
}

export default Statistic;
