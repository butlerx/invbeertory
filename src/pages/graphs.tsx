import React, { SFC } from 'react';
import { graphql } from 'gatsby';

import { Layout, SEO, Pie, Bar, Card, Deck } from '../components';
import { StockProps } from '../types';
import { purchasesByBrewery, purchasesByStyle, stockByAbv, sortIntKeys } from '../utils';

const Graphs: SFC<StockProps> = ({
  data: {
    allGoogleSheetInventoryRow: { nodes },
  },
}) => (
  <Layout>
    <SEO title="Brewery Stats" />
    <Deck>
      <Card>
        <Pie title="Beers by Brewery" data={purchasesByBrewery(nodes)} />
      </Card>
      <Card>
        <Bar title="ABV in stock" data={sortIntKeys(stockByAbv(nodes))} />
      </Card>
      <Card>
        <Pie title="Beers by Style" data={purchasesByStyle(nodes)} />
      </Card>
    </Deck>
  </Layout>
);

export const query = graphql`
  query {
    allGoogleSheetInventoryRow {
      nodes {
        brewery
        abv
        style
        stock
        purchased
      }
    }
  }
`;

export default Graphs;
