<script lang="ts">
	import { getCurrentWindow } from '@tauri-apps/api/window';

	const appWindow = getCurrentWindow();

	function handleDragStart(e: MouseEvent) {
		if ((e.target as HTMLElement).closest('.controls')) return;
		e.preventDefault();
		appWindow.startDragging();
	}

	function stopBubble(e: MouseEvent) {
		e.stopPropagation();
	}

	function minimize() {
		appWindow.minimize();
	}

	function close() {
		appWindow.hide();
	}
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<header onmousedown={handleDragStart}>
	<span class="title">TaskFloat</span>
	<div class="controls" onmousedown={stopBubble}>
		<button onclick={minimize} title="Minimalizuj">
			<svg width="12" height="12" viewBox="0 0 12 12">
				<rect x="2" y="5.5" width="8" height="1" rx="0.5" fill="currentColor" />
			</svg>
		</button>
		<button onclick={close} class="close" title="Zamknij">
			<svg width="12" height="12" viewBox="0 0 12 12">
				<path d="M3 3l6 6M9 3l-6 6" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" />
			</svg>
		</button>
	</div>
</header>

<style>
	header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		height: var(--titlebar-height);
		padding: 0 12px;
		background: rgba(20, 20, 20, 0.50);
		border-bottom: 1px solid var(--border);
		cursor: grab;
		flex-shrink: 0;
		-webkit-user-select: none;
		user-select: none;
	}

	header:active {
		cursor: grabbing;
	}

	.title {
		font-size: 12px;
		font-weight: 600;
		color: var(--text-secondary);
		letter-spacing: 0.3px;
		pointer-events: none;
	}

	.controls {
		display: flex;
		gap: 4px;
	}

	.controls button {
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0;
		border-radius: 6px;
		color: var(--text-muted);
	}

	.controls button:hover {
		color: var(--text-primary);
		background: var(--bg-hover);
	}

	.controls .close:hover {
		color: #FF453A;
	}
</style>
