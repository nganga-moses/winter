import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { supabase } from "../lib/client";
import { apiRequest } from "../api/api";
import { useAuthStore } from "../hooks/useAuthStore";

const OAuthCallback: React.FC = () => {
    const [error, setError] = useState<string | null>(null);
    const navigate = useNavigate();
    const { setAuth } = useAuthStore();

    useEffect(() => {
        const processOAuth = async () => {
            try {
                const hashParams = new URLSearchParams(window.location.hash.substring(1));
                const accessToken = hashParams.get("access_token");
                const refreshToken = hashParams.get("refresh_token");

                if (!accessToken || !refreshToken) {
                    throw new Error("OAuth login failed: Missing access or refresh token.");
                }

                // Set session in Supabase
                const { error: setError } = await supabase.auth.setSession({
                    access_token: accessToken,
                    refresh_token: refreshToken,
                });

                if (setError) throw new Error(setError.message);


                // Get updated session
                const { data, error: getSessionError } = await supabase.auth.getSession();
                if (getSessionError || !data.session) throw new Error("Failed to retrieve session.");

                const user = data.session.user;
                const token = data.session.access_token;
                const fullName = user.user_metadata?.full_name || "";
                const [firstName = "", lastName = ""] = fullName.split(" ");



                // Sync user with FastAPI backend
                await apiRequest(
                    `${import.meta.env.VITE_API_URL}/signup`,
                    "POST",
                    {
                        supabase_id: user.id,
                        email: user.email,
                        first_name: firstName,
                        last_name: lastName,
                    },
                    token
                );

                console.log("User ID: "+user.id);

                await setAuth(token, user.id);
                navigate("/dashboard");
            } catch (err: any) {
                setError(err.message || "OAuth login failed.");
            }
        };

        processOAuth();
    }, [navigate, setAuth]);

    return (
        <div className="h-screen flex items-center justify-center bg-black text-white">
            <p>{error ? `Error: ${error}` : "Logging you in..."}</p>
        </div>
    );
};

export default OAuthCallback;