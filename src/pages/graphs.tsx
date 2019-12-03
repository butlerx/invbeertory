import React, { SFC } from 'react';
import { Link, graphql } from 'gatsby';

import { Layout, SEO, Pie, Bar, Card, Deck } from '../components';
import { Beer, StockProps } from '../types';
import { purchasesByBrewery, purchasesByStyle, stockByAbv, sortIntKeys } from '../utils';

const Graphs: SFC<StockProps> = ({
  data: {
    allGoogleSheetBeerInvRow: { nodes },
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
    </Deck>
  </Layout>
);

export const query = graphql`
  query {
    allGoogleSheetBeerInvRow {
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
