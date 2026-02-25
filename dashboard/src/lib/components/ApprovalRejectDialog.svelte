<script lang="ts">
	import { XCircle, X } from 'lucide-svelte';

	interface Props {
		itemId: number;
		onConfirm: (id: number, notes: string) => void;
		onCancel: () => void;
	}

	let { itemId, onConfirm, onCancel }: Props = $props();
	let notes = $state('');

	function handleConfirm() {
		onConfirm(itemId, notes.trim());
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			e.preventDefault();
			onCancel();
		} else if (e.key === 'Enter' && (e.metaKey || e.ctrlKey)) {
			e.preventDefault();
			handleConfirm();
		}
	}
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="reject-dialog" onkeydown={handleKeydown}>
	<div class="reject-header">
		<span class="reject-title">
			<XCircle size={14} />
			Reject Item
		</span>
		<button class="reject-close" onclick={onCancel}>
			<X size={14} />
		</button>
	</div>
	<textarea
		class="reject-notes"
		bind:value={notes}
		placeholder="Optional: reason for rejection..."
		rows="2"
	></textarea>
	<div class="reject-actions">
		<button class="reject-confirm" onclick={handleConfirm}>
			<XCircle size={12} />
			Reject
		</button>
		<button class="reject-cancel" onclick={onCancel}>Cancel</button>
	</div>
</div>

<style>
	.reject-dialog {
		padding: 12px;
		border: 1px solid var(--color-danger);
		border-radius: 6px;
		background-color: color-mix(in srgb, var(--color-danger) 5%, var(--color-surface));
	}

	.reject-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 8px;
	}

	.reject-title {
		display: flex;
		align-items: center;
		gap: 6px;
		font-size: 13px;
		font-weight: 600;
		color: var(--color-danger);
	}

	.reject-close {
		padding: 2px;
		border: none;
		background: none;
		color: var(--color-text-subtle);
		cursor: pointer;
		border-radius: 3px;
		display: flex;
		align-items: center;
	}

	.reject-close:hover {
		background-color: var(--color-surface-hover);
		color: var(--color-text);
	}

	.reject-notes {
		width: 100%;
		padding: 8px;
		border: 1px solid var(--color-border);
		border-radius: 4px;
		background-color: var(--color-base);
		color: var(--color-text);
		font-family: var(--font-mono);
		font-size: 12px;
		line-height: 1.4;
		resize: vertical;
		outline: none;
		box-sizing: border-box;
		margin-bottom: 8px;
	}

	.reject-notes:focus {
		border-color: var(--color-danger);
		box-shadow: 0 0 0 2px color-mix(in srgb, var(--color-danger) 20%, transparent);
	}

	.reject-actions {
		display: flex;
		gap: 6px;
		justify-content: flex-end;
	}

	.reject-confirm {
		display: flex;
		align-items: center;
		gap: 4px;
		padding: 4px 12px;
		border: 1px solid var(--color-danger);
		border-radius: 4px;
		background-color: var(--color-danger);
		color: white;
		font-size: 12px;
		font-weight: 600;
		cursor: pointer;
		transition: opacity 0.15s ease;
	}

	.reject-confirm:hover {
		opacity: 0.9;
	}

	.reject-cancel {
		padding: 4px 12px;
		border: 1px solid var(--color-border);
		border-radius: 4px;
		background-color: var(--color-surface);
		color: var(--color-text-muted);
		font-size: 12px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.reject-cancel:hover {
		background-color: var(--color-surface-hover);
		color: var(--color-text);
	}
</style>
