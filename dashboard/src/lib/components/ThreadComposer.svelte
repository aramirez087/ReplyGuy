<script lang="ts">
	import { type ThreadBlock } from '$lib/api';
	import { tweetWeightedLen, MAX_TWEET_CHARS } from '$lib/utils/tweetLength';
	import { Plus, Trash2, GripVertical } from 'lucide-svelte';

	let {
		initialBlocks = undefined,
		onchange,
		onvalidchange
	}: {
		initialBlocks?: ThreadBlock[];
		onchange: (blocks: ThreadBlock[]) => void;
		onvalidchange: (valid: boolean) => void;
	} = $props();

	function createDefaultBlocks(): ThreadBlock[] {
		return [
			{ id: crypto.randomUUID(), text: '', media_paths: [], order: 0 },
			{ id: crypto.randomUUID(), text: '', media_paths: [], order: 1 }
		];
	}

	let blocks = $state<ThreadBlock[]>(createDefaultBlocks());
	let focusedBlockId = $state<string | null>(null);

	// Sync when parent provides structurally different blocks (AI assist, recovery).
	// Compares block IDs to avoid re-syncing our own edits that were passed back up.
	$effect(() => {
		if (initialBlocks && initialBlocks.length > 0) {
			const currentIds = blocks.map((b) => b.id).join(',');
			const incomingIds = initialBlocks.map((b) => b.id).join(',');
			if (currentIds !== incomingIds) {
				blocks = [...initialBlocks];
			}
		}
	});

	const sortedBlocks = $derived(
		[...blocks].sort((a, b) => a.order - b.order)
	);

	const validationErrors = $derived.by(() => {
		const errors: string[] = [];
		const nonEmpty = blocks.filter((b) => b.text.trim().length > 0);
		if (nonEmpty.length < 2) {
			errors.push('Thread needs at least 2 tweets with content.');
		}
		for (const block of blocks) {
			if (tweetWeightedLen(block.text) > MAX_TWEET_CHARS) {
				const idx = sortedBlocks.findIndex((b) => b.id === block.id);
				errors.push(`Tweet ${idx + 1} exceeds ${MAX_TWEET_CHARS} characters.`);
			}
		}
		return errors;
	});

	const canSubmit = $derived(
		blocks.filter((b) => b.text.trim().length > 0).length >= 2 &&
			blocks.every((b) => tweetWeightedLen(b.text) <= MAX_TWEET_CHARS)
	);

	$effect(() => {
		onvalidchange(canSubmit);
	});

	function emitChange() {
		onchange([...blocks]);
	}

	function addBlock() {
		const maxOrder = blocks.reduce((max, b) => Math.max(max, b.order), -1);
		blocks = [
			...blocks,
			{ id: crypto.randomUUID(), text: '', media_paths: [], order: maxOrder + 1 }
		];
		emitChange();
	}

	function removeBlock(id: string) {
		if (blocks.length <= 2) return;
		blocks = blocks
			.filter((b) => b.id !== id)
			.sort((a, b) => a.order - b.order)
			.map((b, i) => ({ ...b, order: i }));
		emitChange();
	}

	function updateBlockText(id: string, text: string) {
		blocks = blocks.map((b) => (b.id === id ? { ...b, text } : b));
		emitChange();
	}

	function getCharCount(text: string): number {
		return tweetWeightedLen(text);
	}

	function isOverLimit(text: string): boolean {
		return getCharCount(text) > MAX_TWEET_CHARS;
	}

	function isWarning(text: string): boolean {
		return getCharCount(text) > 260 && !isOverLimit(text);
	}

	export function getBlocks(): ThreadBlock[] {
		return [...blocks];
	}

	export function setBlocks(newBlocks: ThreadBlock[]) {
		blocks = newBlocks;
		emitChange();
	}
</script>

