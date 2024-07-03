import React from "react";
import { useTranslation } from "react-i18next";
import Search from "../../components/Search";
import Statistic from "../../components/count/Statistic";
import EntriesTable from "../../components/tables/entries/EntriesTable";
import { useNavigate } from "react-router-dom";

function EntriesAndDocuments() {
  const { t } = useTranslation();

  return (
    <>
      <EntriesTable />
    </>
  );
}

export default EntriesAndDocuments;
