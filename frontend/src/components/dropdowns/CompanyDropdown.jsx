import React, { useState, useEffect } from "react";

function CompanyDropdown({ value , setSelectedValue }) {
  const [isOpen, setIsOpen] = useState(false);
  const [searchValue, setSearchValue] = useState("");
  const [visibleCompanies, setVisibleCompanies] = useState([]);

  const items = value;

  function handleSearchChange(value) {
    setSearchValue(value);
    const searchTerm = value.toLowerCase();

    const updatedItems = items.map((item) => {
      const text = item.name.toLowerCase();
      return {
        id: item.id,
        name: item.name,
        visible: text.includes(searchTerm),
      };
    });

    setVisibleCompanies(updatedItems);
    setIsOpen(true);
  }

  function selectItem(item) {
    setSearchValue(item.name);
    setSelectedValue(item.id);
    setIsOpen(false);
  }

  return (
    <div className="relative w-full">
      <input
        type="text"
        className="input w-full"
        value={searchValue}
        onChange={(e) => handleSearchChange(e.target.value)}
        onFocus={() => {
          setIsOpen(true);
          setVisibleCompanies(
            items.map((item) => ({ ...item, visible: true }))
          );
        }}
        onBlur={() => setTimeout(() => setIsOpen(false), 200)}
      />
      {isOpen && (
        <div className="absolute bg-base-100 border-2 border-neutral rounded-lg top-12 left-0 right-0 px-4 py-3 flex flex-col gap-2">
          {visibleCompanies
            .filter((item) => item.visible)
            .map((item) => (
              <div
                key={item.id}
                className="cursor-pointer"
                onMouseDown={() => {
                  selectItem(item);
                  console.log(items);
                }}
              >
                {item.name}
              </div>
            ))}
        </div>
      )}
    </div>
  );
}

export default CompanyDropdown;
