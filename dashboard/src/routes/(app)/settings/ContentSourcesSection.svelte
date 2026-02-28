<script lang="ts">
	import { onMount } from 'svelte';
	import { FolderOpen } from 'lucide-svelte';
	import SettingsSection from '$lib/components/settings/SettingsSection.svelte';
	import { draft, updateDraft } from '$lib/stores/settings';

	let isTauri = $state(false);
	let browseError = $state('');

	const currentSource = $derived($draft?.content_sources?.sources?.[0]);
	const sourcePath = $derived(currentSource?.path ?? '');
	const sourceWatch = $derived(currentSource?.watch ?? true);
	const sourceLoopBack = $derived(currentSource?.loop_back_enabled ?? true);
	const filePatterns = $derived(currentSource?.file_patterns ?? ['*.md', '*.txt']);

	onMount(async () => {
		try {
			await import('@tauri-apps/plugin-dialog');
			isTauri = true;
		} catch {
			// Not in Tauri context
		}
	});

	function updateSource(updates: Record<string, unknown>) {
		const current = $draft?.content_sources?.sources?.[0];
		updateDraft('content_sources', {
			sources: [{
				source_type: current?.source_type ?? 'local_fs',
				path: current?.path ?? null,
				watch: current?.watch ?? true,
				file_patterns: current?.file_patterns ?? ['*.md', '*.txt'],
				loop_back_enabled: current?.loop_back_enabled ?? true,
				...updates
			}]
		});
		browseError = '';
	}

	function handlePathInput(e: Event) {
		const value = (e.target as HTMLInputElement).value;
		updateSource({ path: value || null });
	}

	async function browseFolder() {
		browseError = '';
		try {
			const { open } = await import('@tauri-apps/plugin-dialog');
			const selected = await open({
				directory: true,
				title: 'Select content source folder'
			});
			if (selected) {
				updateSource({ path: selected as string });
			}
		} catch {
			browseError = 'Could not open folder picker';
		}
	}

	function toggleWatch() {
		updateSource({ watch: !sourceWatch });
	}

	function toggleLoopBack() {
		updateSource({ loop_back_enabled: !sourceLoopBack });
	}
</script>

{#if $draft}
<SettingsSection
	id="sources"
	title="Content Sources"
	description="Connect a local notes folder for the Watchtower to index"
	icon={FolderOpen}
>
	<div class="field-grid">
		<div class="field full-width">
			<label class="field-label" for="source_path">Vault / Notes Folder</label>
			<div class="path-row">
				<input
					id="source_path"
					type="text"
					class="text-input path-input"
					value={sourcePath}
					oninput={handlePathInput}
					placeholder="~/Documents/my-vault"
				/>
				{#if isTauri}
					<button type="button" class="browse-btn" onclick={browseFolder}>
						<FolderOpen size={14} />
						Browse
					</button>
				{/if}
			</div>
			<span class="field-hint">
				{#if isTauri}
					Click Browse to select your Obsidian vault or notes folder.
				{:else}
					Enter the full path to your Obsidian vault or notes folder.
				{/if}
			</span>
			{#if browseError}
				<span class="field-error">{browseError}</span>
			{/if}
		</div>

		<div class="field full-width">
			<div class="toggle-row">
				<div class="toggle-info">
					<span class="field-label">Watch for Changes</span>
					<span class="field-hint">
						Automatically re-index when files are added or modified
					</span>
				</div>
				<button
					type="button"
					class="toggle"
					class:active={sourceWatch}
					onclick={toggleWatch}
					role="switch"
					aria-checked={sourceWatch}
					aria-label="Toggle file watching"
				>
					<span class="toggle-track">
						<span class="toggle-thumb"></span>
					</span>
				</button>
			</div>
		</div>

		<div class="field full-width">
			<div class="toggle-row">
				<div class="toggle-info">
					<span class="field-label">Loop Back</span>
					<span class="field-hint">
						Write performance metadata back into source file frontmatter
					</span>
				</div>
				<button
					type="button"
					class="toggle"
					class:active={sourceLoopBack}
					onclick={toggleLoopBack}
					role="switch"
					aria-checked={sourceLoopBack}
					aria-label="Toggle loop back"
				>
					<span class="toggle-track">
						<span class="toggle-thumb"></span>
					</span>
				</button>
			</div>
		</div>

		<div class="field full-width">
			<span class="field-label">File Patterns</span>
			<div class="patterns">
				{#each filePatterns as pattern}
					<span class="pattern-tag">{pattern}</span>
				{/each}
			</div>
			<span class="field-hint">
				File patterns are configured in config.toml. Default: *.md, *.txt
			</span>
		</div>
	</div>
</SettingsSection>
{/if}

<style>
	.field-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 20px;
	}

	.field {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.full-width {
		grid-column: 1 / -1;
	}

	.field-label {
		font-size: 13px;
		font-weight: 500;
		color: var(--color-text);
	}

	.field-hint {
		font-size: 12px;
		color: var(--color-text-subtle);
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
		padding: 8px 12px;
		background: var(--color-base);
		border: 1px solid var(--color-border);
		border-radius: 6px;
		color: var(--color-text);
		font-size: 13px;
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
		padding: 8px 14px;
		background: var(--color-surface-hover);
		border: 1px solid var(--color-border);
		border-radius: 6px;
		color: var(--color-text);
		font-size: 13px;
		cursor: pointer;
		white-space: nowrap;
		transition: all 0.15s;
	}

	.browse-btn:hover {
		background: var(--color-accent);
		border-color: var(--color-accent);
		color: white;
	}

	.toggle-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 8px 0;
	}

	.toggle-info {
		display: flex;
		flex-direction: column;
		gap: 2px;
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

	.patterns {
		display: flex;
		gap: 6px;
		flex-wrap: wrap;
	}

	.pattern-tag {
		padding: 4px 10px;
		background: color-mix(in srgb, var(--color-accent) 10%, transparent);
		border: 1px solid color-mix(in srgb, var(--color-accent) 20%, transparent);
		border-radius: 4px;
		font-size: 12px;
		font-family: monospace;
		color: var(--color-accent);
	}
</style>
