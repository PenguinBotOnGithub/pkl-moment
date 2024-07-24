import React, { useRef, useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import Cookies from "universal-cookie";

function ThemeController({ cookies }) {
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
      themeRefs.current[theme].scrollIntoView({ behavior: "smooth", block: "nearest" });
    }
  }, [theme]);

  const themeData = [
    "airdark", "light", "dark", "cupcake", "bumblebee", "emerald",
    "corporate", "synthwave", "retro", "cyberpunk", "valentine",
    "halloween", "garden", "forest", "aqua", "lofi", "pastel",
    "fantasy", "wireframe", "black", "luxury", "dracula", "cmyk",
    "autumn", "business", "acid", "lemonade", "night", "coffee",
    "winter", "dim", "nord", "sunset",
  ];

  return (
    <div className="overflow-x-auto max-w-xs rounded-t-btn bg-base-200">
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

export default ThemeController;
