<script lang="ts">
	import { tasks } from '$lib/stores/tasks';
	import { projects } from '$lib/stores/projects';
	import TaskItem from './TaskItem.svelte';
	import type { Task } from '$lib/types/task';

	let tasksByProject = $derived(groupByProject($tasks));

	function groupByProject(allTasks: Task[]): Map<string, Task[]> {
		const map = new Map<string, Task[]>();
		const pending = allTasks.filter(t => t.status === 'pending');

		// "Bez projektu" column
		const noProject = pending.filter(t => !t.project);
		if (noProject.length > 0) {
			map.set('(bez projektu)', noProject.sort((a, b) => b.urgency - a.urgency));
		}

		// Per-project columns
		for (const proj of $projects) {
			const projectTasks = pending.filter(t => t.project === proj);
			if (projectTasks.length > 0) {
				map.set(proj, projectTasks.sort((a, b) => b.urgency - a.urgency));
			}
		}

		return map;
	}
</script>

<div class="kanban-wrapper">
	<div class="kanban">
		{#each [...tasksByProject.entries()] as [project, projectTasks]}
			<div class="kanban-column">
				<div class="column-header">
					<span class="column-title">{project}</span>
					<span class="column-count">{projectTasks.length}</span>
				</div>
				<div class="column-tasks">
					{#each projectTasks as task (task.uuid)}
						<TaskItem {task} />
					{/each}
				</div>
			</div>
		{/each}

		{#if tasksByProject.size === 0}
			<div class="empty">Brak taskow do wyswietlenia</div>
		{/if}
	</div>
</div>

<style>
	.kanban-wrapper {
		flex: 1;
		min-height: 0;
		overflow: hidden;
		display: flex;
		flex-direction: column;
	}

	.kanban {
		flex: 1;
		display: flex;
		gap: 6px;
		padding: 6px;
		overflow-x: auto;
		overflow-y: hidden;
		min-height: 0;
	}

	/* Visible thin scrollbar */
	.kanban::-webkit-scrollbar {
		height: 6px;
	}

	.kanban::-webkit-scrollbar-track {
		background: rgba(255, 255, 255, 0.03);
		border-radius: 3px;
	}

	.kanban::-webkit-scrollbar-thumb {
		background: rgba(255, 255, 255, 0.15);
		border-radius: 3px;
	}

	.kanban::-webkit-scrollbar-thumb:hover {
		background: rgba(255, 255, 255, 0.25);
	}

	.kanban-column {
		flex: 0 0 260px;
		min-width: 220px;
		display: flex;
		flex-direction: column;
		background: rgba(255, 255, 255, 0.03);
		border-radius: 8px;
		overflow: hidden;
		min-height: 0;
	}

	.column-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 8px 10px;
		border-bottom: 1px solid var(--border);
		flex-shrink: 0;
	}

	.column-title {
		font-size: 11px;
		font-weight: 600;
		color: var(--text-secondary);
		text-transform: uppercase;
		letter-spacing: 0.3px;
	}

	.column-count {
		font-size: 10px;
		color: var(--text-muted);
		background: var(--bg-secondary);
		padding: 1px 6px;
		border-radius: 8px;
	}

	.column-tasks {
		flex: 1;
		overflow-y: auto;
		padding: 2px 0;
		min-height: 0;
	}

	.column-tasks::-webkit-scrollbar {
		width: 4px;
	}

	.column-tasks::-webkit-scrollbar-track {
		background: transparent;
	}

	.column-tasks::-webkit-scrollbar-thumb {
		background: rgba(255, 255, 255, 0.1);
		border-radius: 2px;
	}

	.empty {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--text-muted);
		font-size: 13px;
	}
</style>
