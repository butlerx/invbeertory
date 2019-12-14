import React, { SFC } from 'react';
import { graphql } from 'gatsby';

import { Layout, SEO, Deck, Info } from '../components';
import { Beer } from '../types';

interface TemplateProps {
  data: { googleSheet: { inventory: Beer } };
}

const Stock: SFC<TemplateProps> = ({
  data: {
    googleSheet: { inventory },
  },
}) => (
  <Layout>
    <SEO title="Inventory" />
    <Deck>
      <Info beer={inventory} />
    </Deck>
  </Layout>
);

export const query = graphql`
  query Beer($id: String!) {
    googleSheet(inventory: { elemMatch: { id: { eq: $id } } }) {
      inventory {
        abv
        barrelAged
        barrelType
        brewedWith
        brewery
        collaborator
        drunk
        ibu
        name
        purchased
        size
        stock
        style
        year
      }
    }
  }
`;

export default Stock;
