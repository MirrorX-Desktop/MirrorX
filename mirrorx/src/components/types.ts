export const isMacOS = navigator.platform.toLowerCase().includes('mac');

export interface Domain {
	id: number;
	name: string;
	addr: string;
	signaling_port: string;
	subscribe_port: string;
	is_primary: boolean;
	device_id: number;
	password: string;
	finger_print: string;
	remarks: string;
}

export interface LanDiscoverNode {
	display_name: string;
	addrs: Map<string, number>;
	os: string;
	os_version: string;
}

export interface HistoryRecord {
	id: number;
	device_id: number;
	domain: string;
	timestamp: number;
}

export interface Directory {
	path: string;
	entries: Array<Entry>;
	hashed_icons: { [key: string]: string; };
}

export interface Entry {
	is_dir: boolean;
	path: string;
	modified_time: number;
	size: number;
	icon: string | null;
	icon_hash: string | null;
}

export interface FileTransferItem {
	id: string;
	is_upload: boolean;
	local_path: string;
	remote_path: string;
	transferred_size: number;
	total_size: number;
	last_transferred_delta_size: number;
	launch_at: number;
	succeed_at: number;
	failed_at: number;
}

export interface FileTransferItem {
	id: string;
	is_upload: boolean;
	local_path: string;
	remote_path: string;
	transferred_size: number;
	total_size: number;
	last_transferred_delta_size: number;
	launch_at: number;
	succeed_at: number;
	failed_at: number;
}
