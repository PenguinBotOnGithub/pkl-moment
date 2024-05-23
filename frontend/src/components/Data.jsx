import React, { useState } from 'react';

function Data() {
  const [selectedRows, setSelectedRows] = useState([]);

  const handleSelectRow = (rowIndex) => {
    if (selectedRows.includes(rowIndex)) {
      setSelectedRows(selectedRows.filter(index => index !== rowIndex));
    } else {
      setSelectedRows([...selectedRows, rowIndex]);
    }
  };

  const allRows = [
    { id: 1, pembimbing: 'Cy Ganderton', jenisEntri: 'Quality Control Specialist', tanggalPermintaan: 'Blue', verifikasi: 'Terverifikasi' },
    { id: 2, pembimbing: 'Cy Ganderton', jenisEntri: 'Quality Control Specialist', tanggalPermintaan: 'Blue', verifikasi: 'Terverifikasi' },
    { id: 3, pembimbing: 'Cy Ganderton', jenisEntri: 'Quality Control Specialist', tanggalPermintaan: 'Blue', verifikasi: 'Terverifikasi' }
  ];

  return (
    <div className="overflow-x-auto">
      <div className="flex justify-between items-center mb-2">
        <span className="text-lg">Permintaan Verifikasi</span>
        <div className="flex gap-2 ">
          <button
            className={`btn bg-green-500 btn-sm text-black ${selectedRows.length === 0 ? 'opacity-50 cursor-not-allowed' : ''}`}
            disabled={selectedRows.length === 0}
          >
            <span>Verifikasi yang terpilih</span>
          </button>
          <button
            className={`btn bg-yellow-500 btn-sm text-black ${selectedRows.length === 0 ? 'opacity-50 cursor-not-allowed' : ''}`}
            disabled={selectedRows.length === 0}
          >
            <span>Export yang terpilih</span>
          </button>
          <button className="btn bg-blue-500 btn-sm text-black">
            <span>Lihat lebih</span>
          </button>
        </div>
      </div>
      <table className="table bg-base-100 border-0 overflow-hidden rounded-lg ">
        <thead className="border-2 border-neutral bg-neutral box-content">
          <tr className='border-0'>
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
            <th>Data Input</th>
            <th>Verifikasi</th>
          </tr>
        </thead>
        <tbody className="box-content">
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
              <td>
                <button className="btn btn-info btn-xs rounded-lg mr-2">Detail</button>
              </td>
              <td className="">{row.verifikasi}</td>
            </tr>
          ))}
        </tbody>
      </table>
      
    </div>
  );
}

export default Data;
