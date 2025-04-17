import { useEffect } from 'react';
import { useNavigate } from "react-router-dom";
import { useAuthStore } from "../hooks/useAuthStore";
import { supabase } from "../lib/client.tsx";

const RedirectHome = () => {
    const navigate = useNavigate();
    const { token, supabaseId, loading, setAuth } = useAuthStore();

    useEffect(() => {
        const checkAuth = async () => {
            if (loading) return; // Wait for initial load

            if (token && supabaseId) {
                navigate("/dashboard", { replace: true });
                return;
            }

            // Fallback to Supabase session
            const { data } = await supabase.auth.getSession();
            const session = data?.session;

            if (session?.access_token && session.user?.id) {
                await setAuth(session.access_token, session.user?.id, true
                );
                navigate("/dashboard", { replace: true });
            } else {
                navigate("/login", { replace: true });
            }
        };

        checkAuth();
    }, [token,supabaseId,loading,setAuth, navigate]);

    return null;
};

export default RedirectHome;