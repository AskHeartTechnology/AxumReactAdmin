/** @type {import('prettier').Config & import('prettier-plugin-tailwindcss').PluginOptions} */
const config = {
  semi: false,
  singleQuote: true,
  trailingComma: 'es5',
  jsxSingleQuote: false,
  arrowParens: 'avoid',
  plugins: ['prettier-plugin-tailwindcss'],
}

export default config
