<script lang="ts">
	import { ChevronDown, ChevronRight, History } from 'lucide-svelte';
	import { api, type EditHistoryEntry } from '$lib/api';

	interface Props {
		approvalId: number;
	}

	let { approvalId }: Props = $props();
	let expanded = $state(false);
	let entries = $state<EditHistoryEntry[]>([]);
	let loaded = $state(false);

	async function toggle() {
		expanded = !expanded;
		if (expanded && !loaded) {
			try {
				entries = await api.approval.editHistory(approvalId);
				loaded = true;
			} catch {
				entries = [];
				loaded = true;
			}
		}
	}

	function relativeTime(iso: string): string {
		const diff = Date.now() - new Date(iso).getTime();
		const mins = Math.floor(diff / 60_000);
		if (mins < 1) return 'just now';
		if (mins < 60) return `${mins}m ago`;
		const hours = Math.floor(mins / 60);
		if (hours < 24) return `${hours}h ago`;
		const days = Math.floor(hours / 24);
		return `${days}d ago`;
	}
</script>

<div class="edit-history">
	<button class="toggle-btn" onclick={toggle}>
		{#if expanded}
			<ChevronDown size={12} />
		{:else}
			<ChevronRight size={12} />
		{/if}
		<History size={12} />
		Edit History
	</button>

	{#if expanded}
		<div class="history-list">
			{#if entries.length === 0}
				<span class="history-empty">No edits recorded</span>
			{:else}
				{#each entries as entry}
					<div class="history-entry">
						<div class="entry-header">
							<span class="entry-editor">{entry.editor}</span>
							<span class="entry-field">edited {entry.field}</span>
							<span class="entry-time">{relativeTime(entry.created_at)}</span>
						</div>
						<div class="entry-diff">
							<div class="diff-old">{entry.old_value}</div>
							<div class="diff-new">{entry.new_value}</div>
						</div>
					</div>
				{/each}
			{/if}
		</div>
	{/if}
</div>

<style>
	.edit-history {
		margin-top: 4px;
	}

	.toggle-btn {
		display: flex;
		align-items: center;
		gap: 4px;
		padding: 2px 6px;
		border: none;
		background: none;
		color: var(--color-text-subtle);
		font-size: 11px;
		font-weight: 500;
		cursor: pointer;
		border-radius: 3px;
		transition: all 0.15s ease;
	}

	.toggle-btn:hover {
		background-color: var(--color-surface-hover);
		color: var(--color-text);
	}

	.history-list {
		margin-top: 6px;
		padding-left: 16px;
		border-left: 2px solid var(--color-border-subtle);
	}

	.history-empty {
		font-size: 11px;
		color: var(--color-text-subtle);
		font-style: italic;
	}

	.history-entry {
		padding: 6px 0;
		border-bottom: 1px solid var(--color-border-subtle);
	}

	.history-entry:last-child {
		border-bottom: none;
	}

	.entry-header {
		display: flex;
		align-items: center;
		gap: 6px;
		margin-bottom: 4px;
	}

	.entry-editor {
		font-size: 11px;
		font-weight: 600;
		color: var(--color-accent);
	}

	.entry-field {
		font-size: 11px;
		color: var(--color-text-muted);
	}

	.entry-time {
		margin-left: auto;
		font-size: 10px;
		color: var(--color-text-subtle);
	}

	.entry-diff {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.diff-old {
		font-size: 11px;
		color: var(--color-danger);
		text-decoration: line-through;
		padding: 2px 6px;
		background-color: color-mix(in srgb, var(--color-danger) 5%, transparent);
		border-radius: 3px;
		word-break: break-word;
		white-space: pre-wrap;
	}

	.diff-new {
		font-size: 11px;
		color: var(--color-success);
		padding: 2px 6px;
		background-color: color-mix(in srgb, var(--color-success) 5%, transparent);
		border-radius: 3px;
		word-break: break-word;
		white-space: pre-wrap;
	}
</style>
