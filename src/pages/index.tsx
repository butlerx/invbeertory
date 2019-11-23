import React, { SFC } from 'react';
import { Link, graphql } from 'gatsby';

import { Layout, SEO, Table } from '../components';

interface Props {
  data: {
    allGoogleSpreadsheetSheet1: {
      edges: [
        {
          node: {
            name: string;
            brewery: string;
            year: number;
            abv: number;
            style: string;
            size: number;
            rating: number;
            drunk: boolean;
            stock: number;
          };
        },
      ];
    };
  };
}

const IndePage: SFC<Props> = ({ data }) => (
  <Layout>
    <SEO title="Home" />
    <Table
      headings={['name', 'brewery', 'year', 'abv', 'style', 'size', 'rating', 'drunk', 'stock']}
      data={data.allGoogleSpreadsheetSheet1.edges.map(({ node }) => node)}
    />
    <Link to="/page-2/">Go to page 2</Link>
  </Layout>
);

export const query = graphql`
  query {
    allGoogleSpreadsheetSheet1 {
      edges {
        node {
          name
          brewery
          year
          abv
          style
          size
          rating
          drunk
          stock
        }
      }
    }
  }
`;

export default IndexPage;
