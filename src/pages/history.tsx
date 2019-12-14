import React, { SFC } from 'react';
import { graphql } from 'gatsby';

import { Layout, SEO, BeerTable } from '../components';
import { StockProps } from '../types';

const History: SFC<StockProps> = ({
  data: {
    googleSheet: { inventory },
  },
}) => (
  <Layout>
    <SEO title="History" />
    <BeerTable title="History" beers={inventory} />
  </Layout>
);

export const query = graphql`
  query {
    googleSheet {
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

export default History;
