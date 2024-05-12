import React, { useState } from "react";

function Sidebar({ index = 0 }) {
  const [isExpanded, setIsExpanded] = useState(true);

  return (
    <div className="flex-none p-2">
      <div className="btn btn-square btn-ghost">
        <label className="swap swap-rotate">
          <input
            type="checkbox"
            onClick={() => {
              setIsExpanded(!isExpanded);
              console.log(isExpanded);
            }}
          />
          <span className="swap-off fill-current material-symbols-rounded">
            arrow_back
          </span>
          <span className="swap-on fill-current material-symbols-rounded">
            menu
          </span>
        </label>
      </div>
      <ul className={`menu ${isExpanded && `w-56`} px-0 pt-2`}>
        <li>
          <a className={`p-3 ${index == 0 && "active"}`}>
            <span className="material-symbols-rounded">dashboard</span>
            {isExpanded && <span>Dashboard</span>}
          </a>
        </li>
        <li>
          <a className={`p-3 ${index == 1 && "active"}`}>
            <span className="material-symbols-rounded">description</span>
            {isExpanded && <span>Entry and Document</span>}
          </a>
        </li>
        <li>
          <a className={`p-3 ${index == 2 && "active"}`}>
            <span className="material-symbols-rounded">manage_accounts</span>
            {isExpanded && <span>Users</span>}
          </a>
        </li>
        <li>
          <a className={`p-3 ${index == 3 && "active"}`}>
            <span className="material-symbols-rounded">settings</span>
            {isExpanded && <span>Settings</span>}
          </a>
        </li>
      </ul>
    </div>
  );
}

export default Sidebar;
