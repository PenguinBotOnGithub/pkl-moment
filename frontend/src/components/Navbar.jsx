import React from "react";
import { Link } from "react-router-dom";
<<<<<<< HEAD
<<<<<<< HEAD
import ThemeController from "./ThemeController";
=======
>>>>>>> 291a5c7 (frontend/feat: [AS] themes, breadcrumb, simple pkl icon, UI fix, move company add table to company add)
=======
import ThemeController from "./ThemeController";
>>>>>>> 5340312 (frontend/feat: [AS] fix login redirect, add theme controller to navbar, move journal button 1 step down in sidebar, theme controller add dropdown style, login form wider, delete unused code)

// Utility function to convert text to sentence case
const toSentenceCase = (text) => {
  if (!text) return text;
  return text.charAt(0).toUpperCase() + text.slice(1).toLowerCase();
};

// Utility function to check if a value is a number
const isNumber = (value) => !isNaN(value) && isFinite(value);

<<<<<<< HEAD
<<<<<<< HEAD
function Navbar({ breadcrumbs = [], cookies }) {
  // Filter out unwanted values (e.g., "ADMIN" and numbers)
  const filteredBreadcrumbs = breadcrumbs.filter(
    (breadcrumb) => breadcrumb.label !== "ADMIN" && !isNumber(breadcrumb.label)
=======
function Navbar({ breadcrumbs = [] }) {
  // Filter out unwanted values (e.g., "ADMIN" and numbers)
  const filteredBreadcrumbs = breadcrumbs.filter(breadcrumb => 
    breadcrumb.label !== "ADMIN" && !isNumber(breadcrumb.label)
>>>>>>> 291a5c7 (frontend/feat: [AS] themes, breadcrumb, simple pkl icon, UI fix, move company add table to company add)
=======
function Navbar({ breadcrumbs = [], cookies }) {
  // Filter out unwanted values (e.g., "ADMIN" and numbers)
  const filteredBreadcrumbs = breadcrumbs.filter(
    (breadcrumb) => breadcrumb.label !== "ADMIN" && !isNumber(breadcrumb.label)
>>>>>>> 5340312 (frontend/feat: [AS] fix login redirect, add theme controller to navbar, move journal button 1 step down in sidebar, theme controller add dropdown style, login form wider, delete unused code)
  );

  return (
    <div className="navbar flex-none">
      <div className="flex-1">
        <div className="breadcrumbs text-md">
          <ul>
            {filteredBreadcrumbs.map((breadcrumb, index) => (
              <li key={index}>
                {index === filteredBreadcrumbs.length - 1 ? (
                  toSentenceCase(breadcrumb.label)
                ) : (
<<<<<<< HEAD
<<<<<<< HEAD
                  <Link to={breadcrumb.url}>
                    {toSentenceCase(breadcrumb.label)}
                  </Link>
=======
                  <Link to={breadcrumb.url}>{toSentenceCase(breadcrumb.label)}</Link>
>>>>>>> 291a5c7 (frontend/feat: [AS] themes, breadcrumb, simple pkl icon, UI fix, move company add table to company add)
=======
                  <Link to={breadcrumb.url}>
                    {toSentenceCase(breadcrumb.label)}
                  </Link>
>>>>>>> 5340312 (frontend/feat: [AS] fix login redirect, add theme controller to navbar, move journal button 1 step down in sidebar, theme controller add dropdown style, login form wider, delete unused code)
                )}
              </li>
            ))}
          </ul>
        </div>
      </div>
      <div className="flex-none">
<<<<<<< HEAD
<<<<<<< HEAD
        <ThemeController cookies={cookies} maxWidth="sm" style="dropdown" />
=======
        <label className="btn btn-square btn-ghost swap swap-rotate">
          <input
            type="checkbox"
            className="theme-controller"
            value="airlight"
          />
          <span className="swap-off fill-current material-symbols-rounded">
            dark_mode
          </span>
          <span className="swap-on fill-current material-symbols-rounded">
            light_mode
          </span>
        </label>
>>>>>>> 291a5c7 (frontend/feat: [AS] themes, breadcrumb, simple pkl icon, UI fix, move company add table to company add)
=======
        <ThemeController cookies={cookies} maxWidth="sm" style="dropdown" />
>>>>>>> 5340312 (frontend/feat: [AS] fix login redirect, add theme controller to navbar, move journal button 1 step down in sidebar, theme controller add dropdown style, login form wider, delete unused code)
      </div>
    </div>
  );
}

export default Navbar;
