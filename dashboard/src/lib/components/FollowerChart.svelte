<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import {
		Chart,
		LineController,
		LineElement,
		PointElement,
		LinearScale,
		CategoryScale,
		Filler,
		Tooltip
	} from 'chart.js';
	import { followerSnapshots, loadDashboard } from '$lib/stores/analytics';

	Chart.register(LineController, LineElement, PointElement, LinearScale, CategoryScale, Filler, Tooltip);

	let canvas: HTMLCanvasElement;
	let chart: Chart | null = null;
	let selectedPeriod = $state<number>(30);

	const periods = [
		{ label: '7d', days: 7 },
		{ label: '30d', days: 30 },
		{ label: '90d', days: 90 }
	];

	function buildChart(snapshots: typeof $followerSnapshots) {
		const reversed = [...snapshots].reverse();
		const labels = reversed.map((s) => {
			const d = new Date(s.snapshot_date);
			return d.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
		});
		const data = reversed.map((s) => s.follower_count);

		if (chart) {
			chart.data.labels = labels;
			chart.data.datasets[0].data = data;
			chart.update();
			return;
		}

		const accentColor = getComputedStyle(document.documentElement)
			.getPropertyValue('--color-accent')
			.trim() || '#58a6ff';

		chart = new Chart(canvas, {
			type: 'line',
			data: {
				labels,
				datasets: [
					{
						label: 'Followers',
						data,
						borderColor: accentColor,
						backgroundColor: accentColor + '20',
						borderWidth: 2,
						pointRadius: 0,
						pointHoverRadius: 4,
						fill: true,
						tension: 0.3
					}
				]
			},
			options: {
				responsive: true,
				maintainAspectRatio: false,
				interaction: { mode: 'index', intersect: false },
				plugins: {
					tooltip: {
						backgroundColor: '#161b22',
						titleColor: '#e6edf3',
						bodyColor: '#8b949e',
						borderColor: '#30363d',
						borderWidth: 1,
						padding: 10,
						displayColors: false
					}
				},
				scales: {
					x: {
						grid: { color: '#21262d' },
						ticks: { color: '#6e7681', maxTicksLimit: 8 }
					},
					y: {
						grid: { color: '#21262d' },
						ticks: { color: '#6e7681' }
					}
				}
			}
		});
	}

	function selectPeriod(days: number) {
		selectedPeriod = days;
		loadDashboard(days);
	}

	const unsubscribe = followerSnapshots.subscribe((snapshots) => {
		if (canvas && snapshots.length > 0) {
			buildChart(snapshots);
		}
	});

	onMount(() => {
		const snapshots: typeof $followerSnapshots = [];
		followerSnapshots.subscribe((v) => {
			snapshots.length = 0;
			snapshots.push(...v);
		})();
		if (snapshots.length > 0) {
			buildChart(snapshots);
		}
	});

	onDestroy(() => {
		unsubscribe();
		chart?.destroy();
		chart = null;
	});
</script>

<div class="chart-container">
	<div class="chart-header">
		<h3>Followers</h3>
		<div class="period-selector">
			{#each periods as p}
				<button
					class="period-btn"
					class:active={selectedPeriod === p.days}
					onclick={() => selectPeriod(p.days)}
				>
					{p.label}
				</button>
			{/each}
		</div>
	</div>
	<div class="chart-body">
		<canvas bind:this={canvas}></canvas>
	</div>
</div>

<style>
	.chart-container {
		background-color: var(--color-surface);
		border: 1px solid var(--color-border-subtle);
		border-radius: 8px;
		padding: 20px;
	}

	.chart-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 16px;
	}

	h3 {
		margin: 0;
		font-size: 15px;
		font-weight: 600;
		color: var(--color-text);
	}

	.period-selector {
		display: flex;
		gap: 4px;
		background-color: var(--color-surface-active);
		border-radius: 6px;
		padding: 2px;
	}

	.period-btn {
		padding: 4px 10px;
		border: none;
		border-radius: 4px;
		font-size: 12px;
		font-weight: 600;
		color: var(--color-text-muted);
		background: transparent;
		cursor: pointer;
		transition:
			background-color 0.15s,
			color 0.15s;
	}

	.period-btn:hover {
		color: var(--color-text);
	}

	.period-btn.active {
		background-color: var(--color-surface);
		color: var(--color-text);
	}

	.chart-body {
		position: relative;
		height: 240px;
	}
</style>
