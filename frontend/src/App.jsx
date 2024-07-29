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
<<<<<<< HEAD
<<<<<<< HEAD
=======
import MentorAdd from "./routes/journal/mentor/MentorAdd";
import Mentor from "./routes/journal/mentor/Mentor";
>>>>>>> dd6f3fa (frontend/feat: [MH] Statistic Journal, Tes Mentor dan Student Journal)
=======
>>>>>>> 5340312 (frontend/feat: [AS] fix login redirect, add theme controller to navbar, move journal button 1 step down in sidebar, theme controller add dropdown style, login form wider, delete unused code)


function App() {
  const cookies = new Cookies(null, { path: "/" });
  const isLoggedIn = cookies.get("access-token");
<<<<<<< HEAD
<<<<<<< HEAD
=======
  const router = createBrowserRouter([
    { path: "*", element: <NotFound cookies={cookies} /> },
>>>>>>> 1a0cbff (frontend/fix: [AS] entry add fix, UI adjustments (sidebar active fix, entry detail, not found page), Users Table pagination limiter)

  const router = createBrowserRouter([
    { path: "*", element: <NotFound cookies={cookies} /> },
    {
      path: "login",
      element: !isLoggedIn ? (
        <Login cookies={cookies} />
      ) : (
        <Navigate to="../admin/entries" />
      ),
    },
<<<<<<< HEAD
=======

>>>>>>> 1a0cbff (frontend/fix: [AS] entry add fix, UI adjustments (sidebar active fix, entry detail, not found page), Users Table pagination limiter)
    {
      path: "admin",
<<<<<<< HEAD
      element: isLoggedIn ? <Root cookies={cookies} /> : <Navigate to="/login" />,
      children: [
        { path: "journal", element: <Navigate to="0" /> },
        { path: "entries", element: <Navigate to="0" /> },
        { path: "users", element: <Navigate to="0" /> },
        { path: "journal/:page", element: <Journal /> },
=======
      element: cookies.get("access-token") ? (
        <Root cookies={cookies} />
      ) : (
        <Navigate to="../login" />
      ),
=======

  const router = createBrowserRouter([
    { path: "*", element: <NotFound cookies={cookies} /> },
    {
      path: "login",
      element: !isLoggedIn ? (
        <Login cookies={cookies} />
      ) : (
        <Navigate to="../admin/entries" />
      ),
    },
    {
      path: "admin",
      element: isLoggedIn ? <Root cookies={cookies} /> : <Navigate to="/login" />,
>>>>>>> dd6f3fa (frontend/feat: [MH] Statistic Journal, Tes Mentor dan Student Journal)
      children: [
        { path: "journal", element: <Navigate to="0" /> },
        { path: "entries", element: <Navigate to="0" /> },
        { path: "users", element: <Navigate to="0" /> },
<<<<<<< HEAD
>>>>>>> 291a5c7 (frontend/feat: [AS] themes, breadcrumb, simple pkl icon, UI fix, move company add table to company add)
=======
        { path: "journal/:page", element: <Journal /> },
>>>>>>> dd6f3fa (frontend/feat: [MH] Statistic Journal, Tes Mentor dan Student Journal)
        { path: "entries/:page", element: <EntriesAndDocuments /> },
        { path: "users/:page", element: <AllUsers /> },
        { path: "users/add", element: <UserAdd /> },
        { path: "settings", element: <Settings cookies={cookies} /> },
<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> dd6f3fa (frontend/feat: [MH] Statistic Journal, Tes Mentor dan Student Journal)
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
        
<<<<<<< HEAD
=======
      ],
    },

    {
      path: "admin/entries",
      element: cookies.get("access-token") ? (
        <Root cookies={cookies}/>
      ) : (
        <Navigate to="../login" />
      ),
      children: [
        // { path: "search", element: <SearchEntry /> },
        // { path: "searchdummy", element: <SearchEntryDummy /> },
        // { path: "search/siswa", element: <SearchEntrySiswa /> },
        { path: ":entry/:id", element: <Entry /> },
        { path: ":entry/add", element: <EntryAdd role={cookies.get("role")} /> },
        { path: "wave/:page", element: <Wave /> },
        { path: "wave/add", element: <WaveAdd /> },
        { path: "company", element: <Company /> },
        { path: "company/add", element: <CompanyAdd /> },
        { path: "student", element: <Student /> },
        { path: "student/add", element: <StudentAdd /> },
>>>>>>> 291a5c7 (frontend/feat: [AS] themes, breadcrumb, simple pkl icon, UI fix, move company add table to company add)
=======
>>>>>>> dd6f3fa (frontend/feat: [MH] Statistic Journal, Tes Mentor dan Student Journal)
      ],
    },
  ]);

  return <RouterProvider router={router} />;
}

export default App;
