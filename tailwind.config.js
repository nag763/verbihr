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
		}
  },
  darkMode: 'class',
  plugins: [],
}
