import { writable } from 'svelte/store';

export interface CurrentDomain {
	name: string;
	device_id: string;
	password: string;
}

export const current_domain = writable<CurrentDomain | null>(null);
