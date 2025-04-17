import React from "react";
import ReactDOM from "react-dom/client";
import './index.css';
import App from "./App";
import {listen} from "@tauri-apps/api/event";

async function bootstrapApp() {
    await new Promise<number>((resolve) => {
        listen<number>("port-ready", event => {
            console.log("✅ Port received from Tauri backend:", event.payload);
            (window as any).__API_PORT__ = event.payload;
            resolve(event.payload);
        });
    });

    ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
        <React.StrictMode>
            <App/>
        </React.StrictMode>,
    );
}

bootstrapApp();

