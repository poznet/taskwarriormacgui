import { writable, derived } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import type { Task } from '$lib/types/task';

export const tasks = writable<Task[]>([]);
export const error = writable<string | null>(null);
export const loading = writable(true);
export const activeProject = writable<string | null>(null);
export const searchQuery = writable('');

let pollTimer: ReturnType<typeof setInterval> | null = null;
let currentInterval = 15000;

export const filteredTasks = derived(
	[tasks, activeProject, searchQuery],
	([$tasks, $activeProject, $searchQuery]) => {
		let result = $tasks;
		if ($activeProject) {
			result = result.filter((t) => t.project === $activeProject);
		}
		if ($searchQuery.trim()) {
			const q = $searchQuery.toLowerCase();
			result = result.filter(
				(t) =>
					t.description.toLowerCase().includes(q) ||
					(t.project && t.project.toLowerCase().includes(q)) ||
					(t.tags && t.tags.some((tag) => tag.toLowerCase().includes(q)))
			);
		}
		return result;
	}
);

export const pendingTasks = derived(filteredTasks, ($filtered) =>
	$filtered
		.filter((t) => t.status === 'pending')
		.sort((a, b) => b.urgency - a.urgency)
);

export const completedTodayTasks = derived(filteredTasks, ($filtered) => {
	const todayStr = new Date().toISOString().slice(0, 8);
	return $filtered
		.filter(
			(t) =>
				t.status === 'completed' &&
				t.end &&
				t.end.slice(0, 8) === todayStr
		)
		.sort((a, b) => (b.end ?? '').localeCompare(a.end ?? ''));
});

export async function fetchTasks(): Promise<void> {
	try {
		const filter = '(status:pending or (status:completed and end:today))';
		const result = await invoke<Task[]>('get_tasks', { filter });
		tasks.set(result);
		error.set(null);
	} catch (e) {
		error.set(String(e));
	} finally {
		loading.set(false);
	}
}

function clearPoll() {
	if (pollTimer) {
		clearInterval(pollTimer);
		pollTimer = null;
	}
}

function startPoll() {
	clearPoll();
	fetchTasks();
	pollTimer = setInterval(fetchTasks, currentInterval);
}

export function startPolling(interval?: number): void {
	if (interval) currentInterval = interval;
	startPoll();

	document.addEventListener('visibilitychange', () => {
		if (document.hidden) {
			clearPoll();
		} else {
			startPoll();
		}
	});
}

export function setPollInterval(interval: number): void {
	currentInterval = interval;
	if (pollTimer) startPoll(); // restart with new interval
}

export function stopPolling(): void {
	clearPoll();
}

export async function completeTask(uuid: string): Promise<void> {
	tasks.update((list) =>
		list.map((t) =>
			t.uuid === uuid
				? { ...t, status: 'completed' as const, end: formatTwDate(new Date()) }
				: t
		)
	);

	try {
		await invoke('complete_task', { uuid });
	} catch (e) {
		await fetchTasks();
		error.set(`Nie udalo sie ukonczyc taska: ${e}`);
	}
}

export async function uncompleteTask(uuid: string): Promise<void> {
	tasks.update((list) =>
		list.map((t) =>
			t.uuid === uuid
				? { ...t, status: 'pending' as const, end: undefined }
				: t
		)
	);

	try {
		await invoke('uncomplete_task', { uuid });
	} catch (e) {
		await fetchTasks();
		error.set(`Nie udalo sie przywrocic taska: ${e}`);
	}
}

export async function addTask(
	description: string,
	project?: string,
	priority?: string,
	due?: string
): Promise<void> {
	try {
		await invoke('add_task', {
			description,
			project: project || undefined,
			priority: priority || undefined,
			due: due || undefined
		});
		await fetchTasks();
	} catch (e) {
		error.set(`Nie udalo sie dodac taska: ${e}`);
	}
}

export async function modifyTask(uuid: string, modifications: Record<string, string>): Promise<void> {
	try {
		await invoke('modify_task', { uuid, modifications });
		await fetchTasks();
	} catch (e) {
		error.set(`Nie udalo sie zmodyfikowac taska: ${e}`);
	}
}

export async function deleteTask(uuid: string): Promise<void> {
	tasks.update((list) => list.filter((t) => t.uuid !== uuid));

	try {
		await invoke('delete_task', { uuid });
	} catch (e) {
		await fetchTasks();
		error.set(`Nie udalo sie usunac taska: ${e}`);
	}
}

function formatTwDate(date: Date): string {
	return date.toISOString().replace(/[-:]/g, '').replace(/\.\d+/, '');
}
