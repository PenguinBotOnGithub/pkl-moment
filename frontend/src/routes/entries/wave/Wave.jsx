import React from "react";
import Search from "../../../components/Search";
import { useNavigate } from "react-router-dom";
import WaveTable from "../../../components/tables/wave/WaveTable";

function Wave() {
  const navigate = useNavigate();

  function onAddHandle() {
    navigate("/admin/entries/wave/add");
  }

  return (
    <>
      <Search addOnClick={onAddHandle} />
      <WaveTable />
    </>
  );
}

export default Wave;
