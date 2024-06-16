/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "class",
  content: {
    files: ["./*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      typography: (theme) => ({
        DEFAULT: {
          css: {
            a: {
              textDecorationStyle: 'dashed',
              '&:hover': {
                textDecorationStyle: 'solid'
              }
            }
          },
        },
      }),
    },
  },
  plugins: [
    require("@tailwindcss/typography"),
  ],
}
