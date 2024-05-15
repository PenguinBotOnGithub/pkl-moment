import React from "react";
import Navbar from "../components/Navbar";
import Sidebar from "../components/Sidebar";
import { useTranslation } from "react-i18next";
import Search from "../components/Search";

function Dashboard() {
  const { t } = useTranslation();

  return (
    <>
      <Search />
    </>
  );
}

export default Dashboard;