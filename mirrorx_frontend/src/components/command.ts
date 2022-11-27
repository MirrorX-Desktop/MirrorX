import { invoke } from '@tauri-apps/api';

export function invoke_init_config(): Promise<void> {
	return invoke('init_config');
}

export function invoke_init_signaling(args: { force: boolean }): Promise<void> {
	return invoke('init_signaling', args);
}

export function invoke_get_current_domain(): Promise<{ name: string; device_id: string; password: string }> {
	return invoke('get_current_domain');
}

export function invoke_generate_random_password(): Promise<string> {
	return invoke<string>('generate_random_password');
}

export function invoke_set_current_domain_device_password(args: { password: string }): Promise<void> {
	return invoke('set_current_domain_device_password', args);
}

export function invoke_signaling_visit_request(args: { remoteDeviceId: string }): Promise<void> {
	return invoke('signaling_visit_request', args);
}

export function invoke_signaling_key_exchange(args: {
	addr: string;
	localDeviceId: string;
	remoteDeviceId: string;
	password: string;
}): Promise<void> {
	return invoke('signaling_key_exchange', args);
}

export function invoke_signaling_reply_visit_request(args: {
	allow: boolean;
	activeDeviceId: string;
	passiveDeviceId: string;
}): Promise<void> {
	return invoke('signaling_reply_visit_request', args);
}

export function invoke_get_domains(args: { page: number; limit: number }): Promise<{
	total: number;
	current_domain_name: string;
	domains: Array<{
		id: number;
		name: string;
		addr: string;
		device_id: string;
		finger_print: string;
		remarks: string;
	}>;
}> {
	return invoke('get_domains', args);
}

export function invoke_add_domain(args: { addr: string; remarks: string }): Promise<void> {
	return invoke('add_domain', args);
}

export function invoke_delete_domain(args: { id: number }): Promise<void> {
	return invoke('delete_domain', args);
}

export function invoke_switch_primary_domain(args: { id: number }): Promise<void> {
	return invoke('switch_primary_domain', args);
}

export function invoke_set_domain_remarks(args: { id: number; remarks: string }): Promise<void> {
	return invoke('set_domain_remarks', args);
}

export function invoke_set_language(args: { language: string }): Promise<void> {
	return invoke('set_language', args);
}

export function invoke_get_language(): Promise<string> {
	return invoke('get_language');
}

export function invoke_init_lan_discover(): Promise<void> {
	return invoke('init_lan_discover');
}

export function invoke_get_lan_discover_nodes(): Promise<
	Array<{
		host_name: string;
		addr: string;
		os: string;
		os_version: string;
		tcp_port: number;
		udp_port: number;
	}>
> {
	return invoke('get_lan_discover_nodes');
}
