/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,jsx,ts,tsx}","./public/splashscreen.html"],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        background: "#121212", // Dark background color
        sidebar: "#1E1E1E", // Sidebar background
        primaryText: "#EAEAEA", // Light text
        secondaryText: "#A0A0A0", // Muted text
        button: "#3B82F6", // Blue buttons
        buttonHover: "#2563EB", // Darker blue on hover
      },
      borderRadius: {
        'xl': '1rem'
      },
      animation: {
        'pulse-slow': 'pulse 1.5s ease-in-out infinite',
      },
      keyframes: {
        pulse: {
          '0%, 100%': { opacity: '0.7' },
          '50%': { opacity: '1' },
        },
      }
    },
  },
  plugins: [],
}

