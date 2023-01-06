import { writable } from 'svelte/store';
import type { Directory, Domain } from '$lib/components/types';

export const current_domain = writable<Domain | null>(null);

export const current_remote_directory = writable<Directory | null>(null);
