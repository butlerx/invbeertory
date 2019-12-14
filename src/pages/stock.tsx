import React, { SFC } from 'react';
import { graphql } from 'gatsby';

import { Layout, SEO, BeerTable } from '../components';
import { StockProps } from '../types';

const Stock: SFC<StockProps> = ({
  data: {
    googleSheet: { inventory },
  },
}) => (
  <Layout>
    <SEO title="Inventory" />
    <BeerTable title="Current Stock" beers={inventory} />
  </Layout>
);

export const query = graphql`
  query {
    googleSheet(inventory: { elemMatch: { stock: { ne: 0 } } }) {
      inventory {
        name
        brewery
        year
        abv
        style
        size
        stock
        collaborator
        barrelAged
        barrelType
        brewedWith
      }
    }
  }
`;

export default Stock;
