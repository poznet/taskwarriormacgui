import { writable } from 'svelte/store';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { LogicalSize } from '@tauri-apps/api/dpi';
import { invoke } from '@tauri-apps/api/core';

export interface Settings {
	opacity: number;
	compact: boolean;
	pollInterval: number;
	alwaysOnTop: boolean;
	showCompleted: boolean;
	accentColor: string;
	viewMode: 'list' | 'kanban';
	showPomodoro: boolean;
	notifyDue: boolean;
	windowWidth: number;
	windowHeight: number;
	taskBinaryPath: string;
}

const STORAGE_KEY = 'taskfloat-settings';

const defaults: Settings = {
	opacity: 0.92,
	compact: false,
	pollInterval: 15000,
	alwaysOnTop: true,
	showCompleted: true,
	accentColor: '#0A84FF',
	viewMode: 'list',
	showPomodoro: false,
	notifyDue: true,
	windowWidth: 380,
	windowHeight: 600,
	taskBinaryPath: '',
};

function loadSettings(): Settings {
	try {
		const raw = localStorage.getItem(STORAGE_KEY);
		if (raw) return { ...defaults, ...JSON.parse(raw) };
	} catch {}
	return { ...defaults };
}

export const settings = writable<Settings>(loadSettings());

settings.subscribe((s) => {
	try {
		localStorage.setItem(STORAGE_KEY, JSON.stringify(s));
		invoke('set_task_binary_path', { path: s.taskBinaryPath || '' }).catch(() => {});
	} catch {}
});

// Cross-window sync: when another window updates settings, reload them here
if (typeof window !== 'undefined') {
	window.addEventListener('storage', (e) => {
		if (e.key === STORAGE_KEY && e.newValue) {
			try {
				const updated = { ...defaults, ...JSON.parse(e.newValue) };
				settings.set(updated);
				applySettings(updated);
			} catch {}
		}
	});
}

export function applySettings(s: Settings) {
	const win = getCurrentWindow();

	// Opacity
	document.documentElement.style.setProperty('--window-opacity', String(s.opacity));

	// Compact mode
	document.documentElement.classList.toggle('compact', s.compact);

	// Accent color
	document.documentElement.style.setProperty('--accent', s.accentColor);
	document.documentElement.style.setProperty('--accent-hover', lighten(s.accentColor, 20));
	document.documentElement.style.setProperty('--border-focus', s.accentColor + '80');

	// Always on top
	win.setAlwaysOnTop(s.alwaysOnTop);

	// Kanban mode: set appropriate min width
	if (win.label === 'main') {
		if (s.viewMode === 'kanban') {
			win.setMinSize(new LogicalSize(600, 400));
		} else {
			win.setMinSize(new LogicalSize(300, 400));
		}
	}
}

/** Restore saved window size and start tracking resize. Call once on mount. */
export function initWindowSize() {
	const win = getCurrentWindow();
	if (win.label !== 'main') return;

	const s = loadSettings();
	// Restore saved size
	win.setSize(new LogicalSize(s.windowWidth, s.windowHeight));

	// Track resize with debounce
	let resizeTimer: ReturnType<typeof setTimeout>;
	win.onResized(({ payload: physSize }) => {
		clearTimeout(resizeTimer);
		resizeTimer = setTimeout(() => {
			win.scaleFactor().then(scale => {
				const w = Math.round(physSize.width / scale);
				const h = Math.round(physSize.height / scale);
				settings.update(prev => ({ ...prev, windowWidth: w, windowHeight: h }));
			});
		}, 500);
	});
}

function lighten(hex: string, percent: number): string {
	const num = parseInt(hex.replace('#', ''), 16);
	const r = Math.min(255, (num >> 16) + Math.round(2.55 * percent));
	const g = Math.min(255, ((num >> 8) & 0x00ff) + Math.round(2.55 * percent));
	const b = Math.min(255, (num & 0x0000ff) + Math.round(2.55 * percent));
	return '#' + (0x1000000 + r * 0x10000 + g * 0x100 + b).toString(16).slice(1);
}
