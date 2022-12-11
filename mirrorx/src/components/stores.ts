import { writable } from 'svelte/store';
import type { Domain, LanDiscoverNode } from '$lib/components/types';

export const current_domain = writable<Domain | null>(null);

export const current_lan_discover_nodes = writable<Array<LanDiscoverNode>>([]);
