import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export const projects = writable<string[]>([]);

export async function fetchProjects(): Promise<void> {
	try {
		const result = await invoke<string[]>('get_projects');
		projects.set(result);
	} catch {
		// Silently fail — projects are non-critical
	}
}
