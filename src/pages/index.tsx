import React, { SFC } from 'react';
import { graphql } from 'gatsby';

import { Layout, SEO, Hero, Search } from '../components';

interface Props {
  data: {
    site: {
      siteMetadata: {
        title: string;
        description: string;
      };
    };
    siteSearchIndex: { index: Record<string, string> };
  };
}

const Index: SFC<Props> = ({
  data: {
    site: {
      siteMetadata: { title, description },
    },
    siteSearchIndex,
  },
}) => (
  <Layout>
    <SEO title="Invbeertory" />
    <Hero title={title} message={description} />
    <Search searchIndex={siteSearchIndex.index} />
  </Layout>
);

export const query = graphql`
  query {
    site {
      siteMetadata {
        title
        description
      }
    }
    siteSearchIndex {
      index
    }
  }
`;

export default Index;
