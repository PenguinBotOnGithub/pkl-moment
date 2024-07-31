import React, { useState, useEffect, useRef } from "react";

function Dropdown({
  items = [],
  displayFields,
  searchField,
  setSelectedValue,
  size = "xl",
}) {
  const [isOpen, setIsOpen] = useState(false);
  const [searchValue, setSearchValue] = useState("");
  const [visibleItems, setVisibleItems] = useState([]);
  const inputRef = useRef(null);

  useEffect(() => {
    setVisibleItems(items.map((item) => ({ ...item, visible: true })));
  }, [items]);

  function handleSearchChange(value) {
    setSearchValue(value);
    const searchTerm = value.toLowerCase();

    const updatedItems = items.map((item) => {
      const text = item[searchField].toLowerCase();
      return {
        ...item,
        visible: text.includes(searchTerm),
      };
    });

    setVisibleItems(updatedItems);
    setIsOpen(true);
  }

  function selectItem(item) {
    setSearchValue(item[searchField]);
    setSelectedValue(item.id);
    setIsOpen(false);
  }

  useEffect(() => {
    function handleClickOutside(event) {
      if (inputRef.current && !inputRef.current.contains(event.target)) {
        setIsOpen(false);
      }
    }
    document.addEventListener("mousedown", handleClickOutside);
    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
    };
  }, [inputRef]);

  return (
    <div className="relative w-full" ref={inputRef}>
      {size === "xl" ? (
        <input
          type="text"
          className="input w-full"
          value={searchValue}
          onChange={(e) => handleSearchChange(e.target.value)}
          onFocus={() => {
            setIsOpen(true);
            setVisibleItems(items.map((item) => ({ ...item, visible: true })));
          }}
        />
      ) : (
        <input
          type="text"
          style={{
            backgroundColor: "transparent",
            border: "none",
            outline: "none",
          }}
          value={searchValue}
          onChange={(e) => handleSearchChange(e.target.value)}
          onFocus={() => {
            setIsOpen(true);
            setVisibleItems(items.map((item) => ({ ...item, visible: true })));
          }}
        />
      )}
      {isOpen && (
        <div className="dropdown-content bg-base-300 rounded-box z-[100] w-52 p-2 shadow-2xl overflow-y-auto max-h-[50vh] fixed">
          {visibleItems
            .filter((item) => item.visible)
            .map((item) => (
              <div
                key={item.id}
                className="cursor-pointer flex flex-row justify-between btn btn-sm btn-block btn-ghost"
                onMouseDown={() => selectItem(item)}
              >
                {displayFields.map((field, index) => (
                  <div key={index}>{item[field]}</div>
                ))}
              </div>
            ))}
        </div>
      )}
    </div>
  );
}

export default Dropdown;
