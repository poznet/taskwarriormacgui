import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/plugin-notification';
import type { Task } from '$lib/types/task';

const notifiedToday = new Set<string>();

export async function checkDueNotifications(tasks: Task[]): Promise<void> {
	const now = new Date();
	const todayStr = `${now.getFullYear()}${String(now.getMonth() + 1).padStart(2, '0')}${String(now.getDate()).padStart(2, '0')}`;

	const dueTasks = tasks.filter((t) => {
		if (t.status !== 'pending' || !t.due) return false;
		if (notifiedToday.has(t.uuid)) return false;
		const dueDay = t.due.slice(0, 8);
		return dueDay <= todayStr; // due today or overdue
	});

	if (dueTasks.length === 0) return;

	try {
		let granted = await isPermissionGranted();
		if (!granted) {
			const perm = await requestPermission();
			granted = perm === 'granted';
		}
		if (!granted) return;

		// Group: overdue vs today
		const overdue = dueTasks.filter((t) => t.due!.slice(0, 8) < todayStr);
		const today = dueTasks.filter((t) => t.due!.slice(0, 8) === todayStr);

		if (overdue.length > 0) {
			sendNotification({
				title: `${overdue.length} przeterminowanych taskow!`,
				body: overdue.slice(0, 3).map((t) => t.description).join(', '),
			});
			overdue.forEach((t) => notifiedToday.add(t.uuid));
		}

		if (today.length > 0) {
			sendNotification({
				title: `${today.length} taskow na dzis`,
				body: today.slice(0, 3).map((t) => t.description).join(', '),
			});
			today.forEach((t) => notifiedToday.add(t.uuid));
		}
	} catch {}
}

// Reset notifications at midnight
setInterval(() => {
	const now = new Date();
	if (now.getHours() === 0 && now.getMinutes() === 0) {
		notifiedToday.clear();
	}
}, 60000);
