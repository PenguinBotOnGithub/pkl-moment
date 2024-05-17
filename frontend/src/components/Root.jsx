import React from "react";
import Navbar from "./Navbar";
import Sidebar from "./Sidebar";
import { useTranslation } from "react-i18next";
import Search from "./Search";
import { Outlet } from "react-router-dom";

function Root() {
  const { t } = useTranslation();

  return (
    <div className="flex h-full">
      <Sidebar />
      <div className="flex-1 flex flex-col">
        <Navbar title={t("Dashboard")} sidebar={true} />
        <div className="flex-1 flex flex-col flex-nowrap bg-base-200 rounded-tl-xl p-2 gap-2">
          <Outlet />
        </div>
      </div>
    </div>
  );
}

export default Root;
