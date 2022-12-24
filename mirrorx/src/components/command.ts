import { invoke } from '@tauri-apps/api';
import type { Domain, HistoryRecord, LanDiscoverNode } from '$lib/components/types';

export function invoke_config_init(): Promise<void> {
	return invoke('config_init');
}

export function invoke_config_domain_get(): Promise<Domain> {
	return invoke('config_domain_get');
}

export function invoke_config_domain_get_by_name(name: string): Promise<Domain> {
	return invoke('config_domain_get_by_name', { name });
}

export function invoke_config_domain_get_id_and_names(): Promise<Array<[number, string]>> {
	return invoke('config_domain_get_id_and_names');
}

export function invoke_config_domain_create(addr: string, remarks: string): Promise<void> {
	return invoke('config_domain_create', { addr, is_primary: false, remarks });
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

export function invoke_config_theme_get(): Promise<'light' | 'dark' | 'auto' | null> {
	return invoke('config_theme_get');
}

export function invoke_config_theme_set(theme: 'light' | 'dark' | 'auto'): Promise<void> {
	return invoke('config_theme_set', { theme });
}

export function invoke_config_history_get(time_range: [number, number] | null): Promise<Array<HistoryRecord>> {
	return invoke('config_history_get', { timeRange: time_range });
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

export function invoke_lan_nodes_search(keyword: string): Promise<Array<LanDiscoverNode>> {
	return invoke('lan_nodes_search', { keyword });
}

export function invoke_lan_discoverable_get(): Promise<boolean> {
	return invoke('lan_discoverable_get');
}

export function invoke_lan_discoverable_set(discoverable: boolean): Promise<void> {
	return invoke('lan_discoverable_set', { discoverable });
}

export function invoke_signaling_connect(force: boolean): Promise<void> {
	return invoke('signaling_connect', { force });
}

export function invoke_signaling_visit(remoteDeviceId: string, password: string): Promise<void> {
	return invoke('signaling_visit', { remoteDeviceId, password });
}

export function invoke_utility_generate_random_password(): Promise<string> {
	return invoke('utility_generate_random_password');
}

export function invoke_utility_detect_os_platform(): Promise<string> {
	return invoke('utility_detect_os_platform');
}

export function invoke_utility_enum_graphics_cards(): Promise<Array<{ name: string; is_default: boolean }>> {
	return invoke('utility_enum_graphics_cards');
}

export function invoke_utility_hide_macos_zoom_button(): Promise<void> {
	return invoke('utility_hide_macos_zoom_button');
}
