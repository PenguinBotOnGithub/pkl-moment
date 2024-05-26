import React from "react";
import Navbar from "../components/Navbar";
import Sidebar from "../components/Sidebar";
import { useTranslation } from "react-i18next";
import Search from "../components/Search";
import Statistic from "../components/Statistic";
import Data from "../components/Tables/DashboardTable";

function Dashboard() {
  const { t } = useTranslation();
  

  return (
    <>
      <Search />
      <Statistic />
      <Data />
    </>
  );
}

export default Dashboard;
