<script lang="ts">
	import '../../app.css';
	import { settings, applySettings } from '$lib/stores/settings';
	import { enable, disable, isEnabled } from '@tauri-apps/plugin-autostart';
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import { onMount } from 'svelte';

	let autostartEnabled = $state(false);

	const accentPresets = [
		{ color: '#0A84FF', name: 'Niebieski' },
		{ color: '#30D158', name: 'Zielony' },
		{ color: '#BF5AF2', name: 'Fioletowy' },
		{ color: '#FF9F0A', name: 'Pomaranczowy' },
		{ color: '#FF453A', name: 'Czerwony' },
		{ color: '#AC8E68', name: 'Zloty' },
		{ color: '#98989D', name: 'Szary' },
	];

	const pollOptions = [
		{ value: 5000, label: '5s' },
		{ value: 15000, label: '15s' },
		{ value: 30000, label: '30s' },
		{ value: 60000, label: '1min' },
	];

	onMount(async () => {
		try {
			autostartEnabled = await isEnabled();
		} catch {}

		// Apply current settings to this window too
		applySettings($settings);
	});

	async function toggleAutostart() {
		try {
			if (autostartEnabled) {
				await disable();
			} else {
				await enable();
			}
			autostartEnabled = await isEnabled();
		} catch {}
	}

	function updateSetting<K extends keyof typeof $settings>(key: K, value: (typeof $settings)[K]) {
		settings.update(s => {
			const next = { ...s, [key]: value };
			applySettings(next);
			return next;
		});
	}

	function closeWindow() {
		getCurrentWindow().close();
	}

	function startDrag(e: MouseEvent) {
		if ((e.target as HTMLElement).closest('button, input, select')) return;
		getCurrentWindow().startDragging();
	}
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="settings-window" style="--accent: {$settings.accentColor}">
	<div class="settings-titlebar" onmousedown={startDrag}>
		<span>Ustawienia</span>
		<button onclick={closeWindow} class="close-btn" title="Zamknij">
			<svg width="14" height="14" viewBox="0 0 14 14">
				<path d="M4 4l6 6M10 4l-6 6" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
			</svg>
		</button>
	</div>

	<div class="settings-body">
		<!-- Opacity -->
		<div class="setting-row">
			<label>Przezroczystosc</label>
			<div class="slider-row">
				<input
					type="range"
					min="0.3"
					max="1"
					step="0.05"
					value={$settings.opacity}
					oninput={(e) => updateSetting('opacity', parseFloat(e.currentTarget.value))}
				/>
				<span class="value">{Math.round($settings.opacity * 100)}%</span>
			</div>
		</div>

		<!-- Accent color -->
		<div class="setting-row">
			<label>Kolor akcentu</label>
			<div class="color-presets">
				{#each accentPresets as preset}
					<button
						class="color-dot"
						class:active={$settings.accentColor === preset.color}
						style="background: {preset.color}"
						onclick={() => updateSetting('accentColor', preset.color)}
						title={preset.name}
					></button>
				{/each}
			</div>
		</div>

		<!-- Compact mode -->
		<div class="setting-row toggle-row">
			<label>Kompaktowy widok</label>
			<button
				class="toggle"
				class:on={$settings.compact}
				onclick={() => updateSetting('compact', !$settings.compact)}
			>
				<span class="toggle-thumb"></span>
			</button>
		</div>

		<!-- Always on top -->
		<div class="setting-row toggle-row">
			<label>Zawsze na wierzchu</label>
			<button
				class="toggle"
				class:on={$settings.alwaysOnTop}
				onclick={() => updateSetting('alwaysOnTop', !$settings.alwaysOnTop)}
			>
				<span class="toggle-thumb"></span>
			</button>
		</div>

		<!-- Show completed -->
		<div class="setting-row toggle-row">
			<label>Pokaz ukonczone dzis</label>
			<button
				class="toggle"
				class:on={$settings.showCompleted}
				onclick={() => updateSetting('showCompleted', !$settings.showCompleted)}
			>
				<span class="toggle-thumb"></span>
			</button>
		</div>

		<!-- Autostart -->
		<div class="setting-row toggle-row">
			<label>Uruchom przy starcie</label>
			<button
				class="toggle"
				class:on={autostartEnabled}
				onclick={toggleAutostart}
			>
				<span class="toggle-thumb"></span>
			</button>
		</div>

		<!-- Poll interval -->
		<div class="setting-row">
			<label>Odswiezanie</label>
			<div class="pill-group">
				{#each pollOptions as opt}
					<button
						class="poll-pill"
						class:active={$settings.pollInterval === opt.value}
						onclick={() => updateSetting('pollInterval', opt.value)}
					>
						{opt.label}
					</button>
				{/each}
			</div>
		</div>

		<!-- View mode -->
		<div class="setting-row">
			<label>Widok</label>
			<div class="pill-group">
				<button
					class="poll-pill"
					class:active={$settings.viewMode === 'list'}
					onclick={() => updateSetting('viewMode', 'list')}
				>Lista</button>
				<button
					class="poll-pill"
					class:active={$settings.viewMode === 'kanban'}
					onclick={() => updateSetting('viewMode', 'kanban')}
				>Kanban</button>
			</div>
		</div>

		<!-- Pomodoro -->
		<div class="setting-row toggle-row">
			<label>Timer Pomodoro</label>
			<button
				class="toggle"
				class:on={$settings.showPomodoro}
				onclick={() => updateSetting('showPomodoro', !$settings.showPomodoro)}
			>
				<span class="toggle-thumb"></span>
			</button>
		</div>

		<!-- Due notifications -->
		<div class="setting-row toggle-row">
			<label>Powiadomienia o terminach</label>
			<button
				class="toggle"
				class:on={$settings.notifyDue}
				onclick={() => updateSetting('notifyDue', !$settings.notifyDue)}
			>
				<span class="toggle-thumb"></span>
			</button>
		</div>

		<!-- Shortcut info -->
		<div class="setting-row info">
			<span class="shortcut-label">Skrot klawiszowy</span>
			<kbd>&#x2325;&#x2318;T</kbd>
		</div>

		<div class="setting-row info">
			<span class="shortcut-label">Szukaj</span>
			<kbd>&#x2318;F</kbd>
		</div>
	</div>
</div>

<style>
	:global(body) {
		background: transparent;
		margin: 0;
		padding: 0;
	}

	.settings-window {
		background: rgba(28, 28, 30, 0.96);
		backdrop-filter: blur(30px);
		-webkit-backdrop-filter: blur(30px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		min-height: 100vh;
		display: flex;
		flex-direction: column;
		font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Text', sans-serif;
		color: var(--text-primary);
		--text-primary: rgba(255,255,255,0.95);
		--text-secondary: rgba(255,255,255,0.55);
		--text-muted: rgba(255,255,255,0.35);
		--bg-secondary: rgba(255,255,255,0.06);
		--bg-hover: rgba(255,255,255,0.05);
		--bg-active: rgba(255,255,255,0.08);
		--border: rgba(255,255,255,0.08);
	}

	.settings-titlebar {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 12px 16px 8px;
		font-size: 13px;
		font-weight: 600;
		color: var(--text-primary);
		cursor: grab;
		user-select: none;
		-webkit-user-select: none;
	}

	.settings-titlebar:active {
		cursor: grabbing;
	}

	.close-btn {
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0;
		border-radius: 6px;
		color: var(--text-muted);
		background: none;
		border: none;
		cursor: pointer;
	}

	.close-btn:hover {
		color: var(--text-primary);
		background: var(--bg-active);
	}

	.settings-body {
		padding: 4px 16px 16px;
		display: flex;
		flex-direction: column;
		gap: 14px;
		overflow-y: auto;
		flex: 1;
	}

	.setting-row {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.setting-row label {
		font-size: 11px;
		color: var(--text-secondary);
		text-transform: uppercase;
		letter-spacing: 0.3px;
	}

	.toggle-row {
		flex-direction: row;
		align-items: center;
		justify-content: space-between;
	}

	/* Slider */
	.slider-row {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.slider-row input[type="range"] {
		flex: 1;
		-webkit-appearance: none;
		appearance: none;
		height: 4px;
		background: var(--bg-hover);
		border-radius: 2px;
		border: none;
		padding: 0;
	}

	.slider-row input[type="range"]::-webkit-slider-thumb {
		-webkit-appearance: none;
		appearance: none;
		width: 16px;
		height: 16px;
		border-radius: 50%;
		background: var(--accent);
		cursor: pointer;
		border: 2px solid rgba(255, 255, 255, 0.2);
	}

	.slider-row .value {
		font-size: 12px;
		color: var(--text-muted);
		min-width: 36px;
		text-align: right;
	}

	/* Color dots */
	.color-presets {
		display: flex;
		gap: 8px;
	}

	.color-dot {
		width: 22px;
		height: 22px;
		border-radius: 50%;
		border: 2px solid transparent;
		padding: 0;
		cursor: pointer;
		transition: border-color 0.15s, transform 0.15s;
	}

	.color-dot:hover {
		transform: scale(1.15);
		background: inherit !important;
	}

	.color-dot.active {
		border-color: white;
	}

	/* Toggle switch */
	.toggle {
		position: relative;
		width: 40px;
		height: 22px;
		border-radius: 11px;
		background: rgba(255, 255, 255, 0.15);
		padding: 0;
		border: none;
		cursor: pointer;
		transition: background 0.2s;
		flex-shrink: 0;
	}

	.toggle.on {
		background: var(--accent);
	}

	.toggle-thumb {
		position: absolute;
		top: 2px;
		left: 2px;
		width: 18px;
		height: 18px;
		border-radius: 50%;
		background: white;
		transition: transform 0.2s;
		pointer-events: none;
	}

	.toggle.on .toggle-thumb {
		transform: translateX(18px);
	}

	/* Poll pills */
	.pill-group {
		display: flex;
		gap: 4px;
	}

	.poll-pill {
		padding: 4px 10px;
		border-radius: 8px;
		font-size: 11px;
		color: var(--text-secondary);
		background: var(--bg-secondary);
		border: none;
		cursor: pointer;
	}

	.poll-pill:hover {
		background: var(--bg-active);
	}

	.poll-pill.active {
		background: var(--accent);
		color: white;
	}

	/* Shortcut info */
	.info {
		flex-direction: row;
		align-items: center;
		justify-content: space-between;
		padding-top: 8px;
		border-top: 1px solid var(--border);
	}

	.shortcut-label {
		font-size: 11px;
		color: var(--text-muted);
	}

	kbd {
		font-family: -apple-system, sans-serif;
		font-size: 12px;
		color: var(--text-secondary);
		background: var(--bg-secondary);
		padding: 2px 8px;
		border-radius: 4px;
		border: 1px solid var(--border);
	}
</style>
