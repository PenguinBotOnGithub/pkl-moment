import React from "react";
import Search from "../../../components/Search";
import { useNavigate } from "react-router-dom";
import MentorTable from "../../../components/tables/mentor/MentorTable";

function Mentor() {
  const navigate = useNavigate();
  
  function onAddHandle() {
    navigate("/admin/journal/mentor/add");
  }

  return (
    <>
      <Search addOnClick={onAddHandle} />
      <MentorTable />
    </>
  );
}

export default Mentor;
