import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";

export function ModelConfirmScreen({
                                       onBack,
                                       onDownload
                                   }: {
    onBack: () => void;
    onDownload: () => void;
}) {
    const [info, setInfo] = useState<any>(null);
    const [diskFree, setDiskFree] = useState<number | null>(null);

    useEffect(() => {
        invoke("get_model_download_info").then(setInfo);
        invoke("get_free_disk_space").then(setDiskFree);
    }, []);

    function formatBytes(bytes: number) {
        const sizes = ["B", "KB", "MB", "GB", "TB"];
        const i = Math.floor(Math.log(bytes) / Math.log(1024));
        return (bytes / Math.pow(1024, i)).toFixed(2) + " " + sizes[i];
    }

    if (!info) return <p>Loading model info…</p>;

    return (
        <div>
            <h2>Local Agent Details</h2>
            <ul>
                <li><strong>Model:</strong> {info.model_name}</li>
                <li><strong>Estimated Size:</strong> {formatBytes(info.estimated_size_bytes)}</li>
                {diskFree && (
                    <li><strong>Available Disk:</strong> {formatBytes(diskFree)}</li>
                )}
            </ul>
            {diskFree && diskFree < info.estimated_size_bytes && (
                <p style={{color: "red"}}>⚠️ Not enough space. Switch to Cloud mode instead.</p>
            )}
            <button onClick={onBack} className="button">← Back</button>
            <button onClick={onDownload} className="button">⬇️ Download Model</button>
        </div>
    );
}