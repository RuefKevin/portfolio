import type { PageServerLoad } from './$types';
import type { Project } from '$lib/types';
import { env } from '$env/dynamic/private';
import { logRequest, logError } from '$lib/logger';

export const load: PageServerLoad = async ({ fetch }) => {
	try {
		const response = await fetch(`${env.BACKEND_URL}/api/projects`);
		logRequest(response.status, `${env.BACKEND_URL}/api/projects`);

		if (!response.ok) {
			logError('Backend error', response.status);
			return { projects: [] as Project[], error: true };
		}

		const projects: Project[] = await response.json();
		return { projects, error: false };
	} catch (error) {
		logError(`Fetch error [${env.BACKEND_URL}/api/projects]`, error);
		return { projects: [] as Project[], error: true };
	}
};
