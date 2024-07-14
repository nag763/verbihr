/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.rs"],
  theme: {
  extend: {
		gridTemplateRows: {
			'12': 'repeat(12, minmax(0, 1fr))',
		},
		gridRow: {
				'span-9': 'span 9 / span 9',
				'span-10': 'span 10 / span 10',
			}
		},
  },
  plugins: [require('daisyui'), 
  require('tailwindcss-animated')],
  daisyui: {
    themes: [
      {
        light: {
          ...require("daisyui/src/theming/themes")["light"],
		  "base-100": "#eceff4",
		},
	},
	"dark"
	]
}
}
