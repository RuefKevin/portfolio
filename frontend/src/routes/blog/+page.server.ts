import type { PageServerLoad } from './$types';
import type { BlogPost } from '$lib/types';
import { env } from '$env/dynamic/private';
import { logRequest, logError } from '$lib/logger';

export const load: PageServerLoad = async ({ fetch }) => {
    try {
        const response = await fetch(`${env.BACKEND_URL}/api/blog`);
        logRequest(response.status, `${env.BACKEND_URL}/api/blog`);

        if (!response.ok) {
            logError('Backend error', response.status);
            return { posts: [] as BlogPost[], error: true };
        }

        const posts: BlogPost[] = await response.json();
        return { posts, error: false };
    } catch (error) {
        logError(`Fetch error [${env.BACKEND_URL}/api/blog]`, error);
        return { posts: [] as BlogPost[], error: true };
    }
};
