<script lang="ts">
	import { pendingTasks, completedTodayTasks } from '$lib/stores/tasks';

	let total = $derived($pendingTasks.length + $completedTodayTasks.length);
	let done = $derived($completedTodayTasks.length);
	let percent = $derived(total > 0 ? Math.round((done / total) * 100) : 0);
</script>

{#if done > 0}
	<div class="progress-wrap">
		<div class="progress-track">
			<div class="progress-fill" style="width: {percent}%"></div>
		</div>
		<span class="progress-label">{done}/{total} dzis ({percent}%)</span>
	</div>
{/if}

<style>
	.progress-wrap {
		padding: 4px 12px 6px;
		display: flex;
		align-items: center;
		gap: 8px;
		flex-shrink: 0;
	}

	.progress-track {
		flex: 1;
		height: 3px;
		background: var(--bg-hover);
		border-radius: 2px;
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background: var(--accent);
		border-radius: 2px;
		transition: width 0.4s ease-out;
	}

	.progress-label {
		font-size: 10px;
		color: var(--text-muted);
		white-space: nowrap;
	}
</style>
