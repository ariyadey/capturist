// TODO:
//  - Set up ESLint for JSON, CSS and Markdown:
//  https://github.com/eslint
//  - Integrate ESLint with Browserslist:
//  https://web.dev/articles/use-baseline-with-browserslist
//  - Integrate ESLint with Baseline:
//  https://web.dev/blog/eslint-baseline-integration
const eslint = require("@eslint/js");
const tslint = require("typescript-eslint");
const angular = require("angular-eslint");
const eslintConfigPrettier = require("eslint-config-prettier");

module.exports = tslint.config(
  {
    files: ["**/*.ts"],
    extends: [
      eslint.configs.recommended,
      ...tslint.configs.recommended,
      ...tslint.configs.stylistic,
      ...angular.configs.tsRecommended,
      eslintConfigPrettier,
    ],
    processor: angular.processInlineTemplates,
    rules: {
      "@typescript-eslint/array-type": ["error", { default: "generic", readonly: "generic" }],
      "@typescript-eslint/no-unused-expressions": ["error", { allowTernary: true }],
      "@typescript-eslint/no-unused-vars": ["off"], // Delegated to TS compiler
      "@typescript-eslint/consistent-type-definitions": ["error", "type"],
      "@angular-eslint/component-selector": [
        "error",
        {
          type: "element",
          prefix: "cpt",
          style: "kebab-case",
        },
      ],
      "@angular-eslint/directive-selector": [
        "error",
        {
          type: "attribute",
          style: "camelCase",
        },
      ],
    },
  },
  {
    files: ["**/*.html"],
    extends: [...angular.configs.templateRecommended, ...angular.configs.templateAccessibility],
    rules: {
      "@angular-eslint/template/eqeqeq": ["error", { allowNullOrUndefined: true }],
    },
  },
);
