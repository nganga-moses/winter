import { useEffect, useState } from "react";
import { emit, listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";

export function SplashTerminal({ onReady }: { onReady: () => void }) {
    const [line, setLine] = useState("Initiating booting sequence...");
    const [done, setDone] = useState(false);
    const [progress, setProgress] = useState<number | null>(null);
    const [fileSizeGB, setFileSizeGB] = useState<number | null>(null);
    const [showCancel, setShowCancel] = useState(false);

    useEffect(() => {
        invoke("get_model_download_info").then((info: any) => {
            if (info.estimated_size_bytes) {
                setFileSizeGB(info.estimated_size_bytes / (1024 ** 3));
            }
        }).catch(() => {
            setFileSizeGB(null);
        });
    }, []);

    useEffect(() => {
        const unlistenLog = listen<string>("sidecar-log", (event) => {
            const msg = event.payload.trim();
            setLine(msg);

            if (msg.toLowerCase().includes("downloading")) {
                setShowCancel(true);
            }

            if (msg.toLowerCase().includes("ready") || msg.includes("✓")) {
                setDone(true);
                setShowCancel(false);
                setTimeout(async () => {
                    await getCurrentWindow().close();
                }, 500);
            }
        });

        const unlistenProgress = listen<number>("model-download-progress", (event) => {
            setProgress(event.payload);
        });

        return () => {
            unlistenLog.then(f => f());
            unlistenProgress.then(f => f());
        };
    }, []);

    useEffect(() => {
        if (!(window as any).__frontend_ready_emitted) {
            emit("frontend-ready");
            (window as any).__frontend_ready_emitted = true;
        }
    }, []);

    const cancelDownload = async () => {
        await invoke("cancel_model_install");
        setLine("❌ Download cancelled.");
        setShowCancel(false);
    };

    return (
        <div className="min-h-screen bg-black text-green-400 p-6 font-mono text-sm flex items-center justify-center">
            <div className="w-full max-w-2xl bg-black/90 p-4 rounded border border-green-600 shadow-xl animate-fade-in space-y-3">
                {!done && <div className="text-green-300 animate-pulse">⌛ Initializing Agent...</div>}

                <div className="whitespace-pre-wrap">{line}</div>

                {progress !== null && (
                    <div>
                        📦 Download Progress: {progress}%{" "}
                        {fileSizeGB && <span> (~{fileSizeGB.toFixed(2)} GB)</span>}
                    </div>
                )}

                {showCancel && (
                    <button
                        onClick={cancelDownload}
                        className="mt-2 px-4 py-1 border border-red-400 text-red-300 rounded hover:bg-red-800 hover:text-white transition"
                    >
                        Cancel Download
                    </button>
                )}
            </div>
        </div>
    );
}