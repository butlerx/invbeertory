/* eslint-disable no-underscore-dangle */
import React from 'react';
import { Beer } from '../../../types';
import { TableBeer } from '../types';
import { inObject } from '../../../utils';
import { left } from '../style.module.scss';

const printBrewer = (beer: Beer): string =>
  beer.brewery + (inObject('collaborator', beer) ? ` with ${beer.collaborators}` : '');

const Cell = ({ row }: { row: TableBeer }): React.ReactElement => (
  <div className={left}>{printBrewer(row._original)}</div>
);

export const brewery = {
  Header: 'Brewery',
  accessor: 'brewery',
  minWidth: 26,
  Cell,
};
