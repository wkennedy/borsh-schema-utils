/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        'high-confidence': '#27ae60',
        'medium-confidence': '#f39c12',
        'low-confidence': '#e74c3c',
      }
    },
  },
  plugins: [],
}

