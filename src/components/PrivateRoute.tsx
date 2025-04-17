import { Navigate, Outlet } from "react-router-dom";
import { useAuthStore } from "../hooks/useAuthStore";
import React from "react";

const PrivateRoute: React.FC = () => {
    const { isAuthenticated, loading } = useAuthStore();

    if (loading) {
        return <div className="text-white p-4">Loading...</div>; // Or a spinner
    }

    return isAuthenticated ? <Outlet /> : <Navigate to="/" replace />;
};

export default PrivateRoute;