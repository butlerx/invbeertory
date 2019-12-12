import React, { ReactElement } from 'react';
import { Beer } from '../../../types';
import { center } from '../style.module.scss';

const Cell = ({ row }: { row: Beer }): ReactElement => <div className={center}>{row.size} ml</div>;

export const size = {
  Header: 'Size',
  accessor: 'size',
  minWidth: 8,
  Cell,
};
