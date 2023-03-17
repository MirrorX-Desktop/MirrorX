import { emit } from '@tauri-apps/api/event';

export interface NotificationEvent {
	level: 'info' | 'success' | 'warning' | 'error';
	title: string;
	message: string;
}

export function emitNotification(event: NotificationEvent): Promise<void> {
	return emit('/dialog/notification', event);
}

export function emitFileNotification(event: NotificationEvent): Promise<void> {
	return emit('/dialog/notification/file', event);
}
