<script lang="ts">
	import { onMount } from 'svelte';
	import { onboardingData } from '$lib/stores/onboarding';
	import { FolderOpen } from 'lucide-svelte';

	let vaultPath = $state($onboardingData.vault_path);
	let vaultWatch = $state($onboardingData.vault_watch);
	let vaultLoopBack = $state($onboardingData.vault_loop_back);
	let isTauri = $state(false);
	let browseError = $state('');

	onMount(async () => {
		try {
			await import('@tauri-apps/plugin-dialog');
			isTauri = true;
		} catch {
			// Not in Tauri context
		}
	});

	$effect(() => {
		onboardingData.updateField('vault_path', vaultPath);
	});

	$effect(() => {
		onboardingData.updateField('vault_watch', vaultWatch);
	});

	$effect(() => {
		onboardingData.updateField('vault_loop_back', vaultLoopBack);
	});

	async function browseFolder() {
		browseError = '';
		try {
			const { open } = await import('@tauri-apps/plugin-dialog');
			const selected = await open({
				directory: true,
				title: 'Select content source folder'
			});
			if (selected) {
				vaultPath = selected as string;
			}
		} catch {
			browseError = 'Could not open folder picker';
		}
	}
</script>

<div class="step">
	<h2>Content Source (Optional)</h2>
	<p class="step-description">
		Connect an Obsidian vault or notes folder so the Watchtower can index your content
		and use it for smarter replies and tweets. You can configure this later in Settings.
	</p>

	<div class="field-group">
		<label class="field-label" for="vault_path">
			<FolderOpen size={14} />
			Vault / Notes Folder
		</label>
		<div class="path-row">
			<input
				id="vault_path"
				type="text"
				class="text-input path-input"
				bind:value={vaultPath}
				placeholder="~/Documents/my-vault"
			/>
			{#if isTauri}
				<button type="button" class="browse-btn" onclick={browseFolder}>
					<FolderOpen size={14} />
					Browse
				</button>
			{/if}
		</div>
		{#if browseError}
			<span class="field-error">{browseError}</span>
		{/if}
	</div>

	<div class="toggle-group">
		<div class="toggle-row">
			<div class="toggle-info">
				<span class="toggle-label">Watch for changes</span>
				<span class="toggle-hint">Re-index automatically when files change</span>
			</div>
			<button
				type="button"
				class="toggle"
				class:active={vaultWatch}
				onclick={() => (vaultWatch = !vaultWatch)}
				role="switch"
				aria-checked={vaultWatch}
				aria-label="Toggle file watching"
			>
				<span class="toggle-track">
					<span class="toggle-thumb"></span>
				</span>
			</button>
		</div>

		<div class="toggle-row">
			<div class="toggle-info">
				<span class="toggle-label">Loop back</span>
				<span class="toggle-hint">Write performance metadata into file frontmatter</span>
			</div>
			<button
				type="button"
				class="toggle"
				class:active={vaultLoopBack}
				onclick={() => (vaultLoopBack = !vaultLoopBack)}
				role="switch"
				aria-checked={vaultLoopBack}
				aria-label="Toggle loop back"
			>
				<span class="toggle-track">
					<span class="toggle-thumb"></span>
				</span>
			</button>
		</div>
	</div>
</div>

<style>
	.step {
		display: flex;
		flex-direction: column;
		gap: 24px;
	}

	h2 {
		font-size: 20px;
		font-weight: 700;
		color: var(--color-text);
		margin: 0;
	}

	.step-description {
		font-size: 14px;
		color: var(--color-text-muted);
		margin: -16px 0 0;
		line-height: 1.5;
	}

	.field-group {
		display: flex;
		flex-direction: column;
		gap: 10px;
	}

	.field-label {
		display: flex;
		align-items: center;
		gap: 6px;
		font-size: 13px;
		font-weight: 600;
		color: var(--color-text);
	}

	.field-error {
		font-size: 12px;
		color: var(--color-danger);
	}

	.path-row {
		display: flex;
		gap: 8px;
	}

	.text-input {
		padding: 10px 14px;
		background: var(--color-surface);
		border: 2px solid var(--color-border);
		border-radius: 8px;
		color: var(--color-text);
		font-size: 14px;
		font-family: var(--font-sans);
		outline: none;
		transition: border-color 0.15s;
	}

	.text-input:focus {
		border-color: var(--color-accent);
	}

	.path-input {
		flex: 1;
	}

	.browse-btn {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 10px 16px;
		background: var(--color-surface);
		border: 2px solid var(--color-border);
		border-radius: 8px;
		color: var(--color-text);
		font-size: 14px;
		cursor: pointer;
		white-space: nowrap;
		transition: all 0.15s;
	}

	.browse-btn:hover {
		border-color: var(--color-accent);
		background: color-mix(in srgb, var(--color-accent) 8%, var(--color-surface));
	}

	.toggle-group {
		display: flex;
		flex-direction: column;
		gap: 4px;
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: 8px;
		padding: 4px 16px;
	}

	.toggle-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 12px 0;
		cursor: default;
	}

	.toggle-row + .toggle-row {
		border-top: 1px solid var(--color-border-subtle);
	}

	.toggle-info {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.toggle-label {
		font-size: 14px;
		font-weight: 500;
		color: var(--color-text);
	}

	.toggle-hint {
		font-size: 12px;
		color: var(--color-text-muted);
	}

	.toggle {
		border: none;
		background: none;
		padding: 0;
		cursor: pointer;
	}

	.toggle-track {
		display: flex;
		align-items: center;
		width: 42px;
		height: 24px;
		padding: 2px;
		background: var(--color-border);
		border-radius: 12px;
		transition: background 0.2s;
	}

	.toggle.active .toggle-track {
		background: var(--color-accent);
	}

	.toggle-thumb {
		width: 20px;
		height: 20px;
		background: white;
		border-radius: 50%;
		transition: transform 0.2s;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
	}

	.toggle.active .toggle-thumb {
		transform: translateX(18px);
	}
</style>
