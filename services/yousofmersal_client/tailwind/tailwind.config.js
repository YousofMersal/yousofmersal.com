/** @type {import('tailwindcss').Config} */
const colors = require('tailwindcss/colors');

module.exports = {
  content: ["./src/**/*.{rs,html,css}"],
  daisyui: {
    styled: true,
    base: true,
    utils: true,
    logs: true,
    rtl: false,
    prefix: ""
  },
  darkMode: ["class", '[data-theme="dark"]'],
  theme: {
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
