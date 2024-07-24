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

  useEffect(() => {
    console.log(role);
  }, []);

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
        <li>
          <NavLink
            to="/admin/entries/0"
            className={({ isActive }) => (isActive ? "active p-3" : "p-3")}
          >
            <span className="material-symbols-rounded">description</span>
            {isExpanded && t("Entries & Document")}
          </NavLink>
        </li>
<<<<<<< HEAD
        <li>
=======
        {role == "admin" && <li>
>>>>>>> 49af8b3 (frontend/refactor: [AS] delete entries table and move to entries and document)
          <NavLink
            to="/admin/journal"
            className={({ isActive }) => (isActive ? "active p-3" : "p-3")}
          >
<<<<<<< HEAD
            <span className="material-symbols-rounded">book_2</span>
            {isExpanded && t("Journal")}
=======
            <span className="material-symbols-rounded">people</span>
            {isExpanded && t("All Users")}
>>>>>>> 291a5c7 (frontend/feat: [AS] themes, breadcrumb, simple pkl icon, UI fix, move company add table to company add)
          </NavLink>
        </li>
        {role == "admin" && (
          <li>
            <NavLink
              to="/admin/users"
              className={({ isActive }) => (isActive ? "active p-3" : "p-3")}
            >
              <span className="material-symbols-rounded">people</span>
              {isExpanded && t("All Users")}
            </NavLink>
          </li>
        )}
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
