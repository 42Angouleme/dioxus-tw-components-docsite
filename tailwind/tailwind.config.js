/** @type {import('tailwindcss').Config} */
module.exports = {
	mode: "all",
	content: [
		// include all rust, html and css files in the src directory
		"../src/**/*.{rs,html,css}",
		"./safelist.txt",
		"../../dioxus-components/src/**/*.{rs,html,css}",
		`${process.env.HOME}/.cargo/registry/src/**/dioxus-tw-components-*/src/**/*.{rs,html,css}`,
	],
	theme: {
		extend: {
			colors: {
				"background": 'hsl(var(--background))',
				"foreground": 'hsl(var(--foreground))',
				"primary": 'hsl(var(--primary))',
				"primary-foreground": 'hsl(var(--primary-foreground))',
				"secondary": 'hsl(var(--secondary))',
				"secondary-foreground": 'hsl(var(--secondary-foreground))',
				"accent": 'hsl(var(--accent))',
				"accent-foreground": 'hsl(var(--accent-foreground))',
				"muted": 'hsl(var(--muted))',
				"muted-foreground": 'hsl(var(--muted-foreground))',
				"destructive": 'hsl(var(--destructive))',
				"destructive-foreground": 'hsl(var(--destructive-foreground))',
				"success": 'hsl(var(--success))',
				"success-foreground": 'hsl(var(--success-foreground))',
				"border": 'hsl(var(--border))',
				"input": 'hsl(var(--input))',
				"ring": 'hsl(var(--ring))',
			},
			borderRadius: {
				"global-radius": 'var(--global-radius)',
			},
			boxShadow: {
				"global-shadow": 'var(--global-shadow)',
			},
			keyframes: {
				"shimmer" : {
					"100%" : {
						"transform" : "translateX(200%)",
					}
				}
			},
			fontSize: {
			},
		},
	},
	plugins: [],
}
