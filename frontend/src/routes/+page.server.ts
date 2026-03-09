import type { PageServerLoad } from './$types';
import type { Project } from '$lib/types';
import { env } from '$env/dynamic/private';

export const load: PageServerLoad = async ({ fetch }) => {
	try {
		const response = await fetch(`${env.BACKEND_URL}/api/projects`);
		console.log(`[${response.status}] GET ${env.BACKEND_URL}/api/projects`);

		if (!response.ok) {
			console.error(`Backend error: ${response.status}`);
			return { projects: [] as Project[], error: true };
		}

		const projects: Project[] = await response.json();
		return { projects, error: false };
	} catch (error) {
		if (error instanceof Error) {
			console.error('Fetch error:', error.message);
		} else {
			console.error('Fetch error: unknown');
		}
		return { projects: [] as Project[], error: true };
	}
};
