import React from "react";
import Navbar from "../components/Navbar";
import Sidebar from "../components/Sidebar";
import { useTranslation } from "react-i18next";
import Search from "../components/Search";
import Statistic from "../components/Statistic";
import Endoc from "../components/Tables/EntriesTable";

function EntriesAndDocuments() {
  const { t } = useTranslation();

  return (
    <>
      <Search />
      <Statistic />
      <Endoc />
    </>
  );
}

export default EntriesAndDocuments;