<div class="thread-composer" role="region" aria-label="Thread editor">
	{#each sortedBlocks as block, i (block.id)}
		<div
			class="tweet-card"
			class:focused={focusedBlockId === block.id}
			class:over-limit={isOverLimit(block.text)}
		>
			<div class="card-gutter">
				<div class="card-number" aria-hidden="true">{i + 1}</div>
				<div class="drag-handle-placeholder" title="Drag to reorder (coming soon)" aria-hidden="true">
					<GripVertical size={14} />
				</div>
			</div>
			<div class="card-body">
				<textarea
					class="card-textarea"
					class:over-limit={isOverLimit(block.text)}
					placeholder={i === 0 ? 'Start your thread...' : `Tweet ${i + 1}...`}
					value={block.text}
					oninput={(e) => updateBlockText(block.id, e.currentTarget.value)}
					onfocus={() => (focusedBlockId = block.id)}
					onblur={() => {
						if (focusedBlockId === block.id) focusedBlockId = null;
					}}
					rows={3}
					aria-label={`Tweet ${i + 1} of ${sortedBlocks.length}`}
				></textarea>
				<div class="card-footer">
					<div
						class="char-counter"
						class:over-limit={isOverLimit(block.text)}
						class:warning={isWarning(block.text)}
						aria-live="polite"
						aria-label="Character count"
					>
						{getCharCount(block.text)}/{MAX_TWEET_CHARS}
					</div>
					{#if sortedBlocks.length > 2}
						<button
							class="remove-card-btn"
							onclick={() => removeBlock(block.id)}
							aria-label="Remove tweet {i + 1}"
						>
							<Trash2 size={12} />
						</button>
					{/if}
				</div>
			</div>
			{#if i < sortedBlocks.length - 1}
				<div class="thread-line" aria-hidden="true"></div>
			{/if}
		</div>
	{/each}

	<button class="add-card-btn" onclick={addBlock} aria-label="Add another tweet to thread">
		<Plus size={14} />
		Add tweet
	</button>

	{#if validationErrors.length > 0}
		<div class="validation-summary" role="status" aria-live="polite">
			{#each validationErrors as err}
				<p class="validation-error">{err}</p>
			{/each}
		</div>
	{/if}
</div>

<style>
	.thread-composer {
		display: flex;
		flex-direction: column;
		gap: 0;
	}

	.tweet-card {
		position: relative;
		display: flex;
		gap: 8px;
		padding: 12px;
		border: 1px solid var(--color-border-subtle);
		border-radius: 8px;
		background: var(--color-surface);
		margin-bottom: 8px;
		transition: border-color 0.15s ease, box-shadow 0.15s ease;
	}

	.tweet-card.focused {
		border-color: var(--color-accent);
		box-shadow: 0 0 0 1px color-mix(in srgb, var(--color-accent) 20%, transparent);
	}

	.tweet-card.over-limit {
		border-color: var(--color-danger);
	}

	.card-gutter {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 4px;
		flex-shrink: 0;
		width: 24px;
		padding-top: 2px;
	}

	.card-number {
		font-size: 11px;
		font-weight: 600;
		color: var(--color-text-muted);
		font-family: var(--font-mono);
		line-height: 1;
	}

	.drag-handle-placeholder {
		color: var(--color-text-subtle);
		cursor: default;
		opacity: 0.4;
		display: flex;
		align-items: center;
	}

	.card-body {
		flex: 1;
		min-width: 0;
	}

	.card-textarea {
		width: 100%;
		padding: 8px 10px;
		border: 1px solid var(--color-border);
		border-radius: 6px;
		background: var(--color-base);
		color: var(--color-text);
		font-size: 13px;
		font-family: var(--font-sans);
		line-height: 1.5;
		resize: vertical;
		box-sizing: border-box;
		transition: border-color 0.15s ease;
	}

	.card-textarea:focus {
		outline: none;
		border-color: var(--color-accent);
	}

	.card-textarea.over-limit {
		border-color: var(--color-danger);
	}

	.card-footer {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-top: 4px;
	}

	.char-counter {
		font-size: 11px;
		color: var(--color-text-subtle);
		font-family: var(--font-mono);
	}

	.char-counter.warning {
		color: var(--color-warning);
	}

	.char-counter.over-limit {
		color: var(--color-danger);
		font-weight: 600;
	}

	.remove-card-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		border: none;
		border-radius: 4px;
		background: transparent;
		color: var(--color-text-subtle);
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.remove-card-btn:hover {
		background: color-mix(in srgb, var(--color-danger) 10%, transparent);
		color: var(--color-danger);
	}

	.thread-line {
		position: absolute;
		left: 23px;
		bottom: -9px;
		width: 2px;
		height: 8px;
		background: var(--color-border-subtle);
	}

	.add-card-btn {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 8px 12px;
		border: 1px dashed var(--color-border);
		border-radius: 6px;
		background: transparent;
		color: var(--color-text-muted);
		font-size: 12px;
		cursor: pointer;
		transition: all 0.15s ease;
		margin-top: 4px;
	}

	.add-card-btn:hover {
		border-color: var(--color-accent);
		color: var(--color-accent);
		background: color-mix(in srgb, var(--color-accent) 5%, transparent);
	}

	.validation-summary {
		margin-top: 8px;
		padding: 8px 12px;
		border-radius: 6px;
		background: color-mix(in srgb, var(--color-danger) 8%, transparent);
	}

	.validation-error {
		font-size: 12px;
		color: var(--color-danger);
		margin: 0;
		padding: 2px 0;
	}
</style>
