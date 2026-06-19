/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './src/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  theme: {
    extend: {
      colors: {
        anvaya: {
          charcoal: '#0f172a',
          dark: '#1e293b',
          accent: '#3b82f6',
          cobalt: '#2563eb',
          light: '#60a5fa',
        },
      },
    },
  },
  plugins: [],
};
