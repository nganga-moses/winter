export function ModeSelectionScreen({ onNext }: { onNext: () => void }) {
    return (
        <div>
            <p>How do you want to run Winter:</p>
            <button onClick={onNext} className="button">🖥 Local Mode</button>
            <button className="button">☁️ Cloud Mode</button>
        </div>
    );
}