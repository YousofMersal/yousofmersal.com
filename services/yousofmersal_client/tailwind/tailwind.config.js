/** @type {import('tailwindcss').Config} */
const colors = require('tailwindcss/colors');

module.exports = {
  content: [ "./src/**/*.rs" ],
  daisyui: {
    styled: true,
    themes: true,
    base: true,
    utils: true,
    logs: true,
    rtl: false,
    prefix: "",
    darkTheme: "dark"
  },
  theme: {
    mode: 'jit',
    darkMode: "class",
    colors: {
      ico: '#303841',
      slate: colors.slate
    }
  },
  plugins: [
    require('@tailwindcss/typography'),
    require('daisyui')
  ]
}
