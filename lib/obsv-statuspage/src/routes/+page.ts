import { ApiClient, type Status } from '$lib/api';

/** @type {import('./$types').PageLoad} */
export async function load({ fetch }) {
	const client = new ApiClient(fetch);
	const status = await client.get_status();

	process(status);

	return {
		status,
	};
}

/**
 * Processes the received status
 */
function process(status: Status): {
	monitors: {
		a: number,
	}
} {
	return {
		monitors: {
			a: 1,
		}
	};
}
