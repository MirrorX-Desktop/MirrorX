export interface PopupDialogVisitRequestEvent {
	active_device_id: string;
	passive_device_id: string;
	resource_type: string;
}

export interface PopupDialogInputRemotePasswordEvent {
	active_device_id: string;
	passive_device_id: string;
}

export type NotificationEvent = {
	level: 'info' | 'success' | 'warning' | 'error';
	title: string;
	message: string;
};
