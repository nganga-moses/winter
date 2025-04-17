import {BrowserRouter as Router, Route, Routes} from "react-router-dom";
import ShellLayout from "./layouts/ShellLayout.tsx";
import CodeEditor from "./pages/CodeEditor.tsx";
import Dashboard from "./pages/Dashboard.tsx";
import Assistant from "./pages/Assistant.tsx";
import LoginPage from "./pages/LoginPage.tsx";
import PrivateRoute from "./components/PrivateRoute.tsx";
import SignUpPage from "./pages/SignUp.tsx";
import OAuthCallback from "./pages/OauthCallback.tsx";
import RedirectHome from "./pages/RedirectHome.tsx";
import {listen} from "@tauri-apps/api/event";
import {useEffect} from "react";
import {InstallScreen} from "./pages/Install.tsx";

function App() {
    useEffect(() => {
        listen<number>("backend-ready", (event) => {
            const port = event.payload;
            (window as any).__API_PORT__ = port;
            (window as any).__API_URL__ = `http://127.0.0.1:${port}`;
            console.log("✅ Backend ready on port:", port);
        });
    }, []);
    return (
        <Router>
            <Routes>
                <Route path="/signup" element={<SignUpPage/>}/>
                <Route path="/" element={<RedirectHome/>}/>
                <Route path="/login" element={<LoginPage/>}/>
                <Route path="/oauth-callback" element={<OAuthCallback/>}/>
                <Route element={<PrivateRoute/>}>
                    <Route element={<ShellLayout/>}>
                        <Route path="/dashboard" element={<Dashboard/>}/>
                        <Route path="/assistant" element={<Assistant/>}/>
                        <Route path="/projects/:id/editor" element={<CodeEditor/>}/>

                    </Route>
                </Route>
            </Routes>

        </Router>
    );
}

export default App;