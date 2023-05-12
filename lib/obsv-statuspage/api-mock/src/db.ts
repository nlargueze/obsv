/**
 * DB mock data
 */

import { faker } from '@faker-js/faker';

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
	timestamp: Date;
	resp_time_ms: number | null;
	ok: boolean;
};

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
	monitors: Monitor[];
	incidents: Incident[];
	notices: Notice[];
};

function genMonitorCheck(): MonitorCheck {
	return {
		timestamp: faker.date.past(),
		resp_time_ms: faker.number.int({ min: 30, max: 2000 }),
		ok: faker.datatype.boolean({ probability: 0.9 })
	};
}

/**
 * Generates an monitor
 */
export function genMonitor(): Monitor {
	return {
		id: faker.string.uuid(),
		name: faker.word.words(),
		description: faker.lorem.paragraph(),
		checks: faker.helpers.multiple(genMonitorCheck, {
			count: 400
		})
	};
}

/**
 * Generates an incident
 */
export function genIncident(): Incident {
	return {
		id: faker.string.uuid(),
		title: faker.lorem.sentence(),
		description: faker.lorem.paragraph(),
		opened: faker.date.past(),
		closed: null,
		status: IncidentStatus.Opened,
		history: []
	};
}

/**
 * Generates a maintenance
 */
export function genNotice(): Notice {
	return {
		id: faker.string.uuid(),
		title: faker.lorem.sentence(),
		description: faker.lorem.paragraph(),
		type: faker.string.sample(),
		start: faker.date.future(),
		end: faker.date.future()
	};
}

/**
 * Generates a complete status
 */
export function genStatus(): Status {
	return {
		monitors: faker.helpers.multiple(genMonitor, {
			count: 2
		}),
		incidents: faker.helpers.multiple(genIncident, {
			count: 2
		}),
		notices: faker.helpers.multiple(genNotice, {
			count: 1
		})
	};
}

export default {
	genStatus
};
