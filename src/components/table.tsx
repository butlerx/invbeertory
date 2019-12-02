import React, { SFC } from 'react';
import ReactTable, { ReactTableDefaults } from 'react-table';
import 'react-table/react-table.css';

import { filterCaseInsensitive } from '../utils';
import { Deck, Card } from '.';
import { Beer } from '../types';
import { table } from './styles/table.module.scss'

interface Props {
  beers: Beer[];
  title?: string;
}

export const BeerTable: SFC<Props> = ({ title, beers }) => (
  <Deck>
    <Card title={title}>
      <ReactTable
        className={["-striped", "-highlight", table].join(' ')}
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
            Cell: ({ row }: { row: Beer }) => (
              <div
                style={{
                  textAlign: 'center',
                }}
              >
                {row.year}
              </div>
            ),
          },
          {
            Header: 'ABV',
            accessor: 'abv',
            minWidth: 6,
            Cell: ({ row }: { row: Beer }) => (
              <div
                style={{
                  textAlign: 'right',
                }}
              >
                {row.abv}%
              </div>
            ),
          },
          { Header: 'Style', accessor: 'style', minWidth: 40 },
          {
            Header: 'Size',
            accessor: 'size',
            minWidth: 8,
            Cell: ({ row }: { row: Beer }) => (
              <div
                style={{
                  textAlign: 'center',
                }}
              >
                {row.size} ml
              </div>
            ),
          },
          {
            Header: 'Stock',
            accessor: 'stock',
            minWidth: 6,
            Cell: ({ row }: { row: Beer }) => (
              <div
                style={{
                  textAlign: 'right',
                }}
              >
                {row.stock}
              </div>
            ),
          },
        ]}
      />
    </Card>
  </Deck>
);
