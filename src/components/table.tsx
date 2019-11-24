import React, { FunctionComponent } from 'react';
import ReactTable from 'react-table';
import { ReactTableDefaults } from 'react-table';
import 'react-table/react-table.css';
import withDraggableColumns from 'react-table-hoc-draggable-columns';
import 'react-table-hoc-draggable-columns/dist/styles.css';
import { Beer } from '../types';

interface Props {
  beers: Beer[];
}

const ReactTableDraggableColumns = withDraggableColumns(ReactTable);

function filterCaseInsensitive(filter, row) {
  const content = row[filter.pivotId || filter.id];
  if (typeof content !== 'undefined') {
    // filter by text in the table or if it's a object, filter by key
    return typeof content === 'object' && content !== null && content.key
      ? String(content.key)
          .toLowerCase()
          .includes(filter.value.toLowerCase())
      : String(content)
          .toLowerCase()
          .includes(filter.value.toLowerCase());
  }
  return true;
}

export const BeerTable: FunctionComponent<Props> = ({ beers }) => (
  <ReactTableDraggableColumns
    draggableColumns={{
      mode: 'reorder',
      draggable: ['name', 'brewery', 'year', 'abv', 'style', 'size', 'stock'],
    }}
    data={beers}
    showPagination={false}
    defaultPageSize={beers.length}
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
);
