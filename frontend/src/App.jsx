// src/App.js
import React from 'react';
import { createBrowserRouter, RouterProvider } from 'react-router-dom';
import UsageExample from './components/UsageExample';
import './i18n';
import EntryDocument from './routes/EntryDocument';
import AllUsers from './routes/AllUsers';
import Settings from './routes/Settings';

const router = createBrowserRouter([
  {
    path: '/',
    element: <UsageExample />,
  },
  {
    path: '/document',
    element: <EntryDocument />,
  },

  {
    path: '/users',
    element: <AllUsers />,
  },

  {
    path: '/settings',
    element: <Settings />,
  },
]);

function App() {
  return <RouterProvider router={router} />;
}

export default App;
