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
