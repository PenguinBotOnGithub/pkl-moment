// src/routes/Secret.jsx
import React from "react";
import { useAuth } from "../context/AuthContext";

export const Secret = () => {
  const { logout } = useAuth();

  const handleLogout = () => {
    logout();
  };

  return (
    <div>
      <h1>This is a Secret page</h1>
      <button onClick={handleLogout}>Logout</button>
    </div>
  );
};
