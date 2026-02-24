<script lang="ts">
	import "../app.css";
	import Sidebar from "$lib/components/Sidebar.svelte";
	import { setToken } from "$lib/api";
	import { connectWs } from "$lib/stores/websocket";
	import { initTheme } from "$lib/stores/theme";
	import { onMount } from "svelte";

	let { children } = $props();

	onMount(async () => {
		initTheme();
		// In Tauri context, get token from the sidecar via invoke.
		// In browser dev mode, try a dev token or skip auth.
		try {
			const { invoke } = await import("@tauri-apps/api/core");
			const token: string = await invoke("get_api_token");
			setToken(token);
			connectWs(token);
		} catch {
			// Not running in Tauri (browser dev mode) — try reading token from env or skip.
			// For development, you can manually set the token here.
			console.warn(
				"Not in Tauri context. API auth and WebSocket disabled in dev mode.",
			);
		}
	});
</script>

<div class="app-shell">
	<Sidebar />
	<main class="main-content">
		{@render children()}
	</main>
</div>

<style>
	.app-shell {
		display: flex;
		min-height: 100vh;
		background-color: var(--color-base);
	}

	.main-content {
		flex: 1;
		padding: 24px 32px;
		overflow-y: auto;
	}
</style>
