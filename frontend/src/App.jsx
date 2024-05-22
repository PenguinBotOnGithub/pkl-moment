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
import { useCookies } from "react-cookie";
import Cookies from "universal-cookie";

function App() {
  const cookies = new Cookies(null, { path: "/" });
  const isLoggedIn = cookies.get("access-token");
  const router = createBrowserRouter([
    { path: "*", element: <NotFound /> },
    {
      path: "login",
      element: !isLoggedIn ? (
        <Login cookies={cookies} />
      ) : (
        <Navigate to="../admin" />
      ),
    },
    { path: "admin", element: <Navigate to="dashboard" /> },
    {
      path: "admin",
      element: cookies.get("access-token") ? (
        <Root />
      ) : (
        <Navigate to="../login" />
      ),
      children: [
        { path: "dashboard", element: <Dashboard /> },
        { path: "entries", element: <EntriesAndDocuments /> },
        { path: "users", element: <AllUsers /> },
        { path: "settings", element: <Settings /> },
      ],
    },
  ]);
  return <RouterProvider router={router} />;
}

export default App;
