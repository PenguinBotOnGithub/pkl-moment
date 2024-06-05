import React, { useState } from "react";

function WaveTable() {
  const [selectedIds, setSelectedIds] = useState(1);


  const data = [
    { id: 1, tahunPembelajaran: '2024/2025', startDate: '10 Juli 2024', endDate: '12 Juni 2025' },
    { id: 2, tahunPembelajaran: '2023/2024', startDate: '10 Juli 2024', endDate: '12 Juni 2025' },
    { id: 3, tahunPembelajaran: '2022/2023', startDate: '10 Juli 2024', endDate: '12 Juni 2025' },
    { id: 4, tahunPembelajaran: '2021/2022', startDate: '10 Juli 2024', endDate: '12 Juni 2025' }
  ];

  const handleSelect = (id) => {
    setSelectedIds(id);
  };

  return (
    <div className="overflow-x-auto">
      <table className="table bg-base-100 border-0 overflow-hidden rounded-lg">
        <thead className="bg-neutral">
          <tr className="border-0">
            <th className="w-0">No</th>
            <th>Tahun Pembelajaran</th>
            <th>Start Date</th>
            <th>End Date</th>
            <th>Terpilih</th>
            <th>Aksi</th>
          </tr>
        </thead>
        <tbody className="box-content">
          {data.map((row, index) => (
            <tr key={row.id} className="border-t-2 border-neutral">
              <td>{index + 1}</td>
              <td>{row.tahunPembelajaran}</td>
              <td>{row.startDate}</td>
              <td>{row.endDate}</td>
              <td>
                {selectedIds == row.id ? (
                  <span className="opacity-60">Terpilih</span>
                ) : (
                  <button
                    className="btn btn-success btn-xs rounded-lg mr-2"
                    onClick={() => handleSelect(row.id)}
                  >
                    Pilih
                  </button>
                )}
              </td>
              <td>
                <button className="btn btn-info btn-xs rounded-lg mr-2">
                  Edit
                </button>
                <button className="btn btn-error btn-xs rounded-lg">
                  Delete
                </button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
      <div className="flex justify-center items-center gap-2 mt-4">
        <button className="flex-none btn bg-base-100">
          <span className="material-symbols-rounded icon-size-20">
            arrow_back
          </span>
        </button>
        <div className="join flex gap-2">
          <button className="join-item btn">1</button>
          <button className="join-item btn">2</button>
          <button className="join-item btn opacity-50">...</button>
          <button className="join-item btn">99</button>
          <button className="join-item btn">100</button>
        </div>
        <button className="flex-none btn bg-base-100">
          <span className="material-symbols-rounded icon-size-20">
            arrow_forward
          </span>
        </button>
      </div>
    </div>
  );
}

export default WaveTable;
