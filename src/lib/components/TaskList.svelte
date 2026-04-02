<script lang="ts">
	import { pendingTasks, completedTodayTasks, loading, error } from '$lib/stores/tasks';
	import { settings } from '$lib/stores/settings';
	import TaskItem from './TaskItem.svelte';
</script>

<div class="task-list">
	{#if $loading}
		<div class="state">
			<p class="muted">Ladowanie...</p>
		</div>
	{:else if $error}
		<div class="state error">
			<p>Blad polaczenia z Taskwarrior</p>
			<p class="detail">{$error}</p>
		</div>
	{:else if $pendingTasks.length === 0 && $completedTodayTasks.length === 0}
		<div class="state">
			<p class="empty-icon">&#127881;</p>
			<p class="muted">Brak taskow</p>
		</div>
	{:else}
		{#each $pendingTasks as task (task.uuid)}
			<TaskItem {task} />
		{/each}

		{#if $settings.showCompleted && $completedTodayTasks.length > 0}
			<div class="separator">
				<span>Ukonczone dzis</span>
			</div>
			{#each $completedTodayTasks.slice(0, 5) as task (task.uuid)}
				<TaskItem {task} />
			{/each}
			{#if $completedTodayTasks.length > 5}
				<p class="more-done">+{$completedTodayTasks.length - 5} wiecej</p>
			{/if}
		{/if}
	{/if}
</div>

<style>
	.task-list {
		flex: 1;
		overflow-y: auto;
		padding: 4px 0;
	}

	.state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 40px 20px;
		gap: 4px;
	}

	.state .muted {
		color: var(--text-muted);
		font-size: 13px;
	}

	.state.error p {
		color: var(--priority-high);
		font-size: 13px;
	}

	.state.error .detail {
		color: var(--text-muted);
		font-size: 11px;
		word-break: break-word;
		text-align: center;
		max-width: 280px;
	}

	.empty-icon {
		font-size: 32px;
		line-height: 1;
	}

	.separator {
		display: flex;
		align-items: center;
		padding: 12px 16px 6px;
		gap: 8px;
	}

	.separator span {
		font-size: 11px;
		color: var(--text-muted);
		text-transform: uppercase;
		letter-spacing: 0.5px;
		white-space: nowrap;
	}

	.separator::after {
		content: '';
		flex: 1;
		height: 1px;
		background: var(--border);
	}

	.more-done {
		text-align: center;
		font-size: 11px;
		color: var(--text-muted);
		padding: 4px;
	}
</style>
