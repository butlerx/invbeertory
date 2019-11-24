import React, { SFC } from 'react';
import ReactTable, { ReactTableDefaults } from 'react-table';
import 'react-table/react-table.css';

import { filterCaseInsensitive } from '../utils';
import { Beer } from '../types';

interface Props {
  beers: Beer[];
}

export const BeerTable: SFC<Props> = ({ beers }) => (
  <ReactTable
    data={beers}
    showPagination={false}
    defaultPageSize={beers.length}
    defaultFilterMethod={filterCaseInsensitive}
    column={{
      ...ReactTableDefaults.column,
      sortable: true,
      filterable: true,
      minWidth: 20,
    }}
    columns={[
      { Header: 'Name', accessor: 'name', minWidth: 50 },
      { Header: 'Brewery', accessor: 'brewery', minWidth: 26 },
      {
        Header: 'Year',
        accessor: 'year',
        minWidth: 6,
        Cell: row => (
          <div
            style={{
              textAlign: 'center',
            }}
          >
            {row.row.year}
          </div>
        ),
      },
      {
        Header: 'ABV',
        accessor: 'abv',
        minWidth: 6,
        Cell: row => (
          <div
            style={{
              textAlign: 'right',
            }}
          >
            {row.row.abv}%
          </div>
        ),
      },
      { Header: 'Style', accessor: 'style', minWidth: 40 },
      {
        Header: 'Size',
        accessor: 'size',
        minWidth: 8,
        Cell: row => (
          <div
            style={{
              textAlign: 'center',
            }}
          >
            {row.row.size} ml
          </div>
        ),
      },
      {
        Header: 'Stock',
        accessor: 'stock',
        minWidth: 6,
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
);
