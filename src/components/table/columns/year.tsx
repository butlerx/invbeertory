import React, { ReactElement } from 'react';
import { Beer } from '../../../types';
import { center } from '../style.module.scss';

const Cell = ({ row }: { row: Beer }): ReactElement => <div className={center}>{row.year}</div>;

export const year = {
  Header: 'Year',
  accessor: 'year',
  minWidth: 6,
  Cell,
};
