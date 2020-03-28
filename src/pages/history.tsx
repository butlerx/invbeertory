import React, { SFC } from 'react';
import { graphql } from 'gatsby';

import { Layout, SEO, BeerTable } from '../components';
import { StockProps } from '../types';

const History: SFC<StockProps> = ({
  data: {
    allGoogleInventorySheet: { nodes },
  },
}) => (
  <Layout>
    <SEO title="History" />
    <BeerTable title="History" beers={nodes} />
  </Layout>
);

export const query = graphql`
  query {
    allGoogleInventorySheet {
      nodes {
        name
        brewery
        year
        abv
        style
        size
        stock
        purchased
        collaborators
        barrelAged
        barrelType
        brewedWith
      }
    }
  }
`;

export default History;
