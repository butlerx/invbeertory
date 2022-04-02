import React, { ReactElement } from 'react';
import { Beer } from '../../../types';
import { right } from '../style.module.scss';

const Cell = ({ row }: { row: Beer }): ReactElement => <div className={right}>{row.stock}</div>

export const stock = {
  Header: 'Stock',
  accessor: 'stock',
  minWidth: 6,
  Cell,
};
