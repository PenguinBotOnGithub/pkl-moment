import React from 'react'
import StudentTable from '../../components/tables/StudentTable';
import Search from "../../components/Search";
import { useNavigate } from 'react-router-dom';

function Student() {
  const navigate = useNavigate();
  function onAddHandle() {
    navigate("/admin/entries/student/add");
  }
  return (    
    <>
    <Search addOnClick={onAddHandle}/>

    <StudentTable/>
    
    </> 
    

   
  )
}

export default Student;