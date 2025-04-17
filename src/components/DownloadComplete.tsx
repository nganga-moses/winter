export function DownloadCompleteScreen({ onContinue }: { onContinue: () => void }) {
    return (
        <div className="text-center">
            <h2 className="text-green-400 text-lg font-bold mb-4">✅ Model downloaded successfully!</h2>
            <button onClick={onContinue} className="button">Continue</button>
        </div>
    );
}