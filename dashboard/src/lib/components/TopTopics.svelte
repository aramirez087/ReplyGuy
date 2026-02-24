<script lang="ts">
	import type { ContentScore } from '$lib/api';

	interface Props {
		topics: ContentScore[];
	}

	let { topics }: Props = $props();

	const maxScore = $derived(
		topics.length > 0 ? Math.max(...topics.map((t) => t.avg_performance)) : 1
	);
</script>

<div class="topics-container">
	<h3>Top Topics</h3>
	{#if topics.length === 0}
		<div class="empty-state">No topic data yet. Start posting to see what works best.</div>
	{:else}
		<div class="topic-list">
			{#each topics as topic, i}
				<div class="topic-row">
					<div class="topic-rank">{i + 1}</div>
					<div class="topic-info">
						<div class="topic-header">
							<span class="topic-name">{topic.topic}</span>
							<span class="topic-format">{topic.format}</span>
						</div>
						<div class="topic-bar-track">
							<div
								class="topic-bar-fill"
								style="width: {(topic.avg_performance / maxScore) * 100}%"
							></div>
						</div>
						<div class="topic-meta">
							<span>{topic.total_posts} posts</span>
							<span>avg {topic.avg_performance.toFixed(1)}</span>
						</div>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.topics-container {
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

	.topic-list {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.topic-row {
		display: flex;
		align-items: flex-start;
		gap: 12px;
	}

	.topic-rank {
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 6px;
		background-color: var(--color-surface-active);
		color: var(--color-text-muted);
		font-size: 12px;
		font-weight: 700;
		flex-shrink: 0;
	}

	.topic-info {
		flex: 1;
		min-width: 0;
	}

	.topic-header {
		display: flex;
		align-items: center;
		gap: 8px;
		margin-bottom: 6px;
	}

	.topic-name {
		font-size: 13px;
		font-weight: 600;
		color: var(--color-text);
	}

	.topic-format {
		font-size: 11px;
		padding: 1px 6px;
		border-radius: 4px;
		background-color: var(--color-surface-active);
		color: var(--color-text-muted);
	}

	.topic-bar-track {
		height: 6px;
		border-radius: 3px;
		background-color: var(--color-surface-active);
		overflow: hidden;
		margin-bottom: 4px;
	}

	.topic-bar-fill {
		height: 100%;
		border-radius: 3px;
		background-color: var(--color-accent);
		transition: width 0.3s ease;
	}

	.topic-meta {
		display: flex;
		justify-content: space-between;
		font-size: 11px;
		color: var(--color-text-subtle);
	}
</style>
