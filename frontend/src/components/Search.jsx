import React from "react";
import getCurrentDate from "../assets/strings/getCurrentDate";
import Clock from "../assets/strings/Clock";

function Search({addOnClick}) {
  return (
    <div className="flex flex-row gap-2 justify-between">
      <div className="flex flex-row flex-none gap-2">
        <label className="input flex items-center gap-2">
          <input
            type="text"
            className="grow placeholder:text-neutral-content placeholder:opacity-50"
            placeholder="Search"
          />
          <span className="material-symbols-rounded icon-size-20 opacity-50">
            Search
          </span>
        </label>
        <button className="flex-none btn bg-base-100">
          {getCurrentDate("/")}
        </button>
        <button className="flex-none btn w-[93px] p-0 bg-base-100 gap-1">
          {Clock()}
        </button>
      </div>
      <div className="flex flex-row-reverse flex-none gap-2">
        {addOnClick && <button className="flex-none btn bg-base-100" onClick={addOnClick}>
          <span className="material-symbols-rounded icon-size-20">add</span>
          <span className="hidden lg:block">Add</span>
        </button>}
        <button className="flex-none btn bg-base-100">
          <span className="material-symbols-rounded icon-size-20">edit</span>
          <span className="hidden lg:block">Edit</span>
        </button>
        <button className="flex-none btn bg-base-100">
          <span className="material-symbols-rounded icon-size-20">refresh</span>
          <span className="hidden lg:block">Refresh</span>
        </button>
      </div>
    </div>
  );
}

export default Search;
