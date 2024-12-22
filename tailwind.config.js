/** @type {import('tailwindcss').Config} */
const animate = require("tailwindcss-animate")

/** @type {import('tailwindcss').Config} */
module.exports = {

  plugins: [animate],
  theme: {
    extend: {

      colors: {

        'woodsmoke': {
          DEFAULT: '#09090B',
          50: '#B0B0BF',
          100: '#A5A5B5',
          200: '#8F8FA3',
          300: '#787891',
          400: '#65657B',
          500: '#525265',
          600: '#40404E',
          700: '#2E2E38',
          800: '#1B1B21',
          900: '#09090B',
          950: '#000000'
        },

        'white': {
          DEFAULT: '#FFFFFF',
          50: '#FFFFFF',
          100: '#FFFFFF',
          200: '#FFFFFF',
          300: '#FFFFFF',
          400: '#FFFFFF',
          500: '#FFFFFF',
          600: '#737373',
          700: '#575757',
          800: '#3B3B3B',
          900: '#1F1F1F',
          950: '#111111'
        },

      }
    }
  }
}