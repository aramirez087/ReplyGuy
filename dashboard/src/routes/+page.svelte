<script lang="ts">
    import { api } from "$lib/api";
    import { connected, runtimeRunning } from "$lib/stores/websocket";
    import { Activity, Zap, Users, CheckCircle } from "lucide-svelte";
    import { onMount } from "svelte";

    let healthStatus = $state<string>("checking...");
    let serverVersion = $state<string>("");

    onMount(async () => {
        try {
            const health = await api.health();
            healthStatus = health.status;
            serverVersion = health.version;
        } catch {
            healthStatus = "unreachable";
        }
    });
</script>

<svelte:head>
    <title>Dashboard — Tuitbot</title>
</svelte:head>

<div class="page-header">
    <h1>Dashboard</h1>
    <p class="subtitle">Your autonomous growth overview</p>
</div>

<div class="status-cards">
    <div class="card">
        <div class="card-icon">
            <Zap size={20} />
        </div>
        <div class="card-content">
            <span class="card-label">Server</span>
            <span class="card-value" class:online={healthStatus === "ok"}>
                {healthStatus}
                {#if serverVersion}
                    <span class="version">v{serverVersion}</span>
                {/if}
            </span>
        </div>
    </div>

    <div class="card">
        <div class="card-icon">
            <Activity size={20} />
        </div>
        <div class="card-content">
            <span class="card-label">WebSocket</span>
            <span class="card-value" class:online={$connected}>
                {$connected ? "Connected" : "Disconnected"}
            </span>
        </div>
    </div>

    <div class="card">
        <div class="card-icon">
            <Users size={20} />
        </div>
        <div class="card-content">
            <span class="card-label">Runtime</span>
            <span class="card-value" class:online={$runtimeRunning}>
                {$runtimeRunning ? "Running" : "Stopped"}
            </span>
        </div>
    </div>

    <div class="card">
        <div class="card-icon">
            <CheckCircle size={20} />
        </div>
        <div class="card-content">
            <span class="card-label">Approval Queue</span>
            <span class="card-value">—</span>
        </div>
    </div>
</div>

<div class="placeholder-section">
    <p>Charts and analytics will be added in Task 04.</p>
</div>

<style>
    .page-header {
        margin-bottom: 24px;
    }

    h1 {
        font-size: 24px;
        font-weight: 700;
        color: var(--color-text);
        margin: 0 0 4px;
    }

    .subtitle {
        font-size: 13px;
        color: var(--color-text-muted);
        margin: 0;
    }

    .status-cards {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
        gap: 12px;
        margin-bottom: 32px;
    }

    .card {
        display: flex;
        align-items: center;
        gap: 14px;
        padding: 16px;
        background-color: var(--color-surface);
        border: 1px solid var(--color-border-subtle);
        border-radius: 8px;
    }

    .card-icon {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 40px;
        height: 40px;
        border-radius: 8px;
        background-color: var(--color-surface-active);
        color: var(--color-accent);
        flex-shrink: 0;
    }

    .card-content {
        display: flex;
        flex-direction: column;
        gap: 2px;
    }

    .card-label {
        font-size: 11px;
        font-weight: 600;
        text-transform: uppercase;
        letter-spacing: 0.05em;
        color: var(--color-text-subtle);
    }

    .card-value {
        font-size: 14px;
        font-weight: 600;
        color: var(--color-text-muted);
    }

    .card-value.online {
        color: var(--color-success);
    }

    .version {
        font-size: 11px;
        font-weight: 400;
        color: var(--color-text-subtle);
        margin-left: 4px;
    }

    .placeholder-section {
        padding: 40px;
        text-align: center;
        color: var(--color-text-subtle);
        border: 1px dashed var(--color-border);
        border-radius: 8px;
    }
</style>
