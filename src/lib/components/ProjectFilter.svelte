<script lang="ts">
	import { projects } from '$lib/stores/projects';
	import { activeProject } from '$lib/stores/tasks';

	function setProject(p: string | null) {
		activeProject.set(p);
	}
</script>

<div class="project-filter">
	<button
		class="pill"
		class:active={$activeProject === null}
		onclick={() => setProject(null)}
	>
		Wszystkie
	</button>
	{#each $projects as project}
		<button
			class="pill"
			class:active={$activeProject === project}
			onclick={() => setProject(project)}
		>
			{project}
		</button>
	{/each}
</div>

<style>
	.project-filter {
		display: flex;
		gap: 4px;
		padding: 8px 12px;
		border-bottom: 1px solid var(--border);
		overflow-x: auto;
		flex-shrink: 0;
		scrollbar-width: none;
	}

	.project-filter::-webkit-scrollbar {
		display: none;
	}

	.pill {
		flex-shrink: 0;
		padding: 4px 10px;
		border-radius: 12px;
		font-size: 11px;
		color: var(--text-secondary);
		background: var(--bg-secondary);
		transition: all 0.15s;
		white-space: nowrap;
	}

	.pill:hover {
		background: var(--bg-hover);
		color: var(--text-primary);
	}

	.pill.active {
		background: var(--accent);
		color: white;
	}
</style>
