<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		label: string;
		value: string | number;
		change?: number | null;
		icon?: Snippet;
	}

	let { label, value, change = null, icon }: Props = $props();

	const changeClass = $derived(
		change === null || change === 0 ? '' : change > 0 ? 'positive' : 'negative'
	);
	const changeText = $derived(
		change === null || change === 0
			? ''
			: change > 0
				? `+${change}`
				: `${change}`
	);
</script>

<div class="stat-card">
	{#if icon}
		<div class="stat-icon">
			{@render icon()}
		</div>
	{/if}
	<div class="stat-body">
		<span class="stat-label">{label}</span>
		<span class="stat-value">{value}</span>
		{#if changeText}
			<span class="stat-change {changeClass}">{changeText}</span>
		{/if}
	</div>
</div>

<style>
	.stat-card {
		display: flex;
		align-items: center;
		gap: 14px;
		padding: 16px;
		background-color: var(--color-surface);
		border: 1px solid var(--color-border-subtle);
		border-radius: 8px;
	}

	.stat-icon {
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

	.stat-body {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.stat-label {
		font-size: 11px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: var(--color-text-subtle);
	}

	.stat-value {
		font-size: 22px;
		font-weight: 700;
		color: var(--color-text);
		line-height: 1.2;
	}

	.stat-change {
		font-size: 12px;
		font-weight: 600;
	}

	.stat-change.positive {
		color: var(--color-success);
	}

	.stat-change.negative {
		color: var(--color-danger);
	}
</style>
