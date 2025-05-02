import 'vuetify/styles';
import '@/scss/variables.scss';

import { aliases, mdi } from 'vuetify/iconsets/mdi-svg';
import { createVuetify } from 'vuetify';

export default createVuetify({
	icons: {
		defaultSet: 'mdi',
		aliases,
		sets: { mdi }
	},

	theme: {
		themes: {
			light: {
				colors: {
					primary: '#ffcc00',
					offwhite: '#ffeecb',
					bg: '#4f0091'
				}
			}
		}
	}
});