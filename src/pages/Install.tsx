import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";

type ModelInfo = {
    model_name: string;
    quant: string;
    url: string;
    expected_sha256: string;
    estimated_size_bytes: number;
};

export function InstallScreen() {
    const [mode, setMode] = useState<"local" | "cloud" | null>(null);
    const [info, setInfo] = useState<ModelInfo | null>(null);
    const [freeSpaceBytes, setFreeSpaceBytes] = useState<number | null>(null);

    useEffect(() => {
        invoke<ModelInfo>("get_model_download_info")
            .then(setInfo)
            .catch(console.error);
        invoke<number>("get_free_disk_space").then(setFreeSpaceBytes).catch(console.error);
    }, []);

    const handleModeSelect = async (selected: "local" | "cloud") => {
        setMode(selected);
        await invoke("set_current_mode", {mode: selected});
        if (selected === "local") {
            await invoke("install_model");
        }
    };

    function formatBytes(bytes: number, decimals = 2): string {
        if (bytes === 0) return "0 Bytes";
        const k = 1024;
        const dm = decimals;
        const sizes = ["Bytes", "KB", "MB", "GB", "TB"];

        const i = Math.floor(Math.log(bytes) / Math.log(k));
        const value = parseFloat((bytes / Math.pow(k, i)).toFixed(dm));
        return `${value} ${sizes[i]}`;
    }

    return (
        <div className=" capra p-6 font-sans text-white bg-gray-900 min-h-screen flex flex-col items-center justify-center">
            <div className="has-glow glow-box border border-zinc-700 rounded-xl shadow-xl p-6 max-w-md w-full">
                <p className="mb-4">How do you want to use Winter:</p>

                <div className="mode-buttons">
                    <button
                        onClick={() => handleModeSelect("local")}
                        className="btn bg-green-700 hover:bg-green-800 text-white py-2 px-4 rounded"
                    >
                        🖥 Local Mode
                    </button>
                    <button
                        onClick={() => handleModeSelect("cloud")}
                        className="btn bg-blue-600 hover:bg-blue-700 text-white py-2 px-4 rounded"
                    >
                        ☁️ Cloud Mode
                    </button>
                </div><br/>
                <div className="mt-3 border border-green-500 bg-black/60 rounded p-4 mb-6 w-full max-w-md shadow">
                    <p><strong>Local Mode Disk Requirements:</strong></p>
                    {info && (
                        <span><strong>Required:</strong> {formatBytes(info.estimated_size_bytes)} </span>
                    )}

                    {info && freeSpaceBytes !== null && (
                        <div>
                            <span><strong>Available:</strong> {formatBytes(freeSpaceBytes)}</span>

                            {freeSpaceBytes < info.estimated_size_bytes && (
                                <p className="text-red-400 font-bold mt-2">
                                    ⚠️ Not enough disk space! You need at least {formatBytes(info.estimated_size_bytes)}.
                                </p>
                            )}
                        </div>
                    )}
                </div>
            </div>
        </div>
    );
}