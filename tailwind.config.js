const defaultTheme = require('tailwindcss/defaultTheme')

module.exports = {
    content: [
        "./public/**/*.html",
        "./src/**/*.{vue,js,ts,jsx,tsx}",
        "./index.html"
    ],
    theme: {
        extend: {
            fontFamily: {
                sans: ['Inter var', ...defaultTheme.fontFamily.sans],
            },
        },
    },
    plugins: [],
};