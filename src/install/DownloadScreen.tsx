import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export function DownloadScreen() {
    const [progress, setProgress] = useState(0);
    const [status, setStatus] = useState("Downloading model...");

    useEffect(() => {
        listen("model-download-progress", (event) => {
            const { downloaded, total, percent } = event.payload;
            const format = (b: number) => (b / (1024 * 1024 * 1024)).toFixed(2) + " GB";

            setProgress(`${format(downloaded)} / ${format(total)} (${percent}%)`);
        });

        invoke("download_model_file")
            .then(() => setStatus("✅ Model installed!"))
            .catch((err) => setStatus(`❌ Failed: ${err}`));
    }, []);

    return (
        <div>
            <h2>{status}</h2>
            <div style={{ border: "1px solid #333", padding: "5px" }}>
                <div style={{ width: `${progress}%`, background: "limegreen", height: "1rem" }} />
            </div>
            <p>{progress}%</p>
        </div>
    );
}