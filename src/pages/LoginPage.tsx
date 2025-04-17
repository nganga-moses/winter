import { useState, FormEvent } from "react";
import { FaGithub } from "react-icons/fa";
import { FcGoogle } from "react-icons/fc";
import { Link, useNavigate } from "react-router-dom";
import { supabase } from "../lib/client";
import { apiRequest } from "../api/api";
import { useAuthStore } from "../hooks/useAuthStore";
import TopBar from "../components/TopBar.tsx";

const LoginPage: React.FC = () => {
    const [email, setEmail] = useState<string>("");
    const [password, setPassword] = useState<string>("");
    const [error, setError] = useState<string | null>(null);
    const navigate = useNavigate();
    const { setAuth } = useAuthStore();

    const handleLogin = async (e: FormEvent) => {
        e.preventDefault();
        setError(null);

        try {
            const { data, error } = await supabase.auth.signInWithPassword({ email, password });
            if (error) throw new Error(error.message);

            const token = data.session?.access_token;
            const supabaseId = data.user?.id;

            if (!token || !supabaseId) {
                throw new Error("Missing authentication data.");
            }

            // Fetch user details from your FastAPI backend
            const userData = await apiRequest(`${import.meta.env.VITE_API_URL}/users/me`, "GET", null, token);

            // Store in Tauri Store or fallback (useAuthStore handles that)
            await setAuth(token, supabaseId,true);

            // Also store fallback copy in memory if needed
            localStorage.setItem("user", JSON.stringify(userData));

            navigate("/dashboard");
        } catch (err: any) {
            setError(err.message || "Login failed. Please try again.");
        }
    };

    const handleOAuthLogin = async (provider: "github" | "google") => {
        setError(null);
        try {
            const { error } = await supabase.auth.signInWithOAuth({
                provider,
                options: { redirectTo: window.location.origin + "/oauth-callback" },
            });
            if (error) throw new Error(error.message);
        } catch (err: any) {
            setError(err.message || "OAuth login failed.");
        }
    };

    return (
        <div>
            <TopBar/>
        <div className="h-screen flex items-center justify-center bg-blue-gray text-white">

            <div className="w-full max-w-md bg-gray-900 p-8 rounded-lg shadow-lg">
                <div className="flex justify-center mb-4">
                    <img className="h-20 w-20" src="/logo.png" alt="Co-Founder" />
                </div>

                {error && <p className="text-red-500 text-sm text-center mb-2">{error}</p>}

                <p className="text-white text-center mb-4">Log in to your account</p>

                <button
                    onClick={() => handleOAuthLogin("github")}
                    className="flex items-center justify-center w-full p-3  bg-gray-700 hover:bg-orange-400 rounded-md"
                >
                    <FaGithub className="mr-2" /> Continue with GitHub
                </button>

                <button
                    onClick={() => handleOAuthLogin("google")}
                    className="flex items-center justify-center w-full p-3 bg-gray-700 hover:bg-orange-400 rounded-md mt-2"
                >
                    <FcGoogle className="mr-2" /> Continue with Google
                </button>

                <div className="flex items-center space-x-2 my-4">
                    <div className="flex-1 h-px bg-gray-600"></div>
                    <span className="text-gray-500 text-sm">OR</span>
                    <div className="flex-1 h-px bg-gray-600"></div>
                </div>

                <form onSubmit={handleLogin} className="space-y-4">
                    <div>
                        <label className="block text-sm">Email <span className="text-red-500">*</span></label>
                        <input
                            type="email"
                            placeholder="Enter email"
                            value={email}
                            onChange={(e) => setEmail(e.target.value)}
                            required
                            className="w-full p-3 bg-gray-800 border border-gray-700 rounded-md text-white"
                        />
                    </div>

                    <div>
                        <label className="block text-sm">Password <span className="text-red-500">*</span></label>
                        <input
                            type="password"
                            placeholder="Enter password"
                            value={password}
                            onChange={(e) => setPassword(e.target.value)}
                            required
                            className="w-full p-3 bg-gray-800 border border-gray-700 rounded-md text-white"
                        />
                    </div>

                    <button
                        type="submit"
                        className="w-full bg-green-600 hover:bg-green-700 p-3 rounded-md text-white font-semibold"
                    >
                        Sign In
                    </button>
                </form>

                <p className="text-gray-400 text-sm text-center mt-4">
                    Don’t have an account?{" "}
                    <Link to="/signup" className="text-white hover:underline">Sign Up Now</Link>
                </p>
            </div>
        </div>
        </div>
    );
};

export default LoginPage;