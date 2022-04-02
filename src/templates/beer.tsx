import React from 'react';
import { graphql } from 'gatsby';

import { Layout, SEO, Deck, Info } from '../components';
import { Beer } from '../types';

interface TemplateProps {
  data: { googleInventorySheet: Beer };
}

const Stock: React.FC<TemplateProps> = ({ data: { googleInventorySheet } }) => (
  <Layout>
    <SEO title="Inventory" />
    <Deck>
      <Info beer={googleInventorySheet} />
    </Deck>
  </Layout>
);

export const query = graphql`
  query Beer($id: String!) {
    googleInventorySheet(id: { eq: $id }) {
      abv
      barrelAged
      barrelType
      brewedWith
      brewery
      collaborators
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
`;

export default Stock;
