import React from 'react';

function Setelan() {
  return (
    <div className="flex flex-col items-center min-h-screen py-2 mb-2">
      
      <div className="flex justify-between gap-2 w-full max-w-xl mb-2">
      <div className="bg-base-100 text-white p-4 rounded-lg flex flex-col items-start flex-1">
        <div className="flex items-center mb-4">
          <span className="material-symbols-rounded icon-size-24">person</span>
          <span className="ml-2 text-lg font-bold">Pengguna</span>
        </div>
        <div className="w-full">
        <div className="flex justify-between items-center p-2 rounded-md border border-neutral">
  <span>Username</span>
  <div className="h-full border-l border-neutral"></div>
  
  <span>Johan Yanuar</span>
</div>

</div>

      </div>
    </div>

      <div className="flex justify-between gap-2 w-full max-w-xl mb-2">
        <div className="bg-base-100 text-white p-4 rounded-lg flex flex-col items-start flex-1">
          <div className="flex items-center">
            <span className="material-symbols-rounded icon-size-24">brush</span>
            <span className="ml-2 text-lg font-bold">Tampilan</span>
          </div>
          <div className="flex items-center justify-between mt-4 w-full">
            <span>Bahasa</span>
            <select className="select select-bordered ml-auto w-full max-w-xs">
              <option disabled selected>Indonesia</option>
              <option>English</option>
              <option>Jepang</option>
            </select>
          </div>
        </div>
      </div>

      <div className="flex justify-between gap-2 w-full max-w-xl mb-2">
        <div className="bg-base-100 text-white p-4 rounded-lg flex flex-col items-start flex-1">
          <div className="flex items-center">
            <span className="material-symbols-rounded icon-size-24">dashboard</span>
            <span className="ml-2 text-lg font-bold">Konfigurasi Dashboard</span>
          </div>
          <div className="flex items-center justify-between mt-4 w-full">
            <span>Urutkan Item Berdasarkan</span>
            <select className="select select-bordered ml-auto w-full max-w-xs">
              <option disabled selected>Indonesia</option>
              <option>English</option>
              <option>Jepang</option>
            </select>
          </div>
          <div className="flex items-center justify-between mt-4 w-full">
            <span>Max item di tabel</span>
            <button className="btn">10</button>
          </div>
          <div className="flex items-center justify-between mt-4 w-full">
            <span>Tampilan yang telah diverifikasi</span>
            <input type="checkbox" className="toggle ml-auto" checked />
          </div>
        </div>
      </div>

      <div className="flex justify-between gap-2 w-full max-w-xl">
  <div className="bg-base-100 text-red-500 p-4 rounded-lg flex flex-col items-start flex-1">
    <div className="flex items-center">
      <span className="material-symbols-rounded icon-size-24 text-red-500">logout</span>
      <span className="ml-2 text-lg font-bold text-red-500">Logout</span>
    </div>
  </div>
</div>

    </div>
  );
}

export default Setelan;