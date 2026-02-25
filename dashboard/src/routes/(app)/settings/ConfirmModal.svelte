<script lang="ts">
	import { AlertTriangle } from 'lucide-svelte';

	interface Props {
		onConfirm: () => void;
		onCancel: () => void;
	}

	let { onConfirm, onCancel }: Props = $props();
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="modal-backdrop" onclick={onCancel}>
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="modal" onclick={(e) => e.stopPropagation()}>
		<div class="modal-icon">
			<AlertTriangle size={24} />
		</div>
		<h3>Credential Change Warning</h3>
		<p>
			You're changing API credentials or LLM provider settings. This will affect
			active automation. Are you sure you want to save?
		</p>
		<div class="modal-actions">
			<button
				type="button"
				class="discard-btn"
				onclick={onCancel}
			>
				Cancel
			</button>
			<button type="button" class="save-btn" onclick={onConfirm}>
				Save Anyway
			</button>
		</div>
	</div>
</div>

<style>
	.modal-backdrop {
		position: fixed;
		inset: 0;
		z-index: 100;
		background: rgba(0, 0, 0, 0.6);
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.modal {
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: 12px;
		padding: 24px;
		max-width: 420px;
		text-align: center;
	}

	.modal-icon {
		color: var(--color-warning);
		margin-bottom: 12px;
	}

	.modal h3 {
		font-size: 16px;
		font-weight: 600;
		color: var(--color-text);
		margin: 0 0 8px;
	}

	.modal p {
		font-size: 13px;
		color: var(--color-text-muted);
		margin: 0 0 20px;
		line-height: 1.5;
	}

	.modal-actions {
		display: flex;
		gap: 8px;
		justify-content: center;
	}

	.discard-btn {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 7px 14px;
		background: none;
		color: var(--color-text-muted);
		border: 1px solid var(--color-border);
		border-radius: 6px;
		font-size: 13px;
		cursor: pointer;
		transition:
			background 0.15s,
			color 0.15s;
	}

	.discard-btn:hover {
		background: var(--color-surface-hover);
		color: var(--color-text);
	}

	.save-btn {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 7px 16px;
		background: var(--color-accent);
		color: white;
		border: none;
		border-radius: 6px;
		font-size: 13px;
		font-weight: 500;
		cursor: pointer;
		transition:
			background 0.15s,
			opacity 0.15s;
	}

	.save-btn:hover {
		background: var(--color-accent-hover);
	}
</style>
