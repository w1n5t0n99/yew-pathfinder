/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{rs,html}"
  ],
  theme: {
    extend: {
      gridTemplateColumns: {
        'landing-fill': 'repeat(auto-fit, minmax(320px, 1fr))',
        'list-fill': 'repeat(auto-fit, minmax(320px, 1fr))',
      },
      fontFamily: {
        'header': ['Oswald', 'ui-sans-serif', 'system-ui'],
        'body': ['Roboto', 'ui-sans-serif', 'system-ui'],
      },
      colors: {
        'board': '#095C6C',
        'nav': '#013842',
        'start': '#088423',
        'end': '#AF1B0B',
      },
      keyframes: {
        wall: {
          '0%': { transform: 'scale(.3)', backgroundColor: 'rgb(12, 53, 71)'  },
          '50%': { transform: 'scale(1.2)', backgroundColor: 'rgb(12, 53, 71)'  },
          '100%': { transform: 'scale(1.0)', backgroundColor: 'rgb(12, 53, 71)'  },
        },
      },
      animation: {
        'wall': 'wall .3s ease-out',
      },
    },
  },
  plugins: [],
}

