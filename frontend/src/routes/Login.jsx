import React, { useState } from "react";
import { useNavigate } from "react-router-dom"; // To navigate after login
import Navbar from "../components/Navbar";
import PKLMomentIcon from "../assets/drawable/PKLMomentIcon";
import getCurrentDate from "../assets/strings/getCurrentDate";
import { login } from '../services/index'; // Import the login function from the api.js file

function Login({ cookies }) {
  const [formData, setFormData] = useState({
    username: "",
    password: "",
  });
  const navigate = useNavigate();

  const handleInputChange = (e) => {
    const { name, value } = e.target;
    setFormData({
      ...formData,
      [name]: value,
    });
  };

  const handleSubmit = async (e) => {
    e.preventDefault();

    try {
      const result = await login(formData);
      if (result.status === "success") {
        cookies.set("access-token", result.data.token);
        cookies.set("max-item", 10);
        cookies.set("selected-wave", 2);
        cookies.set("role", result.data.role);
        cookies.set("user-id", result.data.id);
        cookies.set("user-name", result.data.username);
        location.reload();
      }
    } catch (error) {
      alert("Please check your login information.");
      console.error("Login failed:", error);
    }

    console.log("Form data submitted:", formData);
  };

  return (
<<<<<<< HEAD
<<<<<<< HEAD
    <div className="flex flex-col size-full" data-theme={cookies.get("theme")}>
      <Navbar cookies={cookies} />
=======
    <div className="flex flex-col size-full" data-theme="airdark">
      <Navbar title="Login" />
>>>>>>> 291a5c7 (frontend/feat: [AS] themes, breadcrumb, simple pkl icon, UI fix, move company add table to company add)
=======
    <div className="flex flex-col size-full" data-theme={cookies.get("theme")}>
      <Navbar cookies={cookies} />
>>>>>>> 5340312 (frontend/feat: [AS] fix login redirect, add theme controller to navbar, move journal button 1 step down in sidebar, theme controller add dropdown style, login form wider, delete unused code)
      <div className="flex-1 flex flex-col gap-2 justify-center items-center bg-base-100 p-4 pb-14">
        <div className="flex items-center mb-6">
          <PKLMomentIcon size={64} />
          <span className="text-4xl text-primary">PKL</span>
          <span className="text-4xl text-secondary">Moment</span>
        </div>
        {/* The form */}
        <form onSubmit={handleSubmit} className="flex flex-col gap-4">
          <div className="flex items-center gap-2">
            <label htmlFor="username" className="w-20">
              Username
            </label>
            <input
              type="text"
              name="username"
              id="username"
              value={formData.username}
              onChange={handleInputChange}
              className="input border-2 border-neutral w-72"
              required
            />
          </div>
          <div className="flex items-center gap-2">
            <div className="w-20">Password</div>
            <input
              type="password"
              name="password"
              id="password"
              value={formData.password}
              onChange={handleInputChange}
              className="input border-2 border-neutral w-72"
              required
            />
          </div>
          <div className="flex justify-end">
            <a href="">Forgot your password?</a>
          </div>
          <button type="submit" className="btn btn-primary text-base">
            Login
          </button>
        </form>
      </div>
    </div>
  );
}

export default Login;
