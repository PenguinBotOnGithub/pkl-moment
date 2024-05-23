
import React from 'react';
import Navbar from '../components/Navbar';
import Sidebar from '../components/Sidebar';
import { useTranslation } from 'react-i18next';
import Search from '../components/Search';
import Setelan from '../components/Setelan';

function Settings() {
  const { t } = useTranslation();

  return (
    <>
      
      <Setelan />
    </>
  );
}

export default Settings;
