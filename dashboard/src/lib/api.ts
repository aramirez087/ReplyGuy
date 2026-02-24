const BASE_URL = 'http://localhost:3001';
let token: string = '';

export function setToken(t: string) {
    token = t;
}

export function getToken(): string {
    return token;
}

async function request<T>(path: string, options?: RequestInit): Promise<T> {
    const res = await fetch(`${BASE_URL}${path}`, {
        ...options,
        headers: {
            'Content-Type': 'application/json',
            Authorization: `Bearer ${token}`,
            ...options?.headers
        }
    });
    if (!res.ok) {
        const body = await res.json().catch(() => ({ error: res.statusText }));
        throw new Error(body.error || res.statusText);
    }
    return res.json();
}

export interface HealthResponse {
    status: string;
    version: string;
}

export const api = {
    health: () => request<HealthResponse>('/api/health')
    // Expand in later tasks (04-09)
};
