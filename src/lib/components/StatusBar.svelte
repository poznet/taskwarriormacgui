<script lang="ts">
	import { pendingTasks, completedTodayTasks } from '$lib/stores/tasks';
	import { WebviewWindow } from '@tauri-apps/api/webviewWindow';

	async function openSettings() {
		// Check if settings window already exists
		const existing = await WebviewWindow.getByLabel('settings');
		if (existing) {
			await existing.show();
			await existing.setFocus();
			return;
		}

		new WebviewWindow('settings', {
			url: '/settings',
			title: 'Ustawienia',
			width: 340,
			height: 520,
			minWidth: 300,
			minHeight: 400,
			resizable: true,
			decorations: false,
			transparent: true,
			alwaysOnTop: true,
			center: true,
		});
	}
</script>

<footer>
	<span class="stats">
		{$pendingTasks.length} otwartych
		{#if $completedTodayTasks.length > 0}
			&middot; {$completedTodayTasks.length} ukoncz. dzis
		{/if}
	</span>
	<button class="gear" onclick={openSettings} title="Ustawienia">
		<svg width="16" height="16" viewBox="0 0 16 16" fill="none">
			<path d="M6.5 1.5h3l.4 1.6.9.4 1.5-.7 2.1 2.1-.7 1.5.4.9 1.6.4v3l-1.6.4-.4.9.7 1.5-2.1 2.1-1.5-.7-.9.4-.4 1.6h-3l-.4-1.6-.9-.4-1.5.7-2.1-2.1.7-1.5-.4-.9L1.5 9.5v-3l1.6-.4.4-.9-.7-1.5 2.1-2.1 1.5.7.9-.4.4-1.6z" stroke="currentColor" stroke-width="1.1" stroke-linejoin="round"/>
			<circle cx="8" cy="8" r="2" stroke="currentColor" stroke-width="1.1"/>
		</svg>
	</button>
</footer>

<style>
	footer {
		display: flex;
		align-items: center;
		justify-content: space-between;
		height: 32px;
		padding: 0 12px;
		border-top: 1px solid var(--border);
		flex-shrink: 0;
	}

	.stats {
		font-size: 11px;
		color: var(--text-muted);
	}

	.gear {
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0;
		color: var(--text-muted);
		border-radius: 6px;
	}

	.gear:hover {
		color: var(--text-primary);
	}
</style>
