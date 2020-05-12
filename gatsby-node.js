/* eslint-disable  @typescript-eslint/explicit-function-return-type */
const beerTemplate = require('path').resolve('src/templates/beer.tsx');

exports.createPages = ({ actions, graphql, reporter }) =>
  graphql(`
    query {
      googleSheet {
        inventory {
          id
          name
          brewery
          year
        }
      }
    }
  `).then(result => {
    if (result.errors) {
      reporter.panicOnBuild('🚨  ERROR: Loading "createPages" query', result.errors);
    }
    const { createPage } = actions;
    const { googleSheet } = result.data;
    googleSheet.inventory.forEach(({ id, name, brewery, year }) => {
      if (name === null || brewery === null || year === null) return;
      createPage({
        path: [brewery, year, name]
          .map(uri =>
            uri
              .toString()
              .replace(/(\s+|#)/g, '_')
              .toLowerCase(),
          )
          .join('/'),
        component: beerTemplate,
        context: { id },
      });
    });
  });
