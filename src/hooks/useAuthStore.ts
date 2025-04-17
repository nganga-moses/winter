// src/hooks/useAuthStore.ts
import { useEffect, useState } from 'react';
import { getToken, getSupabaseId, setAuthData, clearAuthData } from '../lib/store';

interface AuthState {
    token: string | null;
    supabaseId: string | null;
    isAuthenticated: boolean;
    loading: boolean;
    persistAcrossSessions:boolean,
    setAuth: (
        token: string,
        supabaseId: string,
        persist?:boolean
    ) => Promise<void>;
    clearAuth: () => Promise<void>;
}

export const useAuthStore = (): AuthState => {
    const [token, setToken] = useState<string | null>(null);
    const [supabaseId, setSupabaseId] = useState<string | null>(null);
    const [loading, setLoading] = useState(true); // ← add loading flag
    const [persistAcrossSessions, setPersistAcrossSessions] = useState(false);

    const load = async () => {
        const t = await getToken();
        const id = await getSupabaseId();
        if (t && id){
            setToken(t);
            setSupabaseId(id);
        }
        setLoading(false); // ← set false after fetching
    };

    const setAuth = async (
        token: string,
        supabaseId: string,
        persist = false
    ) => {

        setToken(token);
        setSupabaseId(supabaseId);
        setPersistAcrossSessions(persist);
        if (persist){
            await setAuthData(token, supabaseId);
        }
    };

    const clearAuth = async () => {
        setToken(null);
        setSupabaseId(null);
        setPersistAcrossSessions(false)
        await clearAuthData();
    };

    useEffect(() => {
        load();
    }, []);

    return {
        token,
        supabaseId,
        isAuthenticated: !!token && !!supabaseId,
        loading,
        persistAcrossSessions,
        setAuth,
        clearAuth
    };
};