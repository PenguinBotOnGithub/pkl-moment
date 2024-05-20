// src/components/ProtectedRoute.jsx
import React from "react";
import { Navigate } from "react-router-dom";
import { useAuth } from "../context/AuthContext"; // Pastikan path sudah benar

export const ProtectedRoute = ({ children }) => {
  const { user } = useAuth();

  if (user === undefined) {
    return <div>Loading...</div>; // Anda dapat menambahkan indikator loading atau placeholder
  }

  return user ? children : <Navigate to="/login" />;
};
