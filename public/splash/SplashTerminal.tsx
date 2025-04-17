import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";

export function SplashTerminal({ onReady }: { onReady: () => void }) {
    const [logs, setLogs] = useState<string[]>([]);
    const [done, setDone] = useState(false);

    useEffect(() => {
        const unlisten = listen<string>("sidecar-log", (event) => {
            const line = event.payload.trim();
            setLogs((prev) => [...prev, line]);

            if (line.toLowerCase().includes("ready") || line.includes("✓")) {
                setDone(true);
                setTimeout(async () => {
                    await getCurrentWindow().close(); // ✅ closes splash window from frontend
                }, 500);
            }
        });

        return () => {
            unlisten.then((f) => f());
        };
    }, []);



    return (
        <div className="min-h-screen bg-black text-green-400 p-6 font-mono text-sm flex items-center justify-center">
            <div className="w-full max-w-2xl bg-black/90 p-4 rounded border border-green-600 shadow-xl animate-fade-in">
                {logs.map((line, i) => (
                    <div key={i} className="whitespace-pre-wrap animate-fade-in">{line}</div>
                ))}
                {!done && <div className="mt-4 text-green-300 animate-pulse">⌛ Booting system...</div>}
            </div>
        </div>
    );
}