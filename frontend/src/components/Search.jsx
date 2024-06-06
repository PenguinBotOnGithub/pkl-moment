import React from "react";
import getCurrentDate from "../assets/strings/getCurrentDate";
import Clock from "../assets/strings/Clock";

function Search({ addOnClick }) {
  return (
    <div className="flex flex-row gap-2 justify-between">
      <div className="flex flex-row w-full gap-2">
        <label className="input w-full max-w-screen-sm flex items-center gap-2">
          <input
            type="text"
            className="grow placeholder:text-neutral-content placeholder:opacity-50"
            placeholder="Search"
          />
          <button className="btn btn-sm btn-ghost px-0 material-symbols-rounded icon-size-20 opacity-50">
            tune
          </button>
        </label>
      </div>
      <div className="flex flex-row-reverse flex-none gap-2">
        {addOnClick && (
          <button className="flex-none btn bg-base-100" onClick={addOnClick}>
            <span className="material-symbols-rounded icon-size-20">add</span>
            <span className="hidden lg:block">Add</span>
          </button>
        )}
        <a className="flex-none btn bg-base-100" href="#edit_modal">
          <span className="material-symbols-rounded icon-size-20">edit</span>
          <span className="hidden lg:block">Edit</span>
        </a>
        <dialog id="edit_modal" className="modal">
          <div className="modal-box">
            <h3 className="font-bold text-lg">Hello!</h3>
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
          <span className="hidden lg:block">Refresh</span>
        </button>
      </div>
    </div>
  );
}

export default Search;
