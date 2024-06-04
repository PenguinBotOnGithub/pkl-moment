import React from "react";

function CompanyTable() {
  const data = [
    { id: 1, namaSiswa: 'Student A', kelas: 'Class A', nis: '6576' },
    { id: 2, namaSiswa: 'Student B', kelas: 'Class B', nis: '8767' },
    { id: 3, namaSiswa: 'Student C', kelas: 'Class C', nis: '6545' }
  ];

  return (
    <div className="overflow-x-auto">
      <table className="table bg-base-100 border-0 overflow-hidden rounded-lg">
        <thead className="bg-neutral">
          <tr className="border-0">
            <th>No</th>
            <th>Nama Siswa</th>
            <th>Kelas</th>
            <th>NIS</th>
            <th>Aksi</th>
          </tr>
        </thead>
        <tbody className="box-content">
          {data.map((row, index) => (
            <tr key={row.id} className="border-t-2 border-neutral">
              <td>{index + 1}</td>
              <td>{row.namaSiswa}</td>
              <td>{row.kelas}</td>
              <td>{row.nis}</td>
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

export default CompanyTable;
