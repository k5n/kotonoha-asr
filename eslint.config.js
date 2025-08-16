import eslint from '@eslint/js';
import prettier from 'eslint-config-prettier';
import svelte from 'eslint-plugin-svelte';
import globals from 'globals';
import svelteParser from 'svelte-eslint-parser';
import tsEslint from 'typescript-eslint';

export default tsEslint.config(
  {
    // .eslintignore の代替
    ignores: ['src-tauri/', 'doc/', '.vscode/', '.svelte-kit/'],
  },
  eslint.configs.recommended,
  svelte.configs['flat/recommended'],
  tsEslint.configs.recommended,
  {
    rules: {
      'no-unused-vars': 'off',
      '@typescript-eslint/no-unused-vars': [
        'warn',
        { argsIgnorePattern: '^_', varsIgnorePattern: '^_', caughtErrorsIgnorePattern: '^_' },
      ],
    },
  },
  // Other config for non-Svelte files
  {
    languageOptions: {
      parser: tsEslint.parser,
      parserOptions: {
        extraFileExtensions: ['.svelte'],
      },
    },
  },
  // Svelte config
  {
    files: [
      '**/*.svelte',
      '**/*.svelte.ts', // Svelte files with TypeScript
    ],
    languageOptions: {
      parser: svelteParser,
      // Parse the `<script>` in `.svelte` as TypeScript by adding the following configuration.
      parserOptions: {
        parser: tsEslint.parser,
      },
      globals: {
        ...globals.browser,
      },
    },
  },
  // For configuration files used in Node.js environment
  // It's okay to use the .js extension instead of the .mjs extension。
  {
    files: ['*.js'],
    languageOptions: {
      ecmaVersion: 2022,
      sourceType: 'module',
      globals: {
        ...globals.node,
      },
    },
  },
  // Disable rules that conflict with Prettier
  prettier
);
