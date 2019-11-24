import React, { SFC } from 'react';
import { Link, graphql } from 'gatsby';

import { Layout, SEO, BeerTable } from '../components';
import { Beer } from '../types';

interface Props {
  data: {
    allGoogleSpreadsheetBeerInv: {
      nodes: Beer[];
    };
  };
}

const IndexPage: SFC<Props> = ({ data }) => (
  <Layout>
    <SEO title="Home" />
    <BeerTable beers={data.allGoogleSpreadsheetBeerInv.nodes} />
  </Layout>
);

export const query = graphql`
  query {
    allGoogleSpreadsheetBeerInv {
      nodes {
        name
        brewery
        year
        abv
        style
        size
        stock
      }
    }
  }
`;

export default IndexPage;
