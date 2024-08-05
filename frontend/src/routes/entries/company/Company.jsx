import React from "react";
import CompanyTable from "../../../components/tables/company/CompanyTable";
import Search from "../../../components/Search";
import { useNavigate } from "react-router-dom";

function Company() {
  const navigate = useNavigate();
  function onAddHandle() {
    navigate("/admin/entries/company/add");
  }
  return (
    <>
      <Search addOnClick={onAddHandle} />
      <CompanyTable />
    </>
  );
}

export default Company;
