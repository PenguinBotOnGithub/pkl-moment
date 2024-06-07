import React, { useState, useEffect } from "react";

function AdviserDropdown({ value , setSelectedValue }) {
  const [isOpen, setIsOpen] = useState(false);
  const [searchValue, setSearchValue] = useState("");
  const [visibleAdvisers, setVisibleAdvisers] = useState([]);

  const items = value;

  useEffect(() => {
    setVisibleAdvisers(items.map((item) => ({ ...item, visible: true })));
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

    setVisibleAdvisers(updatedItems);
    setIsOpen(true);
  }

  function selectItem(item) {
    setSearchValue(item.username);
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
          setVisibleAdvisers(
            items.map((item) => ({ ...item, visible: true }))
          );
        }}
        onBlur={() => setTimeout(() => setIsOpen(false), 200)}
      />
      {isOpen && (
        <div className="absolute bg-base-100 border-2 border-neutral rounded-lg top-12 left-0 right-0 px-4 py-3 flex flex-col gap-2 z-20">
          {visibleAdvisers
            .filter((item) => item.visible)
            .map((item) => (
              <div
                key={item.id}
                className="cursor-pointer"
                onMouseDown={() => selectItem(item)}
              >
                {item.username}
              </div>
            ))}
        </div>
      )}
    </div>
  );
}

export default AdviserDropdown;
