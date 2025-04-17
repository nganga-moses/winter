import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";

type ProgressPayload = {
    downloaded: number;
    total: number;
    percent: number;
    speed_bytes_per_sec: number;
    eta_seconds: number;
};

export function DownloadProgressDisplay() {
    const [progress, setProgress] = useState<ProgressPayload | null>(null);
    const [done, setDone] = useState(false);

    useEffect(() => {
        const unlisten = listen<ProgressPayload>("model-download-progress", (event) => {
            const p = event.payload;
            setProgress(p);
            if (p.percent >= 100) {
                setDone(true);
            }
        });

        return () => {
            unlisten.then((f) => f());
        };
    }, []);

    const formatBytes = (bytes: number) => {
        const sizes = ["B", "KB", "MB", "GB", "TB"];
        const i = Math.floor(Math.log(bytes) / Math.log(1024));
        return (bytes / Math.pow(1024, i)).toFixed(2) + " " + sizes[i];
    };

    const formatETA = (secs: number) => {
        const m = Math.floor(secs / 60);
        const s = secs % 60;
        return `${m}m ${s}s`;
    };

    if (!progress) {
        return <div style={{ color: "#66ff66", fontFamily: "monospace" }}>Initializing download…</div>;
    }

    return (
        <div style={{
            color: "#afafaf",
            fontFamily: "monospace",
            maxWidth: "500px",
            margin: "2rem auto",
            minHeight: "200px", // ✅ Lock height to avoid resize flicker
        }}>
            {!done && (
            <h2 style={{ marginBottom: 12 }}>⌛ Downloading model...</h2>
                )}
            {done && (
                <h2 style={{ marginBottom: 12 }}> Model Downloaded</h2>
            )}

            <div style={{ marginBottom: 6 }}>
                <strong>Progress:</strong> {progress.percent.toFixed(1)}%
            </div>
            <div style={{ marginBottom: 6 }}>
                <strong>Downloaded:</strong> {formatBytes(progress.downloaded)} / {formatBytes(progress.total)}
            </div>
            <div style={{ marginBottom: 6 }}>
                <strong>Speed:</strong> {formatBytes(progress.speed_bytes_per_sec)}/s
            </div>
            <div style={{ marginBottom: 16 }}>
                <strong>ETA:</strong> {formatETA(progress.eta_seconds)}
            </div>

            <div style={{
                height: 12,
                background: "#333",
                borderRadius: 6,
                overflow: "hidden",
                marginBottom: 24,
                width: "100%"
            }}>
                <div style={{
                    width: `${progress.percent}%`,
                    height: "100%",
                    background: "linear-gradient(90deg, #fbbf24, #8b5cf6)",
                    transition: "width 0.2s ease-in-out" // ✅ smooth bar updates
                }} />
            </div>

            {done && (
                <div>
                    <p style={{ color: "#aaffaa", fontWeight: "bold", marginBottom: 12 }}>
                        ✅ Model downloaded successfully!
                    </p>
                    <button
                        onClick={() => getCurrentWindow().close()}
                        style={{
                            background: "#222",
                            color: "white",
                            padding: "0.6rem 1.2rem",
                            border: "1px solid #555",
                            borderRadius: 6,
                            cursor: "pointer"
                        }}
                    >
                        Continue →
                    </button>
                </div>
            )}
        </div>
    );
}