<script lang="ts">
	import { searchQuery } from '$lib/stores/tasks';

	let visible = $state(false);
	let inputEl: HTMLInputElement;

	function handleGlobalKeydown(e: KeyboardEvent) {
		if ((e.metaKey || e.ctrlKey) && e.key === 'f') {
			e.preventDefault();
			visible = !visible;
			if (visible) {
				setTimeout(() => inputEl?.focus(), 50);
			} else {
				searchQuery.set('');
			}
		}
		if (e.key === 'Escape' && visible) {
			visible = false;
			searchQuery.set('');
		}
	}
</script>

<svelte:window onkeydown={handleGlobalKeydown} />

{#if visible}
	<div class="search-bar">
		<svg width="14" height="14" viewBox="0 0 14 14" class="search-icon">
			<circle cx="6" cy="6" r="4.5" stroke="currentColor" stroke-width="1.2" fill="none"/>
			<path d="M9.5 9.5l3 3" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>
		</svg>
		<input
			bind:this={inputEl}
			type="text"
			value={$searchQuery}
			oninput={(e) => searchQuery.set(e.currentTarget.value)}
			onkeydown={(e) => { if (e.key === 'Escape') { visible = false; searchQuery.set(''); }}}
			placeholder="Szukaj..."
		/>
		<button class="clear-btn" onclick={() => { searchQuery.set(''); visible = false; }}>
			<svg width="12" height="12" viewBox="0 0 12 12">
				<path d="M3 3l6 6M9 3l-6 6" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>
			</svg>
		</button>
	</div>
{/if}

<style>
	.search-bar {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 6px 12px;
		border-bottom: 1px solid var(--border);
		background: var(--bg-secondary);
		flex-shrink: 0;
		animation: slideDown 0.15s ease-out;
	}

	@keyframes slideDown {
		from { opacity: 0; transform: translateY(-8px); }
		to { opacity: 1; transform: translateY(0); }
	}

	.search-icon {
		color: var(--text-muted);
		flex-shrink: 0;
	}

	.search-bar input {
		flex: 1;
		background: transparent;
		border: none;
		padding: 4px 0;
		font-size: 12px;
	}

	.search-bar input:focus {
		border: none;
		outline: none;
	}

	.clear-btn {
		width: 20px;
		height: 20px;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0;
		color: var(--text-muted);
		border-radius: 4px;
	}
</style>
