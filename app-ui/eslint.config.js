import js from '@eslint/js';
import pluginVue from 'eslint-plugin-vue';

export default [
  js.configs.recommended,
  ...pluginVue.configs['flat/recommended'],
  {
    languageOptions: {
      ecmaVersion: 2022,
      sourceType: 'module',
      globals: {
        __PACKAGE_VERSION__: 'readonly',
        window: 'readonly',
        document: 'readonly',
        localStorage: 'readonly',
        navigator: 'readonly',
        location: 'readonly',
        fetch: 'readonly',
        URLSearchParams: 'readonly',
        setTimeout: 'readonly',
        clearTimeout: 'readonly',
      }
    },
    rules: {
      'array-bracket-spacing': 'warn',
      'brace-style': 'warn',
      'comma-spacing': 'warn',
      'eol-last': 'warn',
      'eqeqeq': 'warn',
      'indent': ['error', 2, { 'SwitchCase': 1 }],
      'keyword-spacing': 'warn',
      'no-mixed-spaces-and-tabs': 'warn',
      'no-undef': 'error',
      'no-unused-vars': 'warn',
      'no-var': 'warn',
      'object-shorthand': ['warn', 'methods'],
      'prefer-arrow-callback': 'warn',
      'quotes': ['error', 'single'],
      'semi': ['error', 'always'],
      'space-before-blocks': 'warn',
      'space-infix-ops': 'warn',
      'vue/first-attribute-linebreak': 'off',
      'vue/html-closing-bracket-newline': 'off',
      'vue/html-indent': 'off',
      'vue/max-attributes-per-line': 'off',
      'vue/multi-word-component-names': 'off',
      'vue/singleline-html-element-content-newline': 'off',
    }
  }
];
