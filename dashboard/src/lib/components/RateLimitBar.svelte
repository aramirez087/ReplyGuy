<script lang="ts">
	import type { ActionUsage } from '$lib/api';

	interface Props {
		label: string;
		usage: ActionUsage;
	}

	let { label, usage }: Props = $props();

	const percentage = $derived(usage.max > 0 ? (usage.used / usage.max) * 100 : 0);
	const barClass = $derived(percentage >= 80 ? 'danger' : percentage >= 60 ? 'warning' : 'success');
</script>

<div class="limit-bar">
	<div class="limit-header">
		<span class="limit-label">{label}</span>
		<span class="limit-count">{usage.used}/{usage.max}</span>
	</div>
	<div class="limit-track">
		<div class="limit-fill {barClass}" style="width: {Math.min(percentage, 100)}%"></div>
	</div>
</div>

<style>
	.limit-bar {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.limit-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.limit-label {
		font-size: 12px;
		font-weight: 600;
		color: var(--color-text);
		text-transform: capitalize;
	}

	.limit-count {
		font-size: 12px;
		font-weight: 600;
		color: var(--color-text-muted);
		font-variant-numeric: tabular-nums;
	}

	.limit-track {
		height: 6px;
		border-radius: 3px;
		background-color: var(--color-surface-active);
		overflow: hidden;
	}

	.limit-fill {
		height: 100%;
		border-radius: 3px;
		transition: width 0.3s ease;
	}

	.limit-fill.success {
		background-color: var(--color-success);
	}

	.limit-fill.warning {
		background-color: var(--color-warning);
	}

	.limit-fill.danger {
		background-color: var(--color-danger);
	}
</style>
