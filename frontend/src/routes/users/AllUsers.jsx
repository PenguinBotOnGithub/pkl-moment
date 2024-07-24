import React from "react";
import { useTranslation } from "react-i18next";
import Search from "../../components/Search";
import Users from "../../components/tables/users/UsersTable";
import StatisticUser from "../../components/count/StatisticUser";
import { useNavigate } from "react-router-dom";

function AllUsers() {
  const { t } = useTranslation();
  const navigate = useNavigate();
  function onAddHandle() {
    navigate("/admin/users/add");
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
