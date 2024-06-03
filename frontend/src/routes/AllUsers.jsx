import React from "react";
import { useTranslation } from "react-i18next";
import Search from "../components/Search";
import Users from "../components/tables/UsersTable";
import StatisticUser from "../components/count/StatisticUser";

function AllUsers() {
  const { t } = useTranslation();
  function onAddHandle() {
    console.log("add clicked");
  }

  return (
    <>
      <Search addOnClick={onAddHandle} />
      <StatisticUser />
      <Users />
    </>
  );
}

export default AllUsers;
