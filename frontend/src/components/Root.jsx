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
    
    switch (location.pathname) {
      case "/admin/dashboard":
        setTitle(t("Dashboard"));
        break;
      case "/admin/entries":
        setTitle(t("Entries & Document"));
        break;
      case "/admin/users":
        setTitle(t("All Users"));
        break;
      case "/admin/settings":
        setTitle(t("Settings"));
        break;
      case "/admin/entries/company":
          setTitle(t("Entries & Document > Perusahaan"));
          break;
          case "/admin/entries/company/add":
            setTitle(t("Entries & Document > Perusahaan > Tambah"));
            break;
      
      default:
        setTitle(t("Default Title"));
    }
  }, [location, t]);

  return (
    <div className="flex h-full">
      <Sidebar />
      <div className="flex-1 flex flex-col">
        <Navbar title={title} sidebar={true} />
        <div className="flex-1 flex flex-col flex-nowrap bg-base-200 rounded-tl-xl p-2 gap-2">
          <Outlet />
        </div>
      </div>
    </div>
  );
}

export default Root;
