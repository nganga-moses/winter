import { getLocalApiUrl } from "./config";

export const apiRequest = async <T = any>(
    url: string,
    method: "GET" | "POST" | "PUT" | "DELETE",
    payload: Record<string, any> | FormData | null = null,
    token: string | null = null
): Promise<T> => {
    try {
        const headers: Record<string, string> = {};
        const isFormData = payload instanceof FormData;

        if (token) {
            headers["Authorization"] = `Bearer ${token}`;
        }

        if (!isFormData) {
            headers["Content-Type"] = "application/json";
        }

        const response = await fetch(url, {
            method,
            headers,
            body: isFormData
                ? (payload as FormData)
                : payload
                    ? JSON.stringify(payload)
                    : null,
        });

        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        return await response.json();
    } catch (error) {
        console.error("API request failed:", error);
        throw error;
    }
};

export const internalRequest = async <T = any>(
    url: string,
    method: "GET" | "POST" | "PUT" | "DELETE",
    payload: Record<string, any> | FormData | null = null,
    token: string | null = null
): Promise<T> => {
    try {
        const headers: Record<string, string> = {};
        const isFormData = payload instanceof FormData;

        if (token) {
            headers["Authorization"] = `Bearer ${token}`;
        }

        if (!isFormData) {
            headers["Content-Type"] = "application/json";
        }

        const fullUrl = `${getLocalApiUrl()}/${url}`;

        const response = await fetch(fullUrl, {
            method,
            headers,
            body: isFormData
                ? (payload as FormData)
                : payload
                    ? JSON.stringify(payload)
                    : null,
        });

        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        return await response.json();
    } catch (error) {
        console.error("API request failed:", error);
        throw error;
    }
};