import React, { useEffect, useState } from "react";

function StudentEntryAddTable({
  rows,
  onAddRow,
  onDeleteRow,
  onSearchStudent,
}) {
  const [searchStudentValue, setSearchStudentValue] = useState("");
  const [isOpenStudent, setIsOpenStudent] = useState(false);
  const [visibleStudents, setVisibleStudents] = useState([]);

  useEffect(() => {
    onSearchStudent("", setVisibleStudents);
  }, [onSearchStudent]);

  function handleSearchChange(value) {
    setSearchStudentValue(value);
    onSearchStudent(value, setVisibleStudents);
    setIsOpenStudent(true);
  }

  return (
    <table className="table bg-base-100 border-0 rounded-lg max-w-screen-sm">
      <thead className="">
        <tr className="border-0">
          <th className="w-0">No</th>
          <th>Nama Siswa</th>
          <th>Aksi</th>
        </tr>
      </thead>
      <tbody className="box-content">
        {rows.map((row, index) => (
          <tr key={index} className="border-t-2 border-neutral ">
            <td>{index + 1}</td>
            <td>{row.name}</td>
            <td>
              <div
                className="btn btn-error btn-xs rounded-lg mr-2"
                onMouseDown={() => onDeleteRow(index)}
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
              onFocus={() => setIsOpenStudent(true)}
              onBlur={() => setTimeout(() => setIsOpenStudent(false), 200)}
              style={{
                backgroundColor: "transparent",
                border: "none",
                outline: "none",
              }}
            />
            {isOpenStudent && (
              <div className="absolute bg-base-100 border-2 border-neutral rounded-lg -bottom-18 left-16 right-10 px-4 py-3 flex flex-col gap-2">
                {visibleStudents.map((student) => (
                  <div
                    key={student.id}
                    className="cursor-pointer"
                    onMouseDown={() => {
                      onAddRow(student.name);
                      setSearchStudentValue("");
                      setIsOpenStudent(false);
                    }}
                  >
                    {student.name}
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
