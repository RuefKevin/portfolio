import type { PageServerLoad } from './$types';
import type { BlogPost } from '$lib/types';
import { env } from '$env/dynamic/private';
import { logRequest, logError } from '$lib/logger';
import { error } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ fetch, params }) => {
	try {
		const response = await fetch(`${env.BACKEND_URL}/api/blog/${params.slug}`);
		logRequest(response.status, `${env.BACKEND_URL}/api/blog/${params.slug}`);

		if (response.status === 404) {
			error(404, 'Post not found');
		}

		if (!response.ok) {
			logError('Backend error', response.status);
			error(500, 'Could not load post');
		}

		const post: BlogPost = await response.json();
		return { post };
	} catch (e) {
		logError(`Fetch error [${env.BACKEND_URL}/api/blog/${params.slug}]`, e);
		error(500, 'Could not load post');
	}
};
