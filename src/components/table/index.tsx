import React from 'react';
import ReactTable, { ReactTableDefaults } from 'react-table';
import 'react-table/react-table.css';

import { Card } from '../card';
import { Deck } from '../deck';
import { Beer } from '../../types';
import { table } from './style.module.scss';
import { columns } from './columns';

function filterCaseInsensitive(
  filter: { value: string; id: string; pivotId: string },
  row: Beer,
): boolean {
  const content = row[filter.pivotId || filter.id];
  if (typeof content !== 'undefined') {
    // filter by text in the table or if it's a object, filter by key
    return typeof content === 'object' && content !== null && content.key
      ? String(content.key).toLowerCase().includes(filter.value.toLowerCase())
      : String(content).toLowerCase().includes(filter.value.toLowerCase());
  }
  return true;
}


interface Props {
  beers: Beer[];
  title?: string;
}

export const BeerTable: React.FC<Props> = ({ title, beers }) => (
  <Deck>
    <Card title={title}>
      <ReactTable
        className={['-striped', '-highlight', table].join(' ')}
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
        columns={columns}
      />
    </Card>
  </Deck>
);
