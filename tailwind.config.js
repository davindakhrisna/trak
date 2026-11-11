/** @type {import('tailwindcss').Config} */
module.exports = {
	darkMode: "class",
	content: {
		files: ["*.html", "./src/**/*.rs"],
	},
	theme: {
		colors: {
			white: "#FFFFFF",
			black: "#000000",
			gray: "#737373",
			transparent: "transparent",
			current: "currentColor",
		},
		borderRadius: {
			none: "0px",
			DEFAULT: "0px",
			sm: "0px",
			md: "0px",
			lg: "0px",
			xl: "0px",
			"2xl": "0px",
			"3xl": "0px",
			full: "0px",
		},
		boxShadow: {
			none: "none",
			DEFAULT: "none",
		},
		extend: {
			fontFamily: {
				sans: ['"Space Grotesk"', "system-ui", "sans-serif"],
				mono: ['"IBM Plex Mono"', "monospace"],
			},
			maxWidth: {
				spine: "440px",
				"spine-wide": "780px",
			},
		},
	},
	plugins: [],
};
