export interface PublishMessage {
	VisitRequest?: VisitRequest;
}

export interface VisitRequest {
	active_device_id: string;
	passive_device_id: string;
	resource_type: string;
}
