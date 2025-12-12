import { createVuetify } from 'vuetify'
import { aliases, mdi } from 'vuetify/iconsets/mdi-svg'

import 'vuetify/styles'
import '@/scss/variables.scss'

export default createVuetify({
	icons: {
		defaultSet: 'mdi',
		aliases,
		sets: { mdi },
	},

	theme: {
		themes: {
			light: {
				colors: {
					primary: '#ffcc00',
					offwhite: '#ffeecb',
					bg: '#4f0091',
				},
			},
		},
	},
})
