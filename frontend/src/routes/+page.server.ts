import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () =>
{
    const response = await fetch('http://127.0.0.1:3000/api/projects');
    const projects = await response.json();
    return { projects };
};
