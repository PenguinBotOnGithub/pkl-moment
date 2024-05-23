import React from "react";
import Navbar from "../components/Navbar";
import Sidebar from "../components/Sidebar";
import { useTranslation } from "react-i18next";
import Search from "../components/Search";
import Statistic from "../components/Statistic";
import Data from "../components/Tables/DashboardTable";

function Dashboard() {
  const { t } = useTranslation();
  const dummy = [
    { id:1, pembimbing: 'Cy Ganderton', jenisEntri: 'Surat Pengantaran', tanggalPermintaan: '23/05/2024', verifikasi: true },
    { id:2, pembimbing: 'Cy Ganderton', jenisEntri: 'Surat Pengantaran', tanggalPermintaan: '23/05/2024', verifikasi: false },
    { id:3, pembimbing: 'Cy Ganderton', jenisEntri: 'Surat Pengantaran', tanggalPermintaan: '23/05/2024', verifikasi: true }
  ];

  return (
    <>
      <Search />
      <Statistic />
      <Data data={dummy} />
    </>
  );
}

export default Dashboard;
