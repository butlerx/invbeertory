module.exports = {
  singleQuote: true,
  trailingComma: 'all',
  proseWrap: 'always',
  overrides: [
    {
      files: ['*.js', '*.ts', '*.jsx', '*.tsx'],
      options: {
        printWidth: 100,
      },
    },
  ],
};
