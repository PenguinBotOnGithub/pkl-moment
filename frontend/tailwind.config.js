/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  daisyui: {
    themes: [
      {
        airdark: {
          primary: "#EC9E56",
          secondary: "#EC9E56",
          accent: "#EC9E56",
          neutral: "#292929",
          "base-100": "#222222",
          info: "#1B80E5",
          success: "#35D764",
          warning: "#D9D936",
          error: "#F84F31",
        },
        airlight: {
          primary: "#f8ab64",
          secondary: "#f8ab64",
          accent: "#f8ab64",
          neutral: "#d2d2d2",
          "base-100": "#ededed",
          info: "#1b80e5",
          success: "#29e560",
          warning: "#d9d936",
          error: "#f84f31",
        },
      },
    ],
  },
  plugins: [require("daisyui")],
};
