import React from "react";
import { Link } from "react-router-dom";
<<<<<<< HEAD
import ThemeController from "./ThemeController";
=======
>>>>>>> 291a5c7 (frontend/feat: [AS] themes, breadcrumb, simple pkl icon, UI fix, move company add table to company add)

// Utility function to convert text to sentence case
const toSentenceCase = (text) => {
  if (!text) return text;
  return text.charAt(0).toUpperCase() + text.slice(1).toLowerCase();
};

// Utility function to check if a value is a number
const isNumber = (value) => !isNaN(value) && isFinite(value);

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
                  <Link to={breadcrumb.url}>
                    {toSentenceCase(breadcrumb.label)}
                  </Link>
=======
                  <Link to={breadcrumb.url}>{toSentenceCase(breadcrumb.label)}</Link>
>>>>>>> 291a5c7 (frontend/feat: [AS] themes, breadcrumb, simple pkl icon, UI fix, move company add table to company add)
                )}
              </li>
            ))}
          </ul>
        </div>
      </div>
      <div className="flex-none">
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
      </div>
    </div>
  );
}

export default Navbar;
