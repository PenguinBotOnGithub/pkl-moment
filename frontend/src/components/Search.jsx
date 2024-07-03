import React from "react";
import getCurrentDate from "../assets/strings/getCurrentDate";
import Clock from "../assets/strings/Clock";
import { useNavigate } from "react-router-dom";

function Search({ addOnClick }) {
  const navigate = useNavigate();
  return (
    <div className="flex flex-row gap-2 justify-between">
      <div className="flex flex-row w-full gap-2">
        <button className="flex-none btn bg-base-100" onClick={()=>{navigate("/admin/entries/search")}}>
          <span className="material-symbols-rounded icon-size-20">filter_alt</span>
          <span className="hidden md:block">Search by filter</span>
        </button>
      </div>
      <div className="flex flex-row-reverse flex-none gap-2">
        {addOnClick && (
          <button className="flex-none btn bg-base-100" onClick={addOnClick}>
            <span className="material-symbols-rounded icon-size-20">add</span>
            <span className="hidden md:block">Add</span>
          </button>
        )}
        <a className="flex-none btn bg-base-100" href="#edit_modal">
          <span className="material-symbols-rounded icon-size-20">edit</span>
          <span className="hidden md:block">Edit</span>
        </a>
        <dialog id="edit_modal" className="modal">
          <div className="modal-box">
            <h3 className="font-bold text-md">Hello!</h3>
            <p className="py-4">
              You can edit by simply clicking on any data you wanna edit
            </p>
            <div className="modal-action">
              <a href="#" className="btn">
                Yay!
              </a>
            </div>
          </div>
          <form method="dialog" className="modal-backdrop">
            <button>close</button>
          </form>
        </dialog>
        <button className="flex-none btn bg-base-100">
          <span className="material-symbols-rounded icon-size-20">refresh</span>
          <span className="hidden md:block">Refresh</span>
        </button>
      </div>
    </div>
  );
}

export default Search;
