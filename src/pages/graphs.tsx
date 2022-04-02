import React from 'react';
import { graphql } from 'gatsby';

import { Layout, SEO, Pie, Bar, Card, Deck, StackedBar } from '../components';
import { StockProps } from '../types';
import {
  purchasesByBrewery,
  purchasesByStyle,
  stockByAbv,
  sortIntKeys,
  breweryByStyle,
} from '../utils';

const Graphs: React.FC<StockProps> = ({
  data: {
    googleSheet: { inventory },
  },
}) => (
  <Layout>
    <SEO title="Brewery Stats" />
    <Deck>
      <Card>
        <Pie title="Beers by Brewery" data={purchasesByBrewery(inventory)} />
      </Card>
      <Card>
        <Bar title="ABV in stock" data={sortIntKeys(stockByAbv(inventory))} />
      </Card>
      <Card>
        <Pie title="Beers by Style" data={purchasesByStyle(inventory)} />
      </Card>
      <Card>
        <StackedBar title="Brewery by Style" data={breweryByStyle(inventory)} />
      </Card>
    </Deck>
  </Layout>
);

export const query = graphql`
  query {
    googleSheet {
      inventory {
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
