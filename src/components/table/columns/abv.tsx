import React from 'react';
import { Beer } from '../../../types';
import { right } from '../style.module.scss';

const Cell = ({ row }: { row: Beer }): React.ReactElement => (
  <div className={right}>{row.abv} %</div>
);

export const abv = {
  Header: 'ABV',
  accessor: 'abv',
  minWidth: 6,
  Cell,
};
