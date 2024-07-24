/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  daisyui: {
    themes: [
      {
        airdark: {
          primary: "#EC9E56",
          secondary: "#ec2939",
          accent: "#EC9E56",
          neutral: "#3a3a3a",
          "base-100": "#2c2c2c",
          "base-200": "#1b1b1b",
          "base-300": "#3a3a3a",
          info: "#1B80E5",
          success: "#35D764",
          warning: "#D9D936",
          error: "#F84F31",
        },
<<<<<<< HEAD
<<<<<<< HEAD
=======
        airlight: {
          primary: "#f8ab64",
          secondary: "#ec2939",
          accent: "#f8ab64",
          neutral: "#d2d2d2",
          "base-100": "#ededed",
          info: "#1b80e5",
          success: "#29e560",
          warning: "#d9d936",
          error: "#f84f31",
        },
>>>>>>> 49af8b3 (frontend/refactor: [AS] delete entries table and move to entries and document)
=======
>>>>>>> 291a5c7 (frontend/feat: [AS] themes, breadcrumb, simple pkl icon, UI fix, move company add table to company add)
      },
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
    ],
  },
  plugins: [require("daisyui")],
};
