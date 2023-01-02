module.exports = {
  content: [
    './src/**/*.rs',
    './public/**/*.html',
    './index.html',
    './src/**/*.{js,jsx,ts,tsx,vue,rs}'
  ],
  plugins: [
    require('@tailwindcss/forms'),
    require('@tailwindcss/typography'),
  ],


  theme: {
    extends: {},
    container: {
      center: false,
    },
  },


}