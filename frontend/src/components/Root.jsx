import React, { useEffect, useState } from "react";
import { useLocation } from "react-router-dom";
import Navbar from "./Navbar";
import Sidebar from "./Sidebar";
import { useTranslation } from "react-i18next";
import { Outlet } from "react-router-dom";

function Root() {
  const { t } = useTranslation();
  const location = useLocation();
  const [title, setTitle] = useState("");

  useEffect(() => {
    if (location.pathname === "/admin/dashboard") {
      setTitle(t("Dashboard"));
    } else if (location.pathname === "/admin/entries") {
      setTitle(t("Entries & Document"));
    } else if (/^\/admin\/entries\/[^/]+\/\d+$/.test(location.pathname)) {
      setTitle(t("Entries & Document > Detail"));
    } else if (/^\/admin\/entries\/[^/]+\/add$/.test(location.pathname)) {
      setTitle(t("Entries & Document > Add"));
    } else if (location.pathname === "/admin/entries/company") {
      setTitle(t("Entries & Document > Perusahaan"));
    } else if (/^\/admin\/entries\/company\/add$/.test(location.pathname)) {
      setTitle(t("Entries & Document > Perusahaan > Tambah"));
    } else if (location.pathname === "/admin/entries/student") {
      setTitle(t("Entries & Document > Siswa"));
    } else if (/^\/admin\/entries\/student\/add$/.test(location.pathname)) {
      setTitle(t("Entries & Document > Siswa > Tambah"));
    } else if (location.pathname === "/admin/entries/wave") {
      setTitle(t("Entries & Document > Gelombang"));
    } else if (/^\/admin\/entries\/wave\/add$/.test(location.pathname)) {
      setTitle(t("Entries & Document > Gelombang > Tambah"));
    } else if (location.pathname === "/admin/users") {
      setTitle(t("All Users"));
    } else if (location.pathname === "/admin/settings") {
      setTitle(t("Settings"));
    } else {
      setTitle(t("Default Title"));
    }
  }, [location, t]);

  return (
    <div className="flex h-full">
      <Sidebar />
      <div className="flex-1 flex flex-col">
        <Navbar title={title} sidebar={true} />
        <div className="flex-1 flex flex-col flex-nowrap bg-base-200 rounded-tl-xl p-2 gap-2 overflow-x-auto">
          <Outlet />
        </div>
      </div>
    </div>
  );
}

export default Root;
