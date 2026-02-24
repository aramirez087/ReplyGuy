<script lang="ts">
	interface Props {
		metadata: string | null;
	}

	let { metadata }: Props = $props();
	let expanded = $state(false);

	const parsed = $derived.by(() => {
		if (!metadata) return null;
		try {
			return JSON.parse(metadata);
		} catch {
			return null;
		}
	});

	const hasScore = $derived(parsed && typeof parsed.score === 'number');
</script>

{#if parsed}
	<div class="metadata-row">
		{#if hasScore}
			<button class="score-toggle" onclick={() => (expanded = !expanded)}>
				<span class="toggle-arrow">{expanded ? '\u25BC' : '\u25B6'}</span>
				Score: {parsed.score}
			</button>
		{/if}
		{#each Object.entries(parsed) as [key, value]}
			{#if key !== 'score' && key !== 'score_breakdown' && !expanded}
				<span class="meta-tag">{key}: {value}</span>
			{/if}
		{/each}
	</div>
	{#if expanded && parsed.score_breakdown}
		<div class="score-details">
			{#each Object.entries(parsed.score_breakdown) as [signal, val]}
				<div class="score-row">
					<span class="signal-name">{signal}</span>
					<span class="signal-value">{val}</span>
				</div>
			{/each}
		</div>
	{:else if expanded}
		<div class="score-details">
			{#each Object.entries(parsed) as [key, value]}
				<div class="score-row">
					<span class="signal-name">{key}</span>
					<span class="signal-value">{value}</span>
				</div>
			{/each}
		</div>
	{/if}
{/if}

<style>
	.metadata-row {
		display: flex;
		align-items: center;
		gap: 8px;
		flex-wrap: wrap;
		margin-top: 6px;
	}

	.score-toggle {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		padding: 2px 8px;
		border: 1px solid var(--color-border-subtle);
		border-radius: 4px;
		background: transparent;
		color: var(--color-accent);
		font-size: 12px;
		font-weight: 600;
		cursor: pointer;
		font-variant-numeric: tabular-nums;
	}

	.score-toggle:hover {
		background-color: var(--color-surface-hover);
	}

	.toggle-arrow {
		font-size: 9px;
		color: var(--color-text-subtle);
	}

	.meta-tag {
		font-size: 11px;
		padding: 2px 6px;
		border-radius: 4px;
		background-color: var(--color-surface-active);
		color: var(--color-text-muted);
	}

	.score-details {
		margin-top: 8px;
		padding: 8px 12px;
		border: 1px solid var(--color-border-subtle);
		border-radius: 6px;
		background-color: var(--color-surface-active);
	}

	.score-row {
		display: flex;
		justify-content: space-between;
		padding: 3px 0;
		font-size: 12px;
	}

	.signal-name {
		color: var(--color-text-muted);
		text-transform: capitalize;
	}

	.signal-value {
		font-weight: 600;
		color: var(--color-text);
		font-variant-numeric: tabular-nums;
	}
</style>
