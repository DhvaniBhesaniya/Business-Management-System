/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./index.html', './src/**/*.{js,jsx}'],
  darkMode: 'class',
  theme: {
    extend: {
      boxShadow: {
        glass: '0 10px 30px rgba(0,0,0,0.25)',
      },
    },
  },
  plugins: [],
};

