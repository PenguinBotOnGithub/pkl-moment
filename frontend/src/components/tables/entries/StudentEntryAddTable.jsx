import React, { useEffect, useState } from "react";

function StudentEntryAddTable({
  rows,
  onAddRow,
  onDeleteRow,
  onSearchStudent,
  isMaxWidth,
  items,
}) {
  const [searchStudentValue, setSearchStudentValue] = useState("");
  const [isOpenStudent, setIsOpenStudent] = useState(false);
  const [visibleStudents, setVisibleStudents] = useState([]);

  function handleSearchChange(value) {
    setSearchStudentValue(value);
    if (items && items.length > 0) {  // Only filter if items are available
      onSearchStudent(value, setVisibleStudents, items);
      if (value.trim() !== "") {
        setIsOpenStudent(true);
      }
    }
  }

  return (
    <table
      className={`table bg-base-100 border-0 rounded-box overflow-hidden ${
        isMaxWidth && "max-w-screen-sm"
      }`}
    >
      <thead className="relative">
        <tr className="border-0 bg-base-300">
          <th className="w-0 z-20">No</th>
          
          <th>Nama Siswa</th>
          <th>Kelas</th>
          <th>Aksi</th>
        </tr>
      </thead>
      <tbody className="box-content">
        {rows.map((row, index) => (
          <tr key={index} className="border-t-2 border-base-300 ">
            <td>{index + 1}</td>
            <td>{row.name}</td>
            <td>{row.grade}</td>
            <td>
              <div
                className="btn btn-error btn-xs mr-2"
                onMouseDown={() => {
                  onDeleteRow(index);
                }}
              >
                Delete
              </div>
            </td>
          </tr>
        ))}
        <tr>
          <td>
            <span className="material-symbols-rounded icon-size-16">
              search
            </span>
          </td>
          <td>
            <input
              type="text"
              value={searchStudentValue}
              onChange={(e) => handleSearchChange(e.target.value)}
              onFocus={() => {
                onSearchStudent("", setVisibleStudents, items);
              }}
              onBlur={() => setTimeout(() => setIsOpenStudent(false), 200)}
              style={{
                backgroundColor: "transparent",
                border: "none",
                outline: "none",
              }}
            />
            {isOpenStudent && (
              <div className="absolute bg-base-100 border-2 border-neutral rounded-lg -bottom-18 left-16 right-10 px-4 py-3 flex flex-col gap-2 z-10">
                {visibleStudents.map((student) => (
                  <div
                    key={student.id}
                    className="cursor-pointer flex flex-row justify-between"
                    onMouseDown={() => {
                      onAddRow(student.id, student.name, student.grade);
                      setSearchStudentValue("");
                      setIsOpenStudent(false);
                    }}
                  >
                    <div>{student.name}</div>
                    <div>{student.grade}</div>
                  </div>
                ))}
              </div>
            )}
          </td>
          <td></td>
        </tr>
      </tbody>
    </table>
  );
}

export default StudentEntryAddTable;
