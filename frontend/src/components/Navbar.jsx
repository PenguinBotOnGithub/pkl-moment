import React from "react";

function Navbar({ title = "Title" }) {
  return (
    <div className="navbar flex-none pl-0">
      <div className="flex-1">
        <a className="btn btn-ghost text-2xl">{title}</a>
      </div>
      <div className="flex-none">
        <div className="btn btn-square btn-ghost">
          <label className="swap swap-rotate">
            <input type="checkbox" class="theme-controller" value="airlight" />
            <span className="swap-off fill-current material-symbols-rounded">
              dark_mode
            </span>
            <span className="swap-on fill-current material-symbols-rounded">
              light_mode
            </span>
          </label>
        </div>
      </div>
    </div>
  );
}

export default Navbar;
