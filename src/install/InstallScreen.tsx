import {useEffect, useState} from "react";
import { ModeSelectionScreen } from "./ModeSelectionScreen";
import { ModelConfirmScreen } from "./ModelConfirmScreen";
import {DownloadProgressDisplay} from "./DownloadProgressDisplay.tsx";
import {invoke} from "@tauri-apps/api/core";
import {DownloadCompleteScreen} from "../components/DownloadComplete.tsx";
import {getCurrentWindow} from "@tauri-apps/api/window";



export function InstallScreen({ onModeSelected }: { onModeSelected: () => void }) {
    const [step, setStep] = useState<"mode" | "confirm" | "download" | "done">("mode");


    return (
        <div className="has-glow glow-box" id="install-panel">
            <div id="">
                {step === "mode" && <ModeSelectionScreen onNext={() => setStep("confirm")}/>}
                {step === "confirm" && (
                    <ModelConfirmScreen
                        onBack={() => setStep("mode")}
                        onDownload={() => {
                            invoke("download_model_file").catch(console.error); // fire-and-forget
                            setStep("download"); // immediately show progress UI
                        }}
                    />
                )}
                {step === "download" && <DownloadProgressDisplay/>}
                {step === "done" && (
                    <DownloadCompleteScreen
                        onContinue={async () => {
                            const win = await getCurrentWindow();
                            await win.close();
                        }}
                    />
                )}
            </div>
        </div>
    );
}
