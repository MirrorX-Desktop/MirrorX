export interface DeleteConfirmEvent {
	domain_id: number;
	domain_name: string;
}

export interface SwitchPrimaryDomainEvent {
	domain_id: number;
	domain_name: string;
}

export interface EditDomainEvent {
	domain_id: number;
	domain_name: string;
	domain_device_id: string;
	domain_finger_print: string;
	domain_remarks: string;
}
