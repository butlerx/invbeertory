import React, { SFC } from 'react';
import { graphql } from 'gatsby';

import { Layout, SEO, Hero } from '../components';

interface Props {
  data: {
    site: {
      siteMetadata: {
        title: string;
        description: string;
      };
    };
  };
}

const Index: SFC<Props> = ({
  data: {
    site: {
      siteMetadata: { title, description },
    },
  },
}) => (
  <Layout>
    <SEO title="Invbeertory" />
    <Hero title={title} message={description} />
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
  }
`;

export default Index;
