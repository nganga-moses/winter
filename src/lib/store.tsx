// src/lib/store.tsx
import { load, Store } from '@tauri-apps/plugin-store';

const isTauri = '__TAURI_INTERNALS__' in window || '__TAURI_METADATA__' in window;

let store: Store | null = null;

export const initStore = async (): Promise<Store | null> => {
    if (!isTauri) return null; // Tauri-only guard

    if (!store) {
        store = await load('store.json', { autoSave: true });
    }

    return store;
};

export const setAuthData = async (token: string, supabaseId: string) => {
    const s = await initStore();
    if (!s) return;

    await s.set('auth.token', token);
    await s.set('auth.supabase_id', supabaseId);
    await s.save();
};

export const getToken = async (): Promise<string | null> => {
    const s = await initStore();
    if (!s) return null;

    return (await s.get<string>('auth.token')) ?? null;
};

export const getSupabaseId = async (): Promise<string | null> => {
    const s = await initStore();
    if (!s) return null;

    return (await s.get<string>('auth.supabase_id')) ?? null;
};

export const clearAuthData = async () => {
    const s = await initStore();
    if (!s) return;

    await s.delete('auth.token');
    await s.delete('auth.supabase_id');
    await s.save();
};