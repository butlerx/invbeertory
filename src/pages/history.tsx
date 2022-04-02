import React from 'react';
import { graphql } from 'gatsby';

import { Layout, SEO, BeerTable } from '../components';
import { StockProps } from '../types';

const History: React.FC<StockProps> = ({
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
    allGoogleInventorySheet(filter: { name: { ne: null } }) {
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
