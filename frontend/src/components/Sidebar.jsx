import React, { useState } from "react";
import { useTranslation } from "react-i18next";

function Sidebar({ index = 0 }) {
  const [isExpanded, setIsExpanded] = useState(true);
  const { t } = useTranslation();

  return (
    <div className="flex-none p-2">
      <label className="btn btn-square btn-ghost swap swap-rotate">
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
      <ul className={`menu ${isExpanded && `w-56`} px-0 pt-2`}>
        <li>
          <a className={`p-3 ${index == 0 && "active"}`}>
            <span className="material-symbols-rounded">dashboard</span>
            {isExpanded && <span>{t("Dashboard")}</span>}
          </a>
        </li>
        <li>
          <a className={`p-3 ${index == 1 && "active"}`}>
            <span className="material-symbols-rounded">description</span>
            {isExpanded && t("Entry and Document")}
          </a>
        </li>
        <li>
          <a className={`p-3 ${index == 2 && "active"}`}>
            <span className="material-symbols-rounded">manage_accounts</span>
            {isExpanded && t("All Users")}
          </a>
        </li>
        <li>
          <a className={`p-3 ${index == 3 && "active"}`}>
            <span className="material-symbols-rounded">settings</span>
            {isExpanded && t("Settings")}
          </a>
        </li>
      </ul>
    </div>
  );
}

export default Sidebar;
