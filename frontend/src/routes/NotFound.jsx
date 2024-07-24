import React from "react";
import Navbar from "../components/Navbar";
import NotFoundMessage from "../assets/drawable/NotFoundMessage";

function NotFound() {
  return (
    <div className="flex h-full flex-col">
      <Navbar title="Are you lost?" href="/admin/entries/0" />
      <div className="flex-1 flex justify-center items-center bg-base-100">
        <NotFoundMessage />
      </div>
    </div>
  );
}

export default NotFound;
