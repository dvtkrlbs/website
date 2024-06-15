/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "selector",
  content: {
    relative: true,
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      textColor: {
        main: 'rgb(var(--color-text-main) / <alpha-value>)'
      },
      backgroundColor: {
        main: 'rgb(var(--color-bg-main) / <alpha-value>)',
        muted: 'rgb(var(--color-bg-muted) / <alpha-value>)'
      },
      borderColor: {
        main: 'rgb(var(--color-border-main) / <alpha-value>)'
      },
    },
  },
  plugins: [],
}
