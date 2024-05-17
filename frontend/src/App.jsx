// src/App.js
import React from "react";
import { Routes, Route } from "react-router-dom";
import "./i18n";
import AllUsers from "./routes/AllUsers";
import Settings from "./routes/Settings";
import Dashboard from "./routes/Dashboard";
import EntriesAndDocuments from "./routes/EntriesAndDocuments";
import Root from "./components/Root";
import { LoginPage } from "./routes/Login";
import { ProtectedRoute } from "./components/ProtectedRoute";

const routes = Routes([
  {
    path: "login",
    element: <LoginPage />
  },
  {
    path: "/",
    element: (
      <ProtectedRoute>
        <Root />
      </ProtectedRoute>
    ),
    children: [
      {
        path: "dashboard",
        element: <Dashboard />,
      },
      {
        path: "entries",
        element: <EntriesAndDocuments />,
      },
      {
        path: "users",
        element: <AllUsers />,
      },
      {
        path: "settings",
        element: <Settings />,
      },
    ],
  },
]);

function App() {
  return <RouterProvider routes={routes} />;
}

export default App