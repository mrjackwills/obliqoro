// eslint.config.mjs
import pluginVue from 'eslint-plugin-vue';
import stylistic from '@stylistic/eslint-plugin';

import eslint from '@eslint/js';
import tseslint from 'typescript-eslint';

import {
	defineConfigWithVueTs,
	vueTsConfigs
} from '@vue/eslint-config-typescript';

const configs = [
	eslint.configs.recommended,
	pluginVue.configs['flat/essential'],
	tseslint.configs.strict,
	stylistic.configs.all,
	tseslint.configs.stylistic,
	vueTsConfigs.recommended
];

export default defineConfigWithVueTs(...configs, {
	rules: {

		'@stylistic/array-element-newline': [
			'error', {
				ArrayExpression: 'consistent',
				ArrayPattern: { minItems: 5 }
			}
		],
		'@stylistic/function-call-argument-newline': [
			'error',
			'never'
		],
		'@stylistic/indent': [
			'error',
			'tab'
		],
		'@stylistic/multiline-ternary': [
			'error',
			'always-multiline'
		],
		'@stylistic/no-confusing-arrow': ['off'],

		'@stylistic/object-curly-newline': [
			'error',
			{ multiline: true }
		],
		'@stylistic/object-curly-spacing': [
			'error',
			'always'
		],
		'@stylistic/padded-blocks': [
			'error',
			'never'
		],
		'@stylistic/quote-props': [
			'error',
			'as-needed'
		],
		'@stylistic/quotes': [
			'error',
			'single',
			{ allowTemplateLiterals: 'always' }
		],
		'@typescript-eslint/array-type': [
			'error',
			{ default: 'generic' }
		],
		'@typescript-eslint/consistent-type-definitions': [
			'error',
			'type'
		],
		'@typescript-eslint/explicit-function-return-type': ['error'],
		'comma-spacing': [
			'error',
			{
				before: false,
				after: true
			}
		],
		'max-len': [
			'error',
			{ code: 200 }
		],
		'no-console': 'error',
		semi: [
			'error',
			'always'
		],
		'space-before-blocks': [
			'error',
			{
				functions: 'always',
				keywords: 'always',
				classes: 'always'
			}
		],
		'vue/html-indent': [
			'error',
			'tab',
			{
				attribute: 1,
				closeBracket: 0,
				alignAttributesVertically: true,
				ignores: []
			}
		],
		'vue/html-quotes': [
			'error',
			'single'
		],
		'vue/mustache-interpolation-spacing': [
			'error',
			'always'
		],
		'vue/script-indent': ['off']
	}
});
