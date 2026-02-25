<script lang="ts">
	import { accounts, currentAccountId, switchAccount, type Account } from "$lib/stores/accounts";
	import { ChevronDown, User } from "lucide-svelte";

	let open = $state(false);

	const current = $derived(
		$accounts.find((a: Account) => a.id === $currentAccountId)
	);

	function toggle() {
		open = !open;
	}

	function select(id: string) {
		switchAccount(id);
		open = false;
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === "Escape") {
			open = false;
		}
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if $accounts.length > 1}
	<div class="account-switcher">
		<button class="account-trigger" onclick={toggle}>
			<User size={14} />
			<span class="account-label">
				{current?.x_username ? `@${current.x_username}` : current?.label ?? 'Default'}
			</span>
			<ChevronDown size={12} />
		</button>

		{#if open}
			<!-- svelte-ignore a11y_click_events_have_key_events -->
			<!-- svelte-ignore a11y_no_static_element_interactions -->
			<div class="account-backdrop" onclick={() => (open = false)}></div>
			<div class="account-dropdown">
				{#each $accounts as account (account.id)}
					<button
						class="account-option"
						class:active={account.id === $currentAccountId}
						onclick={() => select(account.id)}
					>
						<User size={14} />
						<span>
							{account.x_username ? `@${account.x_username}` : account.label}
						</span>
					</button>
				{/each}
			</div>
		{/if}
	</div>
{/if}

<style>
	.account-switcher {
		position: relative;
		padding: 0 8px;
		margin-bottom: 4px;
	}

	.account-trigger {
		display: flex;
		align-items: center;
		gap: 8px;
		width: 100%;
		padding: 6px 10px;
		border: 1px solid var(--color-border-subtle);
		border-radius: 6px;
		background: var(--color-surface);
		color: var(--color-text-muted);
		cursor: pointer;
		font-size: 12px;
		font-weight: 500;
		transition: background-color 0.15s ease, border-color 0.15s ease;
	}

	.account-trigger:hover {
		background-color: var(--color-surface-hover);
		border-color: var(--color-border);
	}

	.account-label {
		flex: 1;
		text-align: left;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.account-backdrop {
		position: fixed;
		inset: 0;
		z-index: 99;
	}

	.account-dropdown {
		position: absolute;
		top: 100%;
		left: 8px;
		right: 8px;
		margin-top: 4px;
		padding: 4px;
		background: var(--color-surface);
		border: 1px solid var(--color-border-subtle);
		border-radius: 6px;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
		z-index: 100;
	}

	.account-option {
		display: flex;
		align-items: center;
		gap: 8px;
		width: 100%;
		padding: 6px 10px;
		border: none;
		border-radius: 4px;
		background: transparent;
		color: var(--color-text-muted);
		cursor: pointer;
		font-size: 12px;
		font-weight: 500;
		transition: background-color 0.15s ease;
	}

	.account-option:hover {
		background-color: var(--color-surface-hover);
		color: var(--color-text);
	}

	.account-option.active {
		background-color: var(--color-surface-active);
		color: var(--color-text);
	}
</style>
