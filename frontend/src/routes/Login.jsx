import React, { useState } from "react";
import Navbar from "../components/Navbar";
import PKLMomentIcon from "../assets/drawable/PKLMomentIcon";
import { useNavigate } from "react-router-dom";

function Login() {
  const navigate = useNavigate();
  const [formData, setFormData] = useState({
    username: '',
    password: '',
  });

  const handleInputChange = (e) => {
    const { name, value } = e.target;
    setFormData({
      ...formData,
      [name]: value,
    });
  };

  const handleSubmit = (e) => {
    e.preventDefault();

    // Add your login logic here, such as sending a request to your server
    navigate("/dashboard");

    console.log('Form data submitted:', formData);
    // You can use Axios or the Fetch API to send the data to your server for authentication
  };
  return (
    <div className="flex flex-col size-full">
      <Navbar title="Login" />
      <div className="flex-1 flex flex-col gap-2 justify-center items-center bg-base-200 p-4">
        <div className="flex items-center mb-6">
          <PKLMomentIcon size={64} />
          <span className="text-4xl text-primary">PKL</span>
          <span className="text-4xl text-secondary">Moment</span>
        </div>
        {/* The form */}
        <form onSubmit={handleSubmit} className="flex flex-col gap-4">
          <div className="flex items-center gap-2">
            <label htmlFor="username" className="w-40">Username</label>
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
          <button type="submit" className="btn btn-secondary text-base">Login</button>
        </form>
      </div>
    </div>
  );
}

export default Login;
