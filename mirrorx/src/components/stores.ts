import { writable } from 'svelte/store';
import type { Domain } from '$lib/components/types';

export const current_domain = writable<Domain | null>(null);
