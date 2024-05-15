import React, { useState } from "react";

import { Link } from "react-router-dom";
import { useTranslation } from "react-i18next";

function Sidebar({ index = -1 }) {
  const [isExpanded, setIsExpanded] = useState(true);
  const { t } = useTranslation();

  const handleExpandToggle = () => {
    setIsExpanded(!isExpanded);
  };

  return (
    <div className="flex-none p-2">
      <label className="btn btn-square btn-ghost swap swap-rotate">
        <input
          type="checkbox"
          onClick={handleExpandToggle}
        />
        <span className="swap-off fill-current material-symbols-rounded">
          arrow_back
        </span>
        <span className="swap-on fill-current material-symbols-rounded">
          menu
        </span>
      </label>
      <ul className={`menu ${isExpanded ? 'w-56' : ''} px-0 pt-4`}>
        <li>
          <Link to="/" className={`p-3 ${index === 0 && "active"}`}>
            <span className="material-symbols-rounded">dashboard</span>
            <span>{isExpanded && t("Dashboard")}</span>
          </Link>
        </li>
        <li>
          <Link to="/document" className={`p-3 ${index === 1 && "active"}`}>
            <span className="material-symbols-rounded">description</span>
            <span>{isExpanded && t("Entry and Document")}</span>
          </Link>
        </li>
        <li>
          <Link to="/users" className={`p-3 ${index === 2 && "active"}`}>
            <span className="material-symbols-rounded">manage_accounts</span>
            <span>{isExpanded && t("All Users")}</span>
          </Link>
        </li>
        <li>
          <Link to="/settings" className={`p-3 ${index === 3 && "active"}`}>
            <span className="material-symbols-rounded">settings</span>
            <span>{isExpanded && t("Settings")}</span>
          </Link>
        </li>
      </ul>
    </div>
  );
}

export default Sidebar;
