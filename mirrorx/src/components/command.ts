import { invoke } from '@tauri-apps/api';
import type { Domain, LanDiscoverNode } from '$lib/components/types';

export function invoke_config_init(): Promise<void> {
	return invoke('config_init');
}

export function invoke_config_domain_get(): Promise<Domain> {
	return invoke('config_domain_get');
}

export function invoke_config_domain_create(addr: string, remarks: string): Promise<void> {
	return invoke('config_domain_create', { addr, remarks });
}

export function invoke_config_domain_delete(id: number): Promise<void> {
	return invoke('config_domain_delete', { id });
}

export function invoke_config_domain_list(
	page: number,
	limit: number
): Promise<{ total: number; domains: Array<Domain> }> {
	return invoke('config_domain_list', { page, limit });
}

export function invoke_config_domain_update(
	id: number,
	update_type: 'set_primary' | { password: string } | { remarks: string }
): Promise<void> {
	return invoke('config_domain_update', { req: { id, update_type } });
}

export function invoke_config_language_get(): Promise<string> {
	return invoke('config_language_get');
}

export function invoke_config_language_set(language: string): Promise<void> {
	return invoke('config_language_set', { language });
}

export function invoke_lan_init(force: boolean): Promise<void> {
	return invoke('lan_init', { force });
}

export function invoke_lan_connect(addr: string): Promise<void> {
	return invoke('lan_connect', { addr });
}

export function invoke_lan_nodes_list(): Promise<Array<LanDiscoverNode>> {
	return invoke('lan_nodes_list');
}

export function invoke_signaling_connect(force: boolean): Promise<void> {
	return invoke('signaling_connect', { force });
}

export function invoke_signaling_visit(remote_device_id: string, password: string): Promise<void> {
	return invoke('signaling_visit', { remote_device_id, password });
}

export function invoke_utility_generate_random_password(): Promise<string> {
	return invoke('utility_generate_random_password');
}
