import React, { SFC } from 'react';
import { Link, graphql } from 'gatsby';

import { Layout, SEO, BeerTable } from '../components';
import { Beer, StockProps } from '../types';

const Stock: SFC<StockProps> = ({
  data: {
    allGoogleSheetBeerInvRow: { nodes },
  },
}) => (
  <Layout>
    <SEO title="Inventory" />
    <BeerTable title="Current Stock" beers={nodes} />
  </Layout>
);

export const query = graphql`
  query {
    allGoogleSheetBeerInvRow(filter: { stock: { ne: 0 } }) {
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