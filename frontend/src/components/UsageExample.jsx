import React from "react";
import Navbar from "./Navbar";
import Sidebar from "./Sidebar";
import { useTranslation } from "react-i18next";

//! Delete this file when merging to master
//! Hapus file ini ketika merge ke master

function UsageExample() {
  const { t } = useTranslation();

  return (
    <div className="flex h-full">
      <Sidebar />
      <div className="w-full">
        <Navbar title={t("Login")} sidebar={true} />
        {/*content disini*/}
      </div>
    </div>
  );
}

export default UsageExample;
