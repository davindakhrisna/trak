/** @type {import('tailwindcss').Config} */
module.exports = {
	darkMode: "class",
	content: {
		files: ["*.html", "./src/**/*.rs"],
	},
	theme: {
		colors: {
			canvas: "var(--bg-canvas)",
			surface: "var(--bg-surface)",
			muted: "var(--bg-muted)",
			ink: "var(--text-ink)",
			"ink-muted": "var(--text-muted)",
			rule: "var(--border-rule)",
			"rule-strong": "var(--border-strong)",
			signal: "var(--signal)",
			"signal-soft": "var(--signal-soft)",
			white: "var(--bg-surface)",
			black: "var(--text-ink)",
			gray: "var(--text-muted)",
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
