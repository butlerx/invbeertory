import React, { SFC } from 'react';
import { Link, graphql } from 'gatsby';

import { Layout, SEO, Pie, Bar, Card } from '../components';
import { Beer } from '../types';
import { purchasesByBrewery, purchasesByStyle, stockByAbv, sortIntKeys } from '../utils';

interface Props {
  data: {
    allGoogleSpreadsheetBeerInv: {
      nodes: Beer[];
    };
  };
}

const IndexPage: SFC<Props> = ({
  data: {
    allGoogleSpreadsheetBeerInv: { nodes },
  },
}) => (
  <Layout>
    <SEO title="Brewery Stats" />
    <Card>
      <Pie title="Beers by Brewery" data={purchasesByBrewery(nodes)} />
    </Card>
    <Card>
      <Bar title="ABV in stock" data={sortIntKeys(stockByAbv(nodes))} />
    </Card>
  </Layout>
);

export const query = graphql`
  query {
    allGoogleSpreadsheetBeerInv {
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

export default IndexPage;
