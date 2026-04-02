<script lang="ts">
	import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/plugin-notification';

	let timeLeft = $state(25 * 60); // 25 min
	let isRunning = $state(false);
	let isBreak = $state(false);
	let interval: ReturnType<typeof setInterval> | null = null;
	let activeTaskDesc = $state('');

	const WORK_TIME = 25 * 60;
	const BREAK_TIME = 5 * 60;

	let { taskDescription = '' }: { taskDescription?: string } = $props();

	$effect(() => {
		activeTaskDesc = taskDescription;
	});

	function formatTime(seconds: number): string {
		const m = Math.floor(seconds / 60).toString().padStart(2, '0');
		const s = (seconds % 60).toString().padStart(2, '0');
		return `${m}:${s}`;
	}

	let progress = $derived(1 - timeLeft / (isBreak ? BREAK_TIME : WORK_TIME));

	function start() {
		isRunning = true;
		interval = setInterval(() => {
			timeLeft--;
			if (timeLeft <= 0) {
				stop();
				if (isBreak) {
					isBreak = false;
					timeLeft = WORK_TIME;
					notify('Przerwa skonczona!', 'Czas wrocic do pracy.');
				} else {
					isBreak = true;
					timeLeft = BREAK_TIME;
					notify('Pomodoro ukonczony!', activeTaskDesc || 'Czas na przerwe.');
				}
			}
		}, 1000);
	}

	function stop() {
		isRunning = false;
		if (interval) clearInterval(interval);
		interval = null;
	}

	function reset() {
		stop();
		isBreak = false;
		timeLeft = WORK_TIME;
	}

	function toggle() {
		if (isRunning) stop();
		else start();
	}

	async function notify(title: string, body: string) {
		try {
			let granted = await isPermissionGranted();
			if (!granted) {
				const permission = await requestPermission();
				granted = permission === 'granted';
			}
			if (granted) {
				sendNotification({ title, body });
			}
		} catch {}
	}
</script>

<div class="pomodoro" class:is-break={isBreak}>
	<div class="timer-ring">
		<svg width="48" height="48" viewBox="0 0 48 48">
			<circle cx="24" cy="24" r="20" fill="none" stroke="var(--bg-hover)" stroke-width="3" />
			<circle
				cx="24" cy="24" r="20" fill="none"
				stroke={isBreak ? 'var(--priority-low)' : 'var(--accent)'}
				stroke-width="3"
				stroke-dasharray={2 * Math.PI * 20}
				stroke-dashoffset={2 * Math.PI * 20 * (1 - progress)}
				stroke-linecap="round"
				transform="rotate(-90 24 24)"
				style="transition: stroke-dashoffset 1s linear"
			/>
		</svg>
		<span class="time">{formatTime(timeLeft)}</span>
	</div>

	<div class="controls">
		<button onclick={toggle} class="toggle-btn">
			{isRunning ? 'Pauza' : (timeLeft < (isBreak ? BREAK_TIME : WORK_TIME) ? 'Wznow' : 'Start')}
		</button>
		{#if timeLeft < (isBreak ? BREAK_TIME : WORK_TIME)}
			<button onclick={reset} class="reset-btn">Reset</button>
		{/if}
	</div>

	<span class="pomodoro-label">{isBreak ? 'Przerwa' : 'Focus'}</span>
</div>

<style>
	.pomodoro {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 8px 12px;
		border-top: 1px solid var(--border);
		flex-shrink: 0;
	}

	.timer-ring {
		position: relative;
		width: 48px;
		height: 48px;
		flex-shrink: 0;
	}

	.time {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		font-size: 11px;
		font-weight: 600;
		color: var(--text-primary);
		font-variant-numeric: tabular-nums;
	}

	.controls {
		display: flex;
		gap: 4px;
	}

	.toggle-btn {
		font-size: 11px;
		padding: 4px 10px;
		background: var(--accent);
		color: white;
		border-radius: 6px;
	}

	.toggle-btn:hover {
		background: var(--accent-hover);
	}

	.is-break .toggle-btn {
		background: var(--priority-low);
	}

	.reset-btn {
		font-size: 11px;
		padding: 4px 8px;
		color: var(--text-muted);
	}

	.pomodoro-label {
		font-size: 10px;
		color: var(--text-muted);
		margin-left: auto;
	}
</style>
