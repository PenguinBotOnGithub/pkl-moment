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
<<<<<<< HEAD
<<<<<<< HEAD
=======
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
            to="/admin/journal"
            className={({ isActive }) => (isActive ? "active p-3" : "p-3")}
          >
            <span className="material-symbols-rounded">book</span>
            {isExpanded && t("Journal")}
          </NavLink>
        </li>
>>>>>>> dd6f3fa (frontend/feat: [MH] Statistic Journal, Tes Mentor dan Student Journal)
=======
>>>>>>> 5340312 (frontend/feat: [AS] fix login redirect, add theme controller to navbar, move journal button 1 step down in sidebar, theme controller add dropdown style, login form wider, delete unused code)
        <li>
          <NavLink
            to="/admin/entries"
            className={({ isActive }) => (isActive ? "active p-3" : "p-3")}
          >
            <span className="material-symbols-rounded">description</span>
            {isExpanded && t("Entries & Document")}
          </NavLink>
        </li>
<<<<<<< HEAD
<<<<<<< HEAD
        <li>
=======
        {role == "admin" && <li>
>>>>>>> 49af8b3 (frontend/refactor: [AS] delete entries table and move to entries and document)
          <NavLink
<<<<<<< HEAD
            to="/admin/journal"
=======
            to="/admin/users"
>>>>>>> 1a0cbff (frontend/fix: [AS] entry add fix, UI adjustments (sidebar active fix, entry detail, not found page), Users Table pagination limiter)
            className={({ isActive }) => (isActive ? "active p-3" : "p-3")}
          >
<<<<<<< HEAD
            <span className="material-symbols-rounded">book_2</span>
            {isExpanded && t("Journal")}
=======
            <span className="material-symbols-rounded">people</span>
            {isExpanded && t("All Users")}
>>>>>>> 291a5c7 (frontend/feat: [AS] themes, breadcrumb, simple pkl icon, UI fix, move company add table to company add)
=======
        <li>
          <NavLink
            to="/admin/journal"
            className={({ isActive }) => (isActive ? "active p-3" : "p-3")}
          >
            <span className="material-symbols-rounded">book_2</span>
            {isExpanded && t("Journal")}
>>>>>>> 5340312 (frontend/feat: [AS] fix login redirect, add theme controller to navbar, move journal button 1 step down in sidebar, theme controller add dropdown style, login form wider, delete unused code)
          </NavLink>
        </li>
        {role == "secretary" && (
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
