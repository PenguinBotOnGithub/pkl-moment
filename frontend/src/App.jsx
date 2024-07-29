import React from "react";
import { createBrowserRouter, Navigate, RouterProvider } from "react-router-dom";
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
import UserAdd from "./routes/users/UserAdd";
import SearchEntry from "./routes/SearchEntry";
import SearchEntryDummy from "./routes/search/SearchEntryDummy";
import SearchEntrySiswa from "./routes/search/SearchEntrySiswa";
import Journal from "./routes/entries/Journal";
import MentorAdd from "./routes/journal/mentor/MentorAdd";
import Mentor from "./routes/journal/mentor/Mentor";


function App() {
  const cookies = new Cookies(null, { path: "/" });
  const isLoggedIn = cookies.get("access-token");

  const router = createBrowserRouter([
    { path: "*", element: <NotFound cookies={cookies} /> },
    {
      path: "login",
      element: !isLoggedIn ? <Login cookies={cookies} /> : <Navigate to="/admin" />,
    },
    {
      path: "admin",
      element: isLoggedIn ? <Root cookies={cookies} /> : <Navigate to="/login" />,
      children: [
        { path: "journal", element: <Navigate to="0" /> },
        { path: "entries", element: <Navigate to="0" /> },
        { path: "users", element: <Navigate to="0" /> },
        { path: "journal/:page", element: <Journal /> },
        { path: "entries/:page", element: <EntriesAndDocuments /> },
        { path: "users/:page", element: <AllUsers /> },
        { path: "users/add", element: <UserAdd /> },
        { path: "settings", element: <Settings cookies={cookies} /> },
        {
          path: "entries",
          children: [
            { path: ":entry/:id", element: <Entry /> },
            { path: ":entry/add", element: <EntryAdd role={cookies.get("role")} /> },
            { path: "wave/:page", element: <Wave /> },
            { path: "wave/add", element: <WaveAdd /> },
            { path: "company", element: <Company /> },
            { path: "company/add", element: <CompanyAdd /> },
            { path: "student", element: <Student /> },
            { path: "student/add", element: <StudentAdd /> },
          ],
        },
        
      ],
    },
  ]);

  return <RouterProvider router={router} />;
}

export default App;
