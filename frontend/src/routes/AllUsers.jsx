import React from 'react';
import Navbar from '../components/Navbar';
import Sidebar from '../components/Sidebar';
import { useTranslation } from 'react-i18next';
import Search from '../components/Search';

function AllUsers() {
  const { t } = useTranslation();

  return (
    <div className="flex h-full">
      <Sidebar index={2} />
      <div className="flex-1 flex flex-col">
        <Navbar title={t('All Users')} sidebar={true} />
        <div className="flex-1 flex-nowrap bg-base-200 rounded-tl-xl p-2">
          {/* Content in here */}
          <Search />
        </div>
      </div>
    </div>
  );
}

export default AllUsers;
