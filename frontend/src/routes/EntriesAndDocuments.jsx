import React from "react";
import { useTranslation } from "react-i18next";
import Search from "../components/Search";
import Statistic from "../components/count/Statistic";
import EntriesTable from "../components/tables/EntriesTable";

function EntriesAndDocuments() {
  const { t } = useTranslation();
  function onAddHandle() {
    console.log("add clicked");
  }

  return (
    <>
      <Search addOnClick={onAddHandle} />
      <Statistic />
      <EntriesTable />
    </>
  );
}

export default EntriesAndDocuments;
