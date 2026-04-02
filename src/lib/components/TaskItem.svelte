<script lang="ts">
	import type { Task } from '$lib/types/task';
	import { completeTask, uncompleteTask, deleteTask, modifyTask, fetchTasks } from '$lib/stores/tasks';
	import { invoke } from '@tauri-apps/api/core';

	let { task }: { task: Task } = $props();

	let isCompleted = $derived(task.status === 'completed');
	let showActions = $state(false);
	let editing = $state(false);
	let editDesc = $state('');
	let editProject = $state('');
	let editPriority = $state('');
	let editDue = $state('');
	let showAnnotations = $state(false);
	let newAnnotation = $state('');
	let completing = $state(false);

	// Urgency heatmap: map 0-20 urgency to opacity of accent border
	let urgencyIntensity = $derived(Math.min(1, task.urgency / 15));

	function priorityColor(p?: string): string {
		switch (p) {
			case 'H': return 'var(--priority-high)';
			case 'M': return 'var(--priority-medium)';
			case 'L': return 'var(--priority-low)';
			default: return 'var(--priority-none)';
		}
	}

	function formatDue(due?: string): string | null {
		if (!due) return null;
		try {
			const year = parseInt(due.slice(0, 4));
			const month = parseInt(due.slice(4, 6)) - 1;
			const day = parseInt(due.slice(6, 8));
			const dueDate = new Date(year, month, day);
			const now = new Date();
			const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());
			const diffDays = Math.round((dueDate.getTime() - today.getTime()) / (1000 * 60 * 60 * 24));
			if (diffDays < 0) return `${Math.abs(diffDays)} dni temu`;
			if (diffDays === 0) return 'dzis';
			if (diffDays === 1) return 'jutro';
			if (diffDays <= 7) return `za ${diffDays} dni`;
			return dueDate.toLocaleDateString('pl-PL', { day: 'numeric', month: 'short' });
		} catch { return null; }
	}

	function formatEndTime(end?: string): string | null {
		if (!end) return null;
		try { return `${end.slice(9, 11)}:${end.slice(11, 13)}`; }
		catch { return null; }
	}

	function dueClass(due?: string): string {
		if (!due) return '';
		const year = parseInt(due.slice(0, 4));
		const month = parseInt(due.slice(4, 6)) - 1;
		const day = parseInt(due.slice(6, 8));
		const dueDate = new Date(year, month, day);
		const now = new Date();
		const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());
		const diff = Math.round((dueDate.getTime() - today.getTime()) / 86400000);
		if (diff < 0) return 'overdue';
		if (diff === 0) return 'today';
		return '';
	}

	async function handleToggle() {
		completing = true;
		if (isCompleted) {
			await uncompleteTask(task.uuid);
		} else {
			await completeTask(task.uuid);
		}
		setTimeout(() => completing = false, 400);
	}

	function startEdit() {
		if (isCompleted) return;
		editing = true;
		editDesc = task.description;
		editProject = task.project ?? '';
		editPriority = task.priority ?? '';
		editDue = task.due ? formatDueForInput(task.due) : '';
	}

	function formatDueForInput(tw: string): string {
		return `${tw.slice(0, 4)}-${tw.slice(4, 6)}-${tw.slice(6, 8)}`;
	}

	async function saveEdit() {
		const mods: Record<string, string> = {};
		if (editDesc !== task.description) mods.description = editDesc;
		if (editProject !== (task.project ?? '')) mods.project = editProject || '';
		if (editPriority !== (task.priority ?? '')) mods.priority = editPriority || '';
		if (editDue) {
			const twDue = editDue.replace(/-/g, '') + 'T000000Z';
			if (twDue !== task.due) mods.due = editDue;
		} else if (task.due) {
			mods.due = '';
		}

		if (Object.keys(mods).length > 0) {
			await modifyTask(task.uuid, mods);
		}
		editing = false;
	}

	function cancelEdit() {
		editing = false;
	}

	function handleEditKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') { e.preventDefault(); saveEdit(); }
		if (e.key === 'Escape') cancelEdit();
	}

	async function addAnnotation() {
		if (!newAnnotation.trim()) return;
		try {
			await invoke('annotate_task', {
				uuid: task.uuid,
				text: newAnnotation.trim()
			});
			newAnnotation = '';
			await fetchTasks();
		} catch {}
	}

	function handleAnnotationKey(e: KeyboardEvent) {
		if (e.key === 'Enter') { e.preventDefault(); addAnnotation(); }
		if (e.key === 'Escape') { newAnnotation = ''; showAnnotations = false; }
	}
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
	class="task-item"
	class:completed={isCompleted}
	class:completing
	style="--urgency-glow: {urgencyIntensity}"
	onmouseenter={() => showActions = true}
	onmouseleave={() => showActions = false}
	ondblclick={startEdit}
>
	<button class="checkbox" onclick={handleToggle} title={isCompleted ? 'Przywroc' : 'Ukoncz'}>
		{#if isCompleted}
			<svg width="16" height="16" viewBox="0 0 16 16" class="check-icon">
				<circle cx="8" cy="8" r="7" fill="var(--accent)" stroke="none" />
				<path d="M5 8l2 2 4-4" stroke="white" stroke-width="1.5" fill="none" stroke-linecap="round" stroke-linejoin="round" />
			</svg>
		{:else}
			<svg width="16" height="16" viewBox="0 0 16 16" class="circle-icon">
				<circle cx="8" cy="8" r="7" fill="none" stroke="var(--text-muted)" stroke-width="1.2" />
			</svg>
		{/if}
	</button>

	<div class="content">
		{#if editing}
			<div class="edit-form">
				<input class="edit-desc" type="text" bind:value={editDesc} onkeydown={handleEditKeydown} autofocus />
				<div class="edit-row">
					<input class="edit-field" type="text" bind:value={editProject} placeholder="projekt" onkeydown={handleEditKeydown} />
					<select class="edit-field" bind:value={editPriority}>
						<option value="">-</option>
						<option value="H">Wysoki</option>
						<option value="M">Sredni</option>
						<option value="L">Niski</option>
					</select>
					<input class="edit-field" type="date" bind:value={editDue} onkeydown={handleEditKeydown} />
				</div>
				<div class="edit-actions">
					<button class="save-btn" onclick={saveEdit}>Zapisz</button>
					<button class="cancel-btn" onclick={cancelEdit}>Anuluj</button>
				</div>
			</div>
		{:else}
			<div class="description-row">
				<span class="priority-dot" style="background: {priorityColor(task.priority)}"></span>
				<span class="description">{task.description}</span>
			</div>
			<div class="meta">
				{#if task.project}
					<span class="project">{task.project}</span>
				{/if}
				{#if task.tags && task.tags.length > 0}
					{#each task.tags as tag}
						<span class="tag">#{tag}</span>
					{/each}
				{/if}
				{#if !isCompleted && formatDue(task.due)}
					<span class="due {dueClass(task.due)}">{formatDue(task.due)}</span>
				{/if}
				{#if isCompleted && formatEndTime(task.end)}
					<span class="end-time">{formatEndTime(task.end)}</span>
				{/if}
			</div>

			<!-- Annotations -->
			{#if task.annotations && task.annotations.length > 0}
				<button class="annotations-toggle" onclick={() => showAnnotations = !showAnnotations}>
					{task.annotations.length} notat{task.annotations.length === 1 ? 'ka' : task.annotations.length < 5 ? 'ki' : 'ek'}
					{showAnnotations ? '▾' : '▸'}
				</button>
			{/if}

			{#if showAnnotations}
				<div class="annotations">
					{#each task.annotations ?? [] as ann}
						<div class="annotation">{ann.description}</div>
					{/each}
					{#if !isCompleted}
						<input
							class="annotation-input"
							type="text"
							bind:value={newAnnotation}
							onkeydown={handleAnnotationKey}
							placeholder="Dodaj notatke..."
						/>
					{/if}
				</div>
			{/if}
		{/if}
	</div>

	{#if showActions && !isCompleted && !editing}
		<div class="action-buttons">
			<button class="action-btn" onclick={startEdit} title="Edytuj">
				<svg width="14" height="14" viewBox="0 0 14 14">
					<path d="M10.5 2.5l1 1-7 7H3.5v-1l7-7z" stroke="currentColor" stroke-width="1" fill="none" stroke-linecap="round" stroke-linejoin="round"/>
				</svg>
			</button>
			{#if !task.annotations?.length}
				<button class="action-btn" onclick={() => showAnnotations = !showAnnotations} title="Notatka">
					<svg width="14" height="14" viewBox="0 0 14 14">
						<path d="M3 3h8v7l-2 1H3z" stroke="currentColor" stroke-width="1" fill="none" stroke-linejoin="round"/>
						<path d="M5 6h4M5 8h2" stroke="currentColor" stroke-width="0.8" stroke-linecap="round"/>
					</svg>
				</button>
			{/if}
			<button class="action-btn delete" onclick={() => deleteTask(task.uuid)} title="Usun">
				<svg width="14" height="14" viewBox="0 0 14 14">
					<path d="M4 4l6 6M10 4l-6 6" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" />
				</svg>
			</button>
		</div>
	{/if}
</div>

<style>
	.task-item {
		display: flex;
		align-items: flex-start;
		gap: 10px;
		padding: 8px 12px;
		border-radius: 6px;
		transition: background 0.15s, opacity 0.3s, transform 0.3s;
		border-left: 2px solid transparent;
		border-left-color: color-mix(in srgb, var(--accent) calc(var(--urgency-glow) * 100%), transparent);
		animation: slideIn 0.25s ease-out;
	}

	@keyframes slideIn {
		from { opacity: 0; transform: translateY(-6px); }
		to { opacity: 1; transform: translateY(0); }
	}

	.task-item:hover {
		background: var(--bg-hover);
	}

	.task-item.completed {
		opacity: 0.4;
	}

	.task-item.completing {
		animation: completeAnim 0.4s ease-out;
	}

	@keyframes completeAnim {
		0% { transform: scale(1); }
		30% { transform: scale(0.97); }
		100% { transform: scale(1); }
	}

	/* Checkbox */
	.checkbox {
		flex-shrink: 0;
		width: 20px;
		height: 20px;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0;
		margin-top: 1px;
		border-radius: 50%;
		background: none;
		transition: transform 0.15s;
	}

	.checkbox:hover {
		background: var(--bg-active);
		transform: scale(1.15);
	}

	.check-icon {
		animation: checkPop 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
	}

	@keyframes checkPop {
		0% { transform: scale(0.5); opacity: 0; }
		100% { transform: scale(1); opacity: 1; }
	}

	/* Content */
	.content {
		flex: 1;
		min-width: 0;
	}

	.description-row {
		display: flex;
		align-items: center;
		gap: 6px;
	}

	.priority-dot {
		width: 6px;
		height: 6px;
		border-radius: 50%;
		flex-shrink: 0;
	}

	.description {
		line-height: 1.4;
		word-break: break-word;
		transition: opacity 0.3s;
	}

	.completed .description {
		text-decoration: line-through;
	}

	.meta {
		display: flex;
		flex-wrap: wrap;
		gap: 6px;
		margin-top: 2px;
		font-size: 11px;
		color: var(--text-secondary);
	}

	.meta .project { color: var(--text-muted); }
	.meta .due.overdue { color: var(--priority-high); }
	.meta .due.today { color: var(--accent); font-weight: 600; }
	.meta .tag {
		color: var(--accent);
		opacity: 0.7;
		font-size: 10px;
	}

	/* Annotations */
	.annotations-toggle {
		font-size: 10px;
		color: var(--text-muted);
		margin-top: 4px;
		padding: 1px 4px;
		border-radius: 3px;
		cursor: pointer;
	}

	.annotations-toggle:hover {
		color: var(--text-secondary);
	}

	.annotations {
		margin-top: 4px;
		padding-left: 2px;
		border-left: 1px solid var(--border);
		display: flex;
		flex-direction: column;
		gap: 3px;
	}

	.annotation {
		font-size: 11px;
		color: var(--text-secondary);
		padding: 2px 8px;
		line-height: 1.4;
	}

	.annotation-input {
		font-size: 11px;
		padding: 3px 8px;
		margin-top: 2px;
		background: transparent;
		border: 1px solid var(--border);
		border-radius: 4px;
	}

	/* Edit form */
	.edit-form {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.edit-desc {
		width: 100%;
		font-size: 13px;
	}

	.edit-row {
		display: flex;
		gap: 4px;
	}

	.edit-field {
		flex: 1;
		min-width: 0;
		font-size: 11px;
		padding: 4px 6px;
	}

	select.edit-field {
		background: var(--bg-secondary);
		border: 1px solid var(--border);
		border-radius: 6px;
		color: var(--text-primary);
		font-size: 11px;
	}

	.edit-actions {
		display: flex;
		gap: 4px;
	}

	.save-btn {
		font-size: 11px;
		padding: 3px 10px;
		background: var(--accent);
		color: white;
		border-radius: 4px;
	}

	.save-btn:hover { background: var(--accent-hover); }

	.cancel-btn {
		font-size: 11px;
		padding: 3px 10px;
		color: var(--text-muted);
	}

	/* Action buttons */
	.action-buttons {
		display: flex;
		gap: 2px;
		flex-shrink: 0;
	}

	.action-btn {
		width: 22px;
		height: 22px;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0;
		color: var(--text-muted);
		border-radius: 4px;
	}

	.action-btn:hover { color: var(--text-primary); background: var(--bg-active); }
	.action-btn.delete:hover { color: var(--priority-high); background: rgba(255,69,58,0.15); }
</style>
