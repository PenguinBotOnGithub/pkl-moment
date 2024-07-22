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
        // cookies.set("role", "advisor_school");
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
    <div className="flex flex-col size-full">
      <Navbar title="Login" />
      <div className="flex-1 flex flex-col gap-2 justify-center items-center bg-base-200 p-4 pb-14">
        <div className="flex items-center mb-6">
          <PKLMomentIcon size={64} />
          <span className="text-4xl text-primary">PKL</span>
          <span className="text-4xl text-secondary">Moment</span>
        </div>
        {/* The form */}
        <form onSubmit={handleSubmit} className="flex flex-col gap-4">
          <div className="flex items-center gap-2">
            <label htmlFor="username" className="w-40">
              Username
            </label>
            <input
              type="text"
              name="username"
              id="username"
              value={formData.username}
              onChange={handleInputChange}
              className="input border-2 border-neutral w-full max-w-xs"
              required
            />
          </div>
          <div className="flex items-center gap-2">
            <div className="w-40">Password</div>
            <input
              type="password"
              name="password"
              id="password"
              value={formData.password}
              onChange={handleInputChange}
              className="input border-2 border-neutral w-full max-w-xs"
              required
            />
          </div>
          <div className="flex justify-end">
            <a href="">Forgot your password?</a>
          </div>
          <button type="submit" className="btn btn-secondary text-base">
            Login
          </button>
        </form>
      </div>
    </div>
  );
}

export default Login;
