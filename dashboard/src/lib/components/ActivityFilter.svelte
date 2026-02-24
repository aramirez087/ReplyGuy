<script lang="ts">
	import { Search, MessageSquare, FileText, AlertTriangle, List } from 'lucide-svelte';

	interface Props {
		selected: string;
		onselect: (category: string) => void;
	}

	let { selected, onselect }: Props = $props();

	const filters = [
		{ value: 'all', label: 'All', icon: List },
		{ value: 'search', label: 'Discovery', icon: Search },
		{ value: 'reply', label: 'Replies', icon: MessageSquare },
		{ value: 'tweet', label: 'Content', icon: FileText },
		{ value: 'errors', label: 'Errors', icon: AlertTriangle }
	] as const;
</script>

<div class="filter-chips">
	{#each filters as filter}
		{@const Icon = filter.icon}
		<button
			class="chip"
			class:active={selected === filter.value}
			onclick={() => onselect(filter.value)}
		>
			<Icon size={14} />
			<span>{filter.label}</span>
		</button>
	{/each}
</div>

<style>
	.filter-chips {
		display: flex;
		gap: 6px;
		flex-wrap: wrap;
	}

	.chip {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 6px 12px;
		border: 1px solid var(--color-border);
		border-radius: 6px;
		background-color: var(--color-surface);
		color: var(--color-text-muted);
		font-size: 13px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.chip:hover {
		background-color: var(--color-surface-hover);
		color: var(--color-text);
	}

	.chip.active {
		background-color: var(--color-accent);
		border-color: var(--color-accent);
		color: white;
	}
</style>
