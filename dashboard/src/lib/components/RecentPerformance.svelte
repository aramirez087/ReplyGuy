<script lang="ts">
	import type { PerformanceItem } from '$lib/api';

	interface Props {
		items: PerformanceItem[];
	}

	let { items }: Props = $props();

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

	function badgeClass(type: string): string {
		if (type === 'reply') return 'badge-reply';
		if (type === 'tweet') return 'badge-tweet';
		return 'badge-thread';
	}
</script>

<div class="perf-container">
	<h3>Recent Performance</h3>
	{#if items.length === 0}
		<div class="empty-state">No performance data yet. Metrics appear after content is posted.</div>
	{:else}
		<div class="perf-table">
			<div class="perf-header">
				<span class="col-type">Type</span>
				<span class="col-content">Content</span>
				<span class="col-metric">Likes</span>
				<span class="col-metric">Replies</span>
				<span class="col-metric">RTs</span>
				<span class="col-metric">Impr.</span>
				<span class="col-score">Score</span>
				<span class="col-time">When</span>
			</div>
			{#each items as item}
				<div class="perf-row">
					<span class="col-type">
						<span class="badge {badgeClass(item.content_type)}">{item.content_type}</span>
					</span>
					<span class="col-content" title={item.content_preview}>
						{item.content_preview}
					</span>
					<span class="col-metric">{item.likes}</span>
					<span class="col-metric">{item.replies_received}</span>
					<span class="col-metric">{item.retweets}</span>
					<span class="col-metric">{item.impressions.toLocaleString()}</span>
					<span class="col-score">{item.performance_score.toFixed(1)}</span>
					<span class="col-time">{relativeTime(item.posted_at)}</span>
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.perf-container {
		background-color: var(--color-surface);
		border: 1px solid var(--color-border-subtle);
		border-radius: 8px;
		padding: 20px;
	}

	h3 {
		margin: 0 0 16px;
		font-size: 15px;
		font-weight: 600;
		color: var(--color-text);
	}

	.empty-state {
		padding: 24px 0;
		text-align: center;
		color: var(--color-text-subtle);
		font-size: 13px;
	}

	.perf-table {
		display: flex;
		flex-direction: column;
		gap: 0;
		overflow-x: auto;
	}

	.perf-header,
	.perf-row {
		display: grid;
		grid-template-columns: 64px 1fr 52px 52px 44px 72px 56px 64px;
		align-items: center;
		gap: 8px;
		padding: 8px 4px;
	}

	.perf-header {
		font-size: 11px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.04em;
		color: var(--color-text-subtle);
		border-bottom: 1px solid var(--color-border-subtle);
	}

	.perf-row {
		font-size: 13px;
		color: var(--color-text-muted);
		border-bottom: 1px solid var(--color-border-subtle);
	}

	.perf-row:last-child {
		border-bottom: none;
	}

	.col-content {
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.col-metric,
	.col-score,
	.col-time {
		text-align: right;
		font-variant-numeric: tabular-nums;
	}

	.col-score {
		font-weight: 600;
		color: var(--color-accent);
	}

	.col-time {
		font-size: 11px;
		color: var(--color-text-subtle);
	}

	.badge {
		display: inline-block;
		font-size: 11px;
		font-weight: 600;
		padding: 2px 6px;
		border-radius: 4px;
	}

	.badge-reply {
		background-color: #58a6ff20;
		color: var(--color-accent);
	}

	.badge-tweet {
		background-color: #3fb95020;
		color: var(--color-success);
	}

	.badge-thread {
		background-color: #d2992220;
		color: var(--color-warning);
	}
</style>
