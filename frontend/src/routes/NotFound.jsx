import React from "react";
import Navbar from "../components/Navbar";
import NotFoundMessage from "../assets/drawable/NotFoundMessage";
import { Link } from "react-router-dom";

function NotFound({cookies}) {
  return (
<<<<<<< HEAD
    <div className="flex h-full" data-theme={cookies.get("theme")}>
      <div className="flex-1 flex flex-col justify-center items-center bg-base-100" >
        <NotFoundMessage />
        <Link to="/admin/entries/0" className="btn btn-primary -mt-10 max-w-xl w-full" >Help i'm lost</Link>
=======
    <div className="flex h-full flex-col">
      <Navbar title="Are you lost?" href="/admin/entries/0" />
      <div className="flex-1 flex justify-center items-center bg-base-100">
        <NotFoundMessage />
>>>>>>> 49af8b3 (frontend/refactor: [AS] delete entries table and move to entries and document)
      </div>
    </div>
  );
}

export default NotFound;
