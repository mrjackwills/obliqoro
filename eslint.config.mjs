import pluginVue from 'eslint-plugin-vue';
import stylisticTs from '@stylistic/eslint-plugin-ts';

import eslint from '@eslint/js';
import tseslint from 'typescript-eslint';

import {
	defineConfigWithVueTs,
	vueTsConfigs
} from '@vue/eslint-config-typescript';

export default defineConfigWithVueTs(
	eslint.configs.recommended,
	pluginVue.configs['flat/essential'],
	tseslint.configs.strict,
	stylisticTs.configs.all,
	tseslint.configs.stylistic,
	vueTsConfigs.recommended,
	{
		rules: {
			'@stylistic/ts/indent': ['error', 'tab'],
			'@stylistic/ts/object-curly-newline': ['error', { multiline: true }],
			'@stylistic/ts/object-curly-spacing': ['error', 'always'],
			'@stylistic/ts/quote-props': ['error', 'as-needed'],
			'@stylistic/ts/quotes': ['error', 'single', { allowTemplateLiterals: true }],
			'@typescript-eslint/array-type': ['error', { default: 'generic' }],
			'@typescript-eslint/consistent-type-definitions': ['error', 'type'],
			'@typescript-eslint/explicit-function-return-type': ['error'],
			'@typescript-eslint/no-unused-vars': [
				'error',
				{
					args: 'all',
					argsIgnorePattern: '^_',
					caughtErrors: 'all',
					caughtErrorsIgnorePattern: '^_',
					destructuredArrayIgnorePattern: '^_',
					varsIgnorePattern: '^_',
					ignoreRestSiblings: true
				}
			],

			'comma-spacing': ['error', {
				before: false,
				after: true
			}],
			'max-len': ['error', { code: 200 }],
			'no-console': 'error',
			semi: ['error', 'always'],
			'space-before-blocks': ['error', {
				functions: 'always',
				keywords: 'always',
				classes: 'always'
			}],
			'vue/html-indent': ['error', 'tab', {
				attribute: 1,
				closeBracket: 0,
				alignAttributesVertically: true,
				ignores: []
			}],
			'vue/html-quotes': ['error', 'single'],
			'vue/mustache-interpolation-spacing': ['error', 'always'],
			'vue/script-indent': ['off']
		}
	}
);
