// src/api/config.ts
export const getLocalApiUrl = () => {
    const port = (window as any).__API_PORT__ || import.meta.env.VITE_API_PORT || 6144;
    return `http://127.0.0.1:${port}`;
};