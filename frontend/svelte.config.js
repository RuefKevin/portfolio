import adapter from '@sveltejs/adapter-node';

const config = {
	kit: {
		adapter: adapter(),
		csp: {
			mode: 'auto',
			directives: {
				'default-src': ['self'],
				'script-src': ['self'],
				'style-src': ['self', 'unsafe-inline', 'https://fonts.googleapis.com'],
				'font-src': ['self', 'https://fonts.gstatic.com'],
				'img-src': ['self', 'data:'],
				'connect-src': ['self'],
				'frame-ancestors': ['none']
			}
		}
	},
	compilerOptions: {
		runes: true
	}
};

export default config;
