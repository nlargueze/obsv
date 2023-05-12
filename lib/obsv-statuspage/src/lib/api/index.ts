import { CONFIG } from '$lib/config';
import type { Status } from './types';

export * from './types';

/**
 * API client
 *
 * This API fetches the status on the server
 */
export class ApiClient {
	private _url: string = CONFIG.api_url;
	private _fetch: typeof fetch;

	constructor(svelte_fetch: typeof fetch) {
		this._fetch = svelte_fetch;
	}

	/**
	 * Returns the
	 */
	async get_status(): Promise<Status> {
		const response = await this._fetch(this._url + '/status');
		const status: Status = await response.json();

		// NB: we need to convert the dates from strings
		for (const monitor of status.monitors) {
			for (const check of monitor.checks) {
				check.timestamp = new Date(check.timestamp);
			}
		}

		for (const notice of status.notices) {
			notice.start = new Date(notice.start);
			notice.end = new Date(notice.end);
		}

		return status;
	}
}
