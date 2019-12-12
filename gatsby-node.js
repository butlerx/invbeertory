/* eslint-disable  @typescript-eslint/explicit-function-return-type */
const beerTemplate = require('path').resolve('src/templates/beer.tsx');

exports.createPages = ({ actions, graphql, reporter }) =>
  graphql(`
    query {
      allGoogleSheetInventoryRow {
        nodes {
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
    const { allGoogleSheetInventoryRow } = result.data;
    allGoogleSheetInventoryRow.nodes.forEach(({ id, name, brewery, year }) => {
      createPage({
        path: [brewery, year, name].map(uri => uri.toString().toLowerCase()).join('/'),
        component: beerTemplate,
        context: { id },
      });
    });
  });
