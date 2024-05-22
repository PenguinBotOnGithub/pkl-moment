import React from "react";

function Navbar({ title = "Title", href = "" }) {
  return (
    <div className="navbar flex-none pl-0">
      <div className="flex-1">
        <a className="btn btn-ghost text-2xl" href={href}>{title}</a>
      </div>
      <div className="flex-none">
        <label className="btn btn-square btn-ghost swap swap-rotate">
          <input type="checkbox" className="theme-controller" value="airlight" />
          <span className="swap-off fill-current material-symbols-rounded">
            dark_mode
          </span>
          <span className="swap-on fill-current material-symbols-rounded">
            light_mode
          </span>
        </label>
      </div>
    </div>
  );
}

export default Navbar;
