import React, { useEffect, useState } from "react";
import { useLocation } from "react-router-dom";
import Navbar from "./Navbar";
import Sidebar from "./Sidebar";
import { useTranslation } from "react-i18next";
import { Outlet } from "react-router-dom";

function Root({ cookies }) {
  const { t } = useTranslation();
  const location = useLocation();
  const [breadcrumbs, setBreadcrumbs] = useState([]);

  useEffect(() => {
    const path = location.pathname;
    const pathSegments = path.split('/').filter(segment => segment);

    const breadcrumbItems = pathSegments.map((segment, index) => {
      const url = `/${pathSegments.slice(0, index + 1).join('/')}`;
      return {
        url,
        label: t(segment.replace(/-/g, ' ').toUpperCase()) || segment,
      };
    });

    setBreadcrumbs([
      ...breadcrumbItems
    ]);
  }, [location, t]);

  return (
    <div className="flex h-full" data-theme={cookies.get("theme")}>
      <Sidebar />
      <div className="flex-1 flex flex-col">
        <Navbar breadcrumbs={breadcrumbs} />
        <div className="flex-1 flex flex-col flex-nowrap bg-base-200 rounded-tl-box p-2 gap-2 overflow-x-auto">
          <Outlet />
        </div>
      </div>
    </div>
  );
}

export default Root;
