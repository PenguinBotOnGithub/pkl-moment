import React from 'react'
import Search from '../../components/Search'
import { useNavigate } from 'react-router-dom'

function Wave() {
  const navigate = useNavigate()
  function onAddHandle(){
    navigate("add");
  }
  return (
    <Search addOnClick={onAddHandle} />
  )
}

export default Wave