// src/hooks/useInjectedPort.ts
import { useEffect, useState } from "react";

export const useInjectedPort = () => {
    const [ready, setReady] = useState(false);

    useEffect(() => {
        const interval = setInterval(() => {
            if ((window as any).__API_PORT__) {
                clearInterval(interval);
                setReady(true);
            }
        }, 100);

        return () => clearInterval(interval);
    }, []);

    return ready;
};