import React from "react";
import { useTranslation } from "react-i18next";
import Search from "../components/Search";
import Statistic from "../components/count/Statistic";
import DashboardTable from "../components/tables/DashboardTable";

function Dashboard() {
  const { t } = useTranslation();

  return (
    <>
      <Search />
      <Statistic />
      <DashboardTable/>
    </>
  );
}

export default Dashboard;
