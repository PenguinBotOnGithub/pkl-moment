import React, { useState } from 'react';

function Users() {
  const [selectedRows, setSelectedRows] = useState([]);

  const handleSelectRow = (rowIndex) => {
    if (selectedRows.includes(rowIndex)) {
      setSelectedRows(selectedRows.filter(index => index !== rowIndex));
    } else {
      setSelectedRows([...selectedRows, rowIndex]);
    }
  };

  const allRows = [
    { id: 1, pembimbing: 'Cy Ganderton', jenisEntri: 'Quality Control Specialist', tanggalPermintaan: 'Blue', verifikasi: 'tidak ada entri' },
    { id: 2, pembimbing: 'Cy Ganderton', jenisEntri: 'Quality Control Specialist', tanggalPermintaan: 'Blue', verifikasi: 'tidak ada entri' },
    { id: 3, pembimbing: 'Cy Ganderton', jenisEntri: 'Quality Control Specialist', tanggalPermintaan: 'Blue', verifikasi: 'tidak ada entri' }
  ];

  return (
    <div className="overflow-x-auto">
      <div className="flex justify-between items-center mb-2">
        <span className="text-lg">Permintaan Verifikasi</span>
        <div className="flex gap-2">
          <button
            className={`btn bg-green-500 btn-sm text-black ${selectedRows.length === 0 ? 'opacity-50 cursor-not-allowed' : ''}`}
            disabled={selectedRows.length === 0}
          >
            <span>Hapus yang terpilih</span>
          </button>
        </div>
      </div>
      <table className="table bg-base-100 overflow-hidden rounded-xl">
        <thead className="bg-neutral">
          <tr>
            <th>
              <input
                type="checkbox" className="checkbox"
                onChange={(e) => {
                  if (e.target.checked) {
                    setSelectedRows(allRows.map((_, index) => index));
                  } else {
                    setSelectedRows([]);
                  }
                }}
                checked={selectedRows.length === allRows.length}
              />
            </th>
            <th>No</th>
            <th>Pembimbing</th>
            <th>Jenis Entri</th>
            <th>Tanggal Permintaan</th>
            <th>Verifikasi</th>
            <th>Aksi</th>
          </tr>
        </thead>
        <tbody>
          {allRows.map((row, index) => (
            <tr key={row.id} className='border-t-2 border-neutral'>
              <td>
                <input
                  type="checkbox" className="checkbox"
                  onChange={() => handleSelectRow(index)}
                  checked={selectedRows.includes(index)}
                />
              </td>
              <th>{row.id}</th>
              <td>{row.pembimbing}</td>
              <td>{row.jenisEntri}</td>
              <td>{row.tanggalPermintaan}</td>
              <td className="text-gray-500">{row.verifikasi}</td>
              <td>
                
                <button className="btn btn-warning btn-xs rounded-lg mr-2">Ganti Password</button>
                <button className="btn btn-error btn-xs rounded-lg">Hapus</button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
      <div className="flex justify-center items-center gap-2 mt-4">
        <button className="flex-none btn bg-base-100">
          <span className="material-symbols-rounded icon-size-20">arrow_back</span>
        </button>
        <div className="join flex gap-2">
          <button className="join-item btn">1</button>
          <button className="join-item btn">2</button>
          <button className="join-item btn btn-disabled">...</button>
          <button className="join-item btn">99</button>
          <button className="join-item btn">100</button>
        </div>
        <button className="flex-none btn bg-base-100">
          <span className="material-symbols-rounded icon-size-20">arrow_forward</span>
        </button>
      </div>
    </div>
  );
}

export default Users;
