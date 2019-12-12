import React, { SFC } from 'react';
import { graphql } from 'gatsby';

import { Layout, SEO, Deck, Info } from '../components';
import { Beer } from '../types';

interface TemplateProps {
  data: { googleSheetInventoryRow: Beer };
}

const Stock: SFC<TemplateProps> = ({ data: { googleSheetInventoryRow } }) => (
  <Layout>
    <SEO title="Inventory" />
    <Deck>
      <Info beer={googleSheetInventoryRow} />
    </Deck>
  </Layout>
);

export const query = graphql`
  query Beer($id: String!) {
    googleSheetInventoryRow(id: { eq: $id }) {
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
`;

export default Stock;
