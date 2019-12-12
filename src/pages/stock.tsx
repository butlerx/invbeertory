import React, { SFC } from 'react';
import { graphql } from 'gatsby';

import { Layout, SEO, BeerTable } from '../components';
import { StockProps } from '../types';

const Stock: SFC<StockProps> = ({
  data: {
    allGoogleSheetInventoryRow: { nodes },
  },
}) => (
  <Layout>
    <SEO title="Inventory" />
    <BeerTable title="Current Stock" beers={nodes} />
  </Layout>
);

export const query = graphql`
  query {
    allGoogleSheetInventoryRow(filter: { stock: { ne: 0 } }) {
      nodes {
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
