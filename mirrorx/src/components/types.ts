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
	host_name: string;
	addr: string;
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
	sub_dirs: Array<DirEntry>;
	files: Array<FileEntry>;
}

export interface DirEntry {
	path: string;
	modified_time: number;
	icon: Uint8Array | null;
}

export interface FileEntry {
	path: string;
	modified_time: number;
	size: number;
	icon: Uint8Array | null;
}
