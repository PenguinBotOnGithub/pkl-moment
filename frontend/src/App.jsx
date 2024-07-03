import React from "react";
import {
  createBrowserRouter,
  Navigate,
  RouterProvider,
} from "react-router-dom";
import "./i18n";
import AllUsers from "./routes/users/AllUsers";
import Settings from "./routes/Settings";
import EntriesAndDocuments from "./routes/entries/EntriesAndDocuments";
import Root from "./components/Root";
import NotFound from "./routes/NotFound";
import Login from "./routes/Login";
import Cookies from "universal-cookie";
import Company from "./routes/entries/company/Company";
import Student from "./routes/entries/student/Student";
import CompanyAdd from "./routes/entries/company/CompanyAdd";
import StudentAdd from "./routes/entries/student/StudentAdd";
import Entry from "./routes/entries/Entry";
import EntryAdd from "./routes/entries/EntryAdd";
import Wave from "./routes/entries/wave/Wave";
import WaveAdd from "./routes/entries/wave/WaveAdd";
import User from "./routes/users/User";
import UserAdd from "./routes/users/UserAdd";
import SearchEntry from "./routes/SearchEntry";
import SearchEntryDummy from "./routes/search/SearchEntryDummy";
import SearchEntrySiswa from "./routes/search/SearchEntrySiswa";

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

    { path: "admin", element: <Navigate to="entries/0" /> },

    {
      path: "admin",
      element: cookies.get("access-token") ? (
        <Root />
      ) : (
        <Navigate to="../login" />
      ),
      children: [
        { path: "entries/:page", element: <EntriesAndDocuments /> },
        { path: "users/:page", element: <AllUsers /> },
        { path: "user", element: <User /> },
        { path: "user/add", element: <UserAdd /> },
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
        { path: "search", element: <SearchEntry /> },
        { path: "searchdummy", element: <SearchEntryDummy /> },
        { path: "search/siswa", element: <SearchEntrySiswa /> },
        { path: ":entry/:id", element: <Entry /> },
        { path: ":entry/add", element: <EntryAdd /> },
        { path: "wave/:page", element: <Wave /> },
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
