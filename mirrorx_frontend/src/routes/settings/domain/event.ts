export interface DeleteConfirmEvent {
	domain_id: number;
	domain_name: string;
}

export interface SwitchPrimaryDomainEvent {
	domain_id: number;
	domain_name: string;
}
