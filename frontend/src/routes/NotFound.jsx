import React from "react";
import Navbar from "../components/Navbar";
import NotFoundMessage from "../assets/drawable/NotFoundMessage";
import { Link } from "react-router-dom";

function NotFound({cookies}) {
  return (
    <div className="flex h-full" data-theme={cookies.get("theme")}>
      <div className="flex-1 flex flex-col justify-center items-center bg-base-100" >
        <NotFoundMessage />
        <Link to="/admin/entries/0" className="btn btn-primary -mt-10 max-w-xl w-full" >Help i'm lost</Link>
      </div>
    </div>
  );
}

export default NotFound;
