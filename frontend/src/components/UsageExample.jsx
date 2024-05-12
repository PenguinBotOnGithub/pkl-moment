import React from "react";
import Navbar from "./Navbar";
import Sidebar from "./Sidebar";

//! Delete this file when merging to master
//! Hapus file ini ketika merge ke master

function UsageExample() {
  return (
    <div className="flex h-full">
      <Sidebar />
      <div className="w-full">
        <Navbar title="Login" sidebar={true} />
        {/*content disini*/}
      </div>
    </div>
  );
}

export default UsageExample;
