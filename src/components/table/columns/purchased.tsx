import React, { ReactElement } from 'react';
import { Beer } from '../../../types';
import { right } from '../style.module.scss';

const Cell = ({ row }: { row: Beer }): ReactElement => <div className={right}>{row.purchased}</div>

export const purchased = {
  Header: 'Purchased',
  accessor: 'purchased',
  minWidth: 9,
  Cell,
};
