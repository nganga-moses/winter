import React, { useEffect } from "react";
import ReactDOM from "react-dom/client";
import { SplashTerminal } from "./SplashTerminal";
import { invoke } from "@tauri-apps/api/core";

ReactDOM.createRoot(document.getElementById("root")!).render(
    <React.StrictMode>
        <SplashTerminal onReady={() => {}} />
    </React.StrictMode>
);

useEffect(() => {
    invoke("notify_splash_ready").catch(console.error);
}, []);