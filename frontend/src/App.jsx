// src/App.js
import React from "react";
import {
  createBrowserRouter,
  Navigate,
  RouterProvider,
} from "react-router-dom";
import "./i18n";
import AllUsers from "./routes/AllUsers";
import Settings from "./routes/Settings";
import Dashboard from "./routes/Dashboard";
import EntriesAndDocuments from "./routes/EntriesAndDocuments";
import Root from "./components/Root";
import NotFound from "./routes/NotFound";
import Login from "./routes/Login";

const router = createBrowserRouter([
  {
    path: "*",
    element: <NotFound />,
  },
  {
    path: "/login",
    element: <Login />,
  },
  {
    element: <Root />,
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
  return <RouterProvider router={router} />;
}

export default App;
