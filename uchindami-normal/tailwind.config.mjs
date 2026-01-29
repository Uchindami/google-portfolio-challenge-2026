import tailwindcssAnimate from "tailwindcss-animate";

/** @type {import('tailwindcss').Config} */
export default {
	darkMode: ["class"],
	content: ["./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}"],
	theme: {
		extend: {
			borderRadius: {
				lg: "var(--radius)",
				md: "calc(var(--radius) - 2px)",
				sm: "calc(var(--radius) - 4px)",
			},
			colors: {
				primary: "rgb(var(--color-primary) / <alpha-value>)",
				secondary: "rgb(var(--color-secondary) / <alpha-value>)",
				text_color: "rgb(var(--color-text) / <alpha-value>)",
				background: "rgb(var(--color-background) / <alpha-value>)",
				surface: "rgb(var(--color-surface) / <alpha-value>)",
				blog: {
					primary: "#1c5cff",
					secondary: "#1e1e1e",
					third: "#969696",
					bento_bg: "#FAF7F0",
					bento_box: "#D8D2C2",
					bento_heading: "#4A4947",
					bento_text: "#181C14",
					bento_accent: "#B17457",
					nav_bg: "#181C14",
					nav_fill: "#c6bdb3",
					nav_hover: "#3C3D37",
					nav_active: "#c6bdb3",
				},
			},
			fontFamily: {
				sans: ["Poppins", "system-ui", "sans-serif"],
				roboto: ["Roboto Mono", "monospace"],
			},
		},
	},
	plugins: [tailwindcssAnimate],
};
