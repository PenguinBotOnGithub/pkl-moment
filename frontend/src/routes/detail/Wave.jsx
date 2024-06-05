import React from 'react';
import Search from '../../components/Search';
import { useNavigate } from 'react-router-dom';
import WaveTable from '../../components/tables/WaveTable';

function Wave() {
  const navigate = useNavigate();

  function onAddHandle() {
    navigate("add");
  }

  return (
    <>
      <Search addOnClick={onAddHandle} />
      <WaveTable />
    </>
  );
}

export default Wave;
