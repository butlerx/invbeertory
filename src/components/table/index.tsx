import React from 'react';
import ReactTable, { ReactTableDefaults } from 'react-table';
import 'react-table/react-table.css';

import { filterCaseInsensitive } from '../../utils';
import { Card } from '../card';
import { Deck } from '../deck';
import { Beer } from '../../types';
import { table } from './style.module.scss';
import { columns } from './columns';

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

BeerTable.defaultProps = {
  title: '',
};
