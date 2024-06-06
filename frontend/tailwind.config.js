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
          neutral: "#3a3a3a",
          "base-100": "#2c2c2c",
          "base-200": "#1b1b1b",
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
