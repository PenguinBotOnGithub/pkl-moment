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
import Cookies from "universal-cookie";
import Company from "./routes/detail/Company";
import Student from "./routes/detail/Student";
import CompanyAdd from "./routes/detail/CompanyAdd";
import StudentAdd from "./routes/detail/StudentAdd";
import Entry from "./routes/detail/Entry";
import EntryAdd from "./routes/detail/EntryAdd";
import Wave from "./routes/detail/Wave";
import WaveAdd from "./routes/detail/WaveAdd";

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
        { path: "settings", element: <Settings cookies={cookies} /> },
      ],
    },

    {
      path: "admin/entries",
      element: cookies.get("access-token") ? (
        <Root />
      ) : (
        <Navigate to="../login" />
      ),
      children: [
        { path: ":entry/:id", element: <Entry /> },
        { path: ":entry/add", element: <EntryAdd /> },
        { path: "wave", element: <Wave /> },
        { path: "wave/add", element: <WaveAdd /> },
        { path: "company", element: <Company /> },
        { path: "company/add", element: <CompanyAdd /> },
        { path: "student", element: <Student /> },
        { path: "student/add", element: <StudentAdd /> },
      ],
    },
  ]);
  return <RouterProvider router={router} />;
}

export default App;
