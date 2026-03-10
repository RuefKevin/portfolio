import type { PageServerLoad } from './$types';
import { env } from '$env/dynamic/private';

export const load: PageServerLoad = async () => {
	return {
		contact: {
			email: env.CONTACT_EMAIL ?? '',
			github: env.CONTACT_GITHUB ?? '',
			linkedin: env.CONTACT_LINKEDIN ?? ''
		}
	};
};
