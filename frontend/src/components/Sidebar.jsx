import React, { useEffect, useState } from "react";

import { NavLink } from "react-router-dom";
import { useTranslation } from "react-i18next";
import Cookies from "universal-cookie";

function Sidebar({ index = -1 }) {
  const [isExpanded, setIsExpanded] = useState(true);
  const { t } = useTranslation();
  const cookies = new Cookies(null, { path: "/" });
  const role = cookies.get("role");

  const handleExpandToggle = () => {
    setIsExpanded(!isExpanded);
  };

  useEffect(()=>{
    console.log(role);
  },[]);

  return (
    <div className="flex-none p-2">
      <label className="btn btn-square btn-ghost swap swap-rotate">
        <input type="checkbox" onClick={handleExpandToggle} />
        <span className="swap-off fill-current material-symbols-rounded">
          arrow_back
        </span>
        <span className="swap-on fill-current material-symbols-rounded">
          menu
        </span>
      </label>
      <ul className={`menu ${isExpanded ? "w-56" : ""} px-0 pt-4 gap-2`}>
        {/* <li>
          <NavLink
            to="/admin/dashboard"
            className={({ isActive }) => (isActive ? "active p-3" : "p-3")}
          >
            <span className="material-symbols-rounded">dashboard</span>
            {isExpanded && t("Dashboard")}
          </NavLink>
        </li> */}
        <li>
          <NavLink
            to="/admin/entries"
            className={({ isActive }) => (isActive ? "active p-3" : "p-3")}
          >
            <span className="material-symbols-rounded">description</span>
            {isExpanded && t("Entries & Document")}
          </NavLink>
        </li>
        {role == "admin" && <li>
          <NavLink
            to="/admin/users"
            className={({ isActive }) => (isActive ? "active p-3" : "p-3")}
          >
            <span className="material-symbols-rounded">people</span>
            {isExpanded && t("All Users")}
          </NavLink>
        </li>}
        <li>
          <NavLink
            to="/admin/settings"
            className={({ isActive }) => (isActive ? "active p-3" : "p-3")}
          >
            <span className="material-symbols-rounded">settings</span>
            {isExpanded && t("Settings")}
          </NavLink>
        </li>
      </ul>
    </div>
  );
}

export default Sidebar;
