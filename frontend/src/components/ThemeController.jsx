import React, { useRef, useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import Cookies from "universal-cookie";

function ThemeController({ maxWidth = "xs", style = "join" }) {
  const cookies = new Cookies(null, { path: "/" });
  const [theme, setTheme] = useState(cookies.get("theme"));
  const navigate = useNavigate();

  const themeRefs = useRef({});

  const onOptionChange = (e) => {
    setTheme(e.target.value);
    cookies.set("theme", e.target.value);
    navigate("#");
  };

  useEffect(() => {
    if (themeRefs.current[theme]) {
      themeRefs.current[theme].scrollIntoView({
        behavior: "smooth",
        block: "nearest",
      });
    }
  }, [theme]);

  const themeData = [
    "airdark",
    "light",
    "dark",
    "cupcake",
    "bumblebee",
    "emerald",
    "corporate",
    "synthwave",
    "retro",
    "cyberpunk",
    "valentine",
    "halloween",
    "garden",
    "forest",
    "aqua",
    "lofi",
    "pastel",
    "fantasy",
    "wireframe",
    "black",
    "luxury",
    "dracula",
    "cmyk",
    "autumn",
    "business",
    "acid",
    "lemonade",
    "night",
    "coffee",
    "winter",
    "dim",
    "nord",
    "sunset",
  ];

  if (style == "join") {
    return (
      <div
        className={`overflow-x-auto max-w-${maxWidth} rounded-t-btn bg-base-200`}
      >
        <div className="join join-horizontal rounded-none">
          {themeData.map((singletheme, index) => (
            <input
              key={index}
              ref={(el) => (themeRefs.current[singletheme] = el)}
              type="radio"
              name="theme-buttons"
              className="btn theme-controller join-item"
              aria-label={singletheme}
              value={singletheme}
              checked={theme === singletheme}
              onChange={onOptionChange}
            />
          ))}
        </div>
      </div>
    );
  }
  if (style == "dropdown") {
    return (
      <div className="dropdown dropdown-end">
        <div tabIndex={0} role="button" className="btn btn-square btn-ghost">
          <span className="material-symbols-rounded">format_paint</span>
        </div>
        <ul
          tabIndex={0}
          className="dropdown-content bg-base-300 rounded-box z-[100] w-52 p-2 shadow-2xl overflow-y-auto h-[50vh]"
        >
          {themeData.map((singletheme, index) => (
            <input
              key={index}
              ref={(el) => (themeRefs.current[singletheme] = el)}
              type="radio"
              name="theme-buttons"
              className="theme-controller btn btn-sm btn-block btn-ghost justify-start"
              aria-label={singletheme}
              value={singletheme}
              checked={theme === singletheme}
              onChange={onOptionChange}
            />
          ))}
        </ul>
      </div>
    );
  }
}

export default ThemeController;
