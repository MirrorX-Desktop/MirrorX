import { sveltekit } from '@sveltejs/kit/vite';
import type { UserConfig } from 'vite';

const config: UserConfig = {
	plugins: [sveltekit()],
	ssr: {
		noExternal: ['typesafe-i18n']
	},
	server: {
		fs: {
			allow: ['..']
		}
	}
};

export default config;
