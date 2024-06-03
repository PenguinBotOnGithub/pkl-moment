import React from "react";
import { useTranslation } from "react-i18next";
import Search from "../components/Search";
import Endoc from "../components/tables/EntriesTable";
import Statistic from "../components/count/Statistic";

function EntriesAndDocuments() {
  const { t } = useTranslation();
  function onAddHandle() {
    console.log("add clicked");
  }

  return (
    <>
      <Search addOnClick={onAddHandle} />
      <Statistic />
      <Endoc />
    </>
  );
}

export default EntriesAndDocuments;
