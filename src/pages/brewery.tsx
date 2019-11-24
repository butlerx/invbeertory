import React, { SFC } from 'react';
import { Link, graphql } from 'gatsby';

import { Layout, SEO, Pie } from '../components';
import { Beer } from '../types';
import { purchasesByBrewery } from '../utils';

interface Props {
  data: {
    allGoogleSpreadsheetBeerInv: {
      nodes: Beer[];
    };
  };
}

const IndexPage: SFC<Props> = ({ data }) => (
  <Layout>
    <SEO title="Brewery Stats" />
    <Pie
      title="Beers by Brewery"
      data={purchasesByBrewery(data.allGoogleSpreadsheetBeerInv.nodes)}
    />
    <Link to="/">Go Home</Link>
  </Layout>
);

export const query = graphql`
  query {
    allGoogleSpreadsheetBeerInv {
      nodes {
        brewery
        purchased
      }
    }
  }
`;

export default IndexPage;
