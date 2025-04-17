import React, { useState } from "react";
import ReactDOM from "react-dom/client";
import { InstallScreen } from "./InstallScreen";
import { DownloadScreen } from "./DownloadScreen"; // you'll build this next

const App = () => {
    const [mode, setMode] = useState<"install" | "download">("install");

    return mode === "install"
        ? <InstallScreen onModeSelected={() => setMode("download")} />
        : <DownloadScreen />;
};

ReactDOM.createRoot(document.getElementById("root")!).render(<App />);