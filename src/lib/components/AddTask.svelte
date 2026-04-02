<script lang="ts">
	import { addTask, activeProject } from '$lib/stores/tasks';

	let description = $state('');
	let isSubmitting = $state(false);

	/**
	 * Quick-add syntax parser:
	 * "Opis taska p:projekt +H due:jutro #tag1 #tag2"
	 * p: or project: → project
	 * +H / +M / +L → priority
	 * due: or d: → due date (TW parsuje: tomorrow, friday, 2026-04-15, etc.)
	 * #tag → tag
	 */
	function parseQuickAdd(input: string): {
		description: string;
		project?: string;
		priority?: string;
		due?: string;
	} {
		let desc = input;
		let project: string | undefined;
		let priority: string | undefined;
		let due: string | undefined;

		// Extract project: p:xxx or project:xxx
		const projMatch = desc.match(/\b(?:p|project):(\S+)/i);
		if (projMatch) {
			project = projMatch[1];
			desc = desc.replace(projMatch[0], '');
		}

		// Extract priority: +H, +M, +L (standalone)
		const priMatch = desc.match(/(?:^|\s)\+([HML])(?:\s|$)/i);
		if (priMatch) {
			priority = priMatch[1].toUpperCase();
			desc = desc.replace(priMatch[0], ' ');
		}

		// Extract due: due:xxx or d:xxx
		const dueMatch = desc.match(/\b(?:due|d):(\S+)/i);
		if (dueMatch) {
			due = dueMatch[1];
			desc = desc.replace(dueMatch[0], '');
		}

		desc = desc.replace(/\s+/g, ' ').trim();

		return { description: desc, project, priority, due };
	}

	async function handleSubmit() {
		const text = description.trim();
		if (!text || isSubmitting) return;

		isSubmitting = true;
		description = '';

		const parsed = parseQuickAdd(text);

		await addTask(
			parsed.description,
			parsed.project ?? $activeProject ?? undefined,
			parsed.priority,
			parsed.due
		);
		isSubmitting = false;
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' && !e.shiftKey) {
			e.preventDefault();
			handleSubmit();
		}
		if (e.key === 'Escape') {
			description = '';
		}
	}
</script>

<div class="add-task">
	<input
		type="text"
		bind:value={description}
		onkeydown={handleKeydown}
		placeholder="Nowy task... (p:projekt +H due:jutro)"
		disabled={isSubmitting}
	/>
</div>

<style>
	.add-task {
		padding: 8px 12px;
		border-bottom: 1px solid var(--border);
		flex-shrink: 0;
	}

	.add-task input {
		width: 100%;
		padding: 8px 10px;
	}
</style>
