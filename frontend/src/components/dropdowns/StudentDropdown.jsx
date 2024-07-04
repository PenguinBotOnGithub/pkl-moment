import React, { useState, useEffect } from "react";

function StudentDropdown({ value, setSelectedValue }) {
  const [isOpen, setIsOpen] = useState(false);
  const [searchValue, setSearchValue] = useState("");
  const [visibleStudent, setVisibleStudent] = useState([]);

  const items = value;

  useEffect(() => {
    setVisibleStudent(items.map((item) => ({ ...item, visible: true })));
  }, []);

  function handleSearchChange(value) {
    setSearchValue(value);
    const searchTerm = value.toLowerCase();

    const updatedItems = items.map((item) => {
      const text = item.username.toLowerCase();
      return {
        id: item.id,
        username: item.username,
        visible: text.includes(searchTerm),
      };
    });

    setVisibleStudent(updatedItems);
    setIsOpen(true);
  }

  function selectItem(item) {
    setSearchValue(item.username);
    setSelectedValue(item.id);
    setIsOpen(false);
  }

  return (
    <div className="relative w-full">
      <label className="input w-full flex items-center">
        <input
          type="text"
          className="input w-full"
          value={searchValue}
          onChange={(e) => handleSearchChange(e.target.value)}
          onFocus={() => {
            setIsOpen(true);
            setVisibleStudent(
              items.map((item) => ({ ...item, visible: true }))
            );
          }}
          onBlur={() => setTimeout(() => setIsOpen(false), 200)}
        />
        <button className="btn btn-sm btn-ghost px-0 material-symbols-rounded icon-size-20 opacity-50">
          tune
        </button>
      </label>
      {isOpen && (
        <div className="absolute max-w-sm w-full bg-base-100 border-2 border-neutral rounded-lg -bottom-18 right-0 px-4 py-3 flex flex-col gap-2 z-10">
          {visibleStudent.map((student) => (
            <div
              key={student.id}
              className="cursor-pointer flex flex-row justify-between"
              onMouseDown={() => {
                onAddRow(student.id, student.name, student.class);
                setSearchStudentValue("");
                setIsOpenStudent(false);
              }}
            >
              <div>{student.name}</div>
              <div>{student.class}</div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}

export default StudentDropdown;
