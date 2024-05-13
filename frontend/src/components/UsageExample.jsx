import React from "react";
import Navbar from "./Navbar";
import Sidebar from "./Sidebar";
import { useTranslation } from "react-i18next";
import Search from "./Search";

//! Delete this file when merging to master
//! Hapus file ini ketika merge ke master

function UsageExample() {
  const { t } = useTranslation();

  return (
    <div className="flex h-full">
      <Sidebar index={0} />
      <div className="flex-1 flex flex-col">
        <Navbar title={t("Dashboard")} sidebar={true} />
        <div className="flex-1 flex-nowrap bg-base-200 rounded-tl-xl p-2">
          {/* Content in here */}
          <Search />
        </div>
      </div>
    </div>
  );
}

export default UsageExample;
