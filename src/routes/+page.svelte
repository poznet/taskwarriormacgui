<script lang="ts">
	import '../app.css';
	import { onMount, onDestroy } from 'svelte';
	import { startPolling, stopPolling, setPollInterval, tasks } from '$lib/stores/tasks';
	import { fetchProjects } from '$lib/stores/projects';
	import { settings, applySettings, initWindowSize } from '$lib/stores/settings';
	import { checkDueNotifications } from '$lib/notifications';
	import TitleBar from '$lib/components/TitleBar.svelte';
	import ProjectFilter from '$lib/components/ProjectFilter.svelte';
	import AddTask from '$lib/components/AddTask.svelte';
	import TaskList from '$lib/components/TaskList.svelte';
	import KanbanView from '$lib/components/KanbanView.svelte';
	import SearchBar from '$lib/components/SearchBar.svelte';
	import ProgressBar from '$lib/components/ProgressBar.svelte';
	import PomodoroTimer from '$lib/components/PomodoroTimer.svelte';
	import StatusBar from '$lib/components/StatusBar.svelte';

	let unsub: (() => void)[] = [];

	onMount(() => {
		initWindowSize();
		applySettings($settings);
		startPolling($settings.pollInterval);
		fetchProjects();

		unsub.push(settings.subscribe((s) => {
			setPollInterval(s.pollInterval);
		}));

		// Due date notifications
		unsub.push(tasks.subscribe(($tasks) => {
			if ($settings.notifyDue) {
				checkDueNotifications($tasks);
			}
		}));
	});

	onDestroy(() => {
		stopPolling();
		unsub.forEach((u) => u());
	});
</script>

<div class="taskfloat-window">
	<TitleBar />
	<SearchBar />
	<ProjectFilter />
	<AddTask />
	<ProgressBar />
	{#if $settings.viewMode === 'kanban'}
		<KanbanView />
	{:else}
		<TaskList />
	{/if}
	{#if $settings.showPomodoro}
		<PomodoroTimer />
	{/if}
	<StatusBar />
</div>
