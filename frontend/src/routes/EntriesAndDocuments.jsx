import React from "react";
import { useTranslation } from "react-i18next";
import Search from "../components/Search";
import Statistic from "../components/count/Statistic";
import EntriesTable from "../components/tables/EntriesTable";
import { useNavigate } from "react-router-dom";

function EntriesAndDocuments() {
  const { t } = useTranslation();
  const navigate = useNavigate();
  function onAddHandle() {
    navigate("/admin/entries/add");
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
