import React from "react";
import Navbar from "../components/Navbar";
import Sidebar from "../components/Sidebar";
import { useTranslation } from "react-i18next";
import Search from "../components/Search";
import Statistic from "../components/Statistic";
import Users from "../components/Tables/UsersTable";
import StUser from "../components/StUser";

function AllUsers() {
  const { t } = useTranslation();

  return (
    <>
      <Search />
      <StUser />
      <Users />
    </>
  );
}

export default AllUsers;
