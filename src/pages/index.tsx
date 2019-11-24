import React, { SFC } from 'react';
import { Link, graphql } from 'gatsby';
import ReactTable, { ReactTableDefaults } from 'react-table';
import 'react-table/react-table.css';

import { Layout, SEO } from '../components';
import { Beer } from '../types';
import { filterCaseInsensitive } from '../utils';

interface Props {
  data: {
    allGoogleSpreadsheetBeerInv: {
      nodes: Beer[];
    };
  };
}

const IndexPage: SFC<Props> = ({
  data: {
    allGoogleSpreadsheetBeerInv: { nodes },
  },
}) => {
  return (
    <Layout>
      <SEO title="Inventory" />
      <ReactTable
        data={nodes}
        showPagination={false}
        defaultPageSize={nodes.length}
        defaultFilterMethod={filterCaseInsensitive}
        column={{
          ...ReactTableDefaults.column,
          sortable: true,
          filterable: true,
          minWidth: 40,
        }}
        columns={[
          { Header: 'Name', accessor: 'name' },
          { Header: 'Brewery', accessor: 'brewery' },
          { Header: 'Year', accessor: 'year', minWidth: 10 },
          {
            Header: 'ABV',
            accessor: 'abv',
            minWidth: 10,
            Cell: row => <>{row.row.size}%</>,
          },
          { Header: 'Style', accessor: 'style' },
          {
            Header: 'Size',
            accessor: 'size',
            minWidth: 10,
            Cell: row => <>{row.row.size} ml</>,
          },
          {
            Header: 'Current Stock',
            accessor: 'stock',
            minWidth: 15,
            Cell: row => (
              <div
                style={{
                  textAlign: 'right',
                }}
              >
                {row.row.stock}
              </div>
            ),
          },
        ]}
      />
    </Layout>
  );
};

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
