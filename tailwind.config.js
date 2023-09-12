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
        'visited': '#00beda',
        'shortestpath': '#fffe6a'
      },
      keyframes: {
        wall: {
          '0%': { transform: 'scale(.3)', backgroundColor: 'rgb(12, 53, 71)'  },
          '50%': { transform: 'scale(1.2)', backgroundColor: 'rgb(12, 53, 71)'  },
          '100%': { transform: 'scale(1.0)', backgroundColor: 'rgb(12, 53, 71)'  },
        },
        shortestpath: {
          '0%': { transform: 'scale(.3)', borderRadius: '100%', backgroundColor: 'rgba(0, 0, 66, 0.75)'  },
          '25%': { backgroundColor: 'rgba(17, 104, 217, 0.75)' },
          '37%': { transform: 'scale(1.2)', backgroundColor: 'rgba(0, 217, 159, 0.75)'  },
          '50%': { transform: 'scale(1.0)', backgroundColor: 'rgba(0, 190, 218, 0.75)'  },
          '51%': { transform: 'scale(.6)', backgroundColor: 'rgb(255, 254, 106)'  },
          '75%': { transform: 'scale(1.2)', backgroundColor: 'rgb(255, 254, 106)'  },
          '100%': { transform: 'scale(1.0)', backgroundColor: 'rgb(255, 254, 106)'  },
        },
        visited: {
          '0%': { transform: 'scale(.3)', borderRadius: '100%', backgroundColor: 'rgba(0, 0, 66, 0.75)'  },
          '50%': { backgroundColor: 'rgba(17, 104, 217, 0.75)' },
          '75%': { transform: 'scale(1.2)', backgroundColor: 'rgba(0, 217, 159, 0.75)'  },
          '100%': { transform: 'scale(1.0)', backgroundColor: 'rgba(0, 190, 218, 0.75)'  },
        },
      },
      animation: {
        'wall': 'wall .3s ease-out',
        'shortestpath': 'shortestpath 3s ease-out',
        'visited': 'visited 1.5s ease-in',
      },
    },
  },
  plugins: [],
}

