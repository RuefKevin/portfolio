import type { PageServerLoad } from './$types';
import { BACKEND_URL } from '$env/static/private';

export const load: PageServerLoad = async () =>
{
    try
    {
        const response = await fetch(`${BACKEND_URL}/api/projects`);
        console.log(`[${response.status}] GET ${BACKEND_URL}/api/projects`);
        const projects = await response.json();
        return { projects };
    }
    catch (error)
    {
        console.error('Fetch error:', error);
        return { projects: [] };
    }
};
