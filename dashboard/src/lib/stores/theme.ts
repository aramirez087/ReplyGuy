import { writable } from 'svelte/store';
import { browser } from '$app/environment';

export type Theme = 'dark' | 'light';

const STORAGE_KEY = 'tuitbot-theme';

function getInitialTheme(): Theme {
    if (browser) {
        const stored = localStorage.getItem(STORAGE_KEY);
        if (stored === 'light' || stored === 'dark') return stored;
    }
    return 'dark'; // Default
}

function createThemeStore() {
    const { subscribe, set, update } = writable<Theme>(getInitialTheme());

    return {
        subscribe,
        set(value: Theme) {
            if (browser) {
                localStorage.setItem(STORAGE_KEY, value);
                applyTheme(value);
            }
            set(value);
        },
        toggle() {
            update((current) => {
                const next: Theme = current === 'dark' ? 'light' : 'dark';
                if (browser) {
                    localStorage.setItem(STORAGE_KEY, next);
                    applyTheme(next);
                }
                return next;
            });
        }
    };
}

function applyTheme(theme: Theme) {
    if (!browser) return;
    const root = document.documentElement;
    root.setAttribute('data-theme', theme);
}

export const theme = createThemeStore();

/** Call once on app mount to apply the persisted theme. */
export function initTheme() {
    if (browser) {
        const current = getInitialTheme();
        applyTheme(current);
    }
}
