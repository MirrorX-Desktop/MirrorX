import { writable } from 'svelte/store';

export interface CurrentDomain {
	name: string;
	device_id: string;
	password: string;
}

export const current_domain = writable<CurrentDomain | null>(null);

export interface LanDiscoverNode {
	host_name: string;
	addr: string;
	os: string;
	os_version: string;
}

export const current_lan_discover_nodes = writable<Array<LanDiscoverNode>>([]);
