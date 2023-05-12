/**
 * A monitor
 */
export type Monitor = {
	id: string;
	name: string;
	description: string;
	checks: MonitorCheck[];
};

/**
 * A monitor check
 */
export type MonitorCheck = {
	timestamp: Date,
	resp_time_ms: number | null;
	ok: boolean,
}

/**
 * An incident
 */
export type Incident = {
	id: string;
	title: string;
	description: string;
	opened: Date;
	closed?: Date | null;
	status: IncidentStatus;
	history: IncidentUpdate[];
};

/**
 * Incident status
 */
export enum IncidentStatus {
	Opened,
	InProgress,
	Closed
}

/**
 * Incident update
 */
export type IncidentUpdate = {
	timestamp: Date;
	description: string;
	status: IncidentStatus;
};


/**
 * A notice (eg maintenance)
 */
export type Notice = {
	id: string;
	title: string;
	description: string;
	type: string;
	start: Date;
	end: Date;
};

/**
 * Status
 */
export type Status = {
	monitors: Monitor[],
	incidents: Incident[],
	notices: Notice[],
}
