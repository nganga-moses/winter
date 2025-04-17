import { useState, FormEvent } from "react";
import { FaGithub } from "react-icons/fa";
import { FcGoogle } from "react-icons/fc";
import { HiEye, HiEyeOff } from "react-icons/hi";
import { Link, useNavigate } from "react-router-dom";
import { supabase } from "../lib/client";
import { apiRequest } from "../api/api";
import { useAuthStore } from "../hooks/useAuthStore";
import TopBar from "../components/TopBar.tsx";

const SignUpPage: React.FC = () => {
    const [email, setEmail] = useState<string>("");
    const [password, setPassword] = useState<string>("");
    const [showPassword, setShowPassword] = useState<boolean>(false);
    const [error, setError] = useState<string | null>(null);
    const navigate = useNavigate();
    const { setAuth } = useAuthStore();

    const handleSignUp = async (e: FormEvent) => {
        e.preventDefault();
        setError(null);

        try {
            const { data, error } = await supabase.auth.signUp({ email, password });
            if (error) throw new Error(error.message);

            if (data.user) {
                const token = data.session?.access_token || null;
                const supabaseId = data.user.id;

                await apiRequest(
                    `${import.meta.env.VITE_API_URL}/signup`,
                    "POST",
                    {
                        supabase_id: supabaseId,
                        email: data.user.email,
                        first_name: data.user.user_metadata?.full_name?.split(" ")[0] || "",
                        last_name: data.user.user_metadata?.full_name?.split(" ")[1] || "",
                    },
                    token
                );
                console.log("token"+token)
                if (token) {
                    await setAuth(token, supabaseId);
                }

                navigate("/dashboard");
            }
        } catch (err: any) {
            setError(err.message || "Sign up failed. Please try again.");
        }
    };

    const handleOAuthLogin = async (provider: "github" | "google") => {
        setError(null);

        try {
            const { error } = await supabase.auth.signInWithOAuth({
                provider,
                options: { redirectTo: window.location.origin + "/dashboard" },
            });

            if (error) throw new Error(error.message);

            // Supabase will redirect on success
        } catch (err: any) {
            setError(err.message || "OAuth sign-up failed.");
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

                {error && <p className="text-red-500 text-sm text-center">{error}</p>}

                <p className="text-gray-400 text-center mb-4">Create a new account</p>

                <button
                    onClick={() => handleOAuthLogin("github")}
                    className="flex items-center justify-center w-full p-3 border border-gray-600 bg-gray-800 hover:bg-gray-700 rounded-md"
                >
                    <FaGithub className="mr-2" /> Continue with GitHub
                </button>

                <button
                    onClick={() => handleOAuthLogin("google")}
                    className="flex items-center justify-center w-full p-3 border border-gray-600 bg-gray-800 hover:bg-gray-700 rounded-md mt-2"
                >
                    <FcGoogle className="mr-2" /> Continue with Google
                </button>

                <div className="flex items-center space-x-2 my-4">
                    <div className="flex-1 h-px bg-gray-600"></div>
                    <span className="text-gray-500 text-sm">OR</span>
                    <div className="flex-1 h-px bg-gray-600"></div>
                </div>

                <form onSubmit={handleSignUp} className="space-y-4">
                    <div>
                        <label className="block text-sm">
                            Email <span className="text-red-500">*</span>
                        </label>
                        <input
                            type="email"
                            placeholder="you@example.com"
                            value={email}
                            onChange={(e) => setEmail(e.target.value)}
                            required
                            className="w-full p-3 bg-gray-800 border border-gray-700 rounded-md text-white"
                        />
                    </div>

                    <div>
                        <label className="block text-sm">
                            Password <span className="text-red-500">*</span>
                        </label>
                        <div className="relative">
                            <input
                                type={showPassword ? "text" : "password"}
                                placeholder="Enter password"
                                value={password}
                                onChange={(e) => setPassword(e.target.value)}
                                required
                                className="w-full p-3 bg-gray-800 border border-gray-700 rounded-md text-white pr-10"
                            />
                            <button
                                type="button"
                                onClick={() => setShowPassword(!showPassword)}
                                className="absolute inset-y-0 right-3 flex items-center text-gray-400 hover:text-white"
                            >
                                {showPassword ? <HiEyeOff size={18} /> : <HiEye size={18} />}
                            </button>
                        </div>
                    </div>

                    <button
                        type="submit"
                        className="w-full bg-green-600 hover:bg-green-700 p-3 rounded-md text-white font-semibold"
                    >
                        Sign Up
                    </button>
                </form>

                <p className="text-gray-400 text-sm text-center mt-4">
                    Have an account?{" "}
                    <Link to="/login" className="text-blue-500 hover:underline">
                        Sign In Now
                    </Link>
                </p>
            </div>
        </div>
        </div>
    );
};

export default SignUpPage;