import React from "react";
import { useTranslation } from "react-i18next";
import Search from "../components/Search";
import Statistic from "../components/Statistic";
import Endoc from "../components/Tables/EntriesTable";

function EntriesAndDocuments() {
  const { t } = useTranslation();
  function onAddHandle() {
    console.log("add clicked");
  }

  return (
    <>
      <Search addOnClick={onAddHandle}/>
      <Statistic />
      <Endoc />
    </>
  );
}

export default EntriesAndDocuments;
