/* eslint-disable no-underscore-dangle */

import React, { ReactElement } from 'react';
import { TableBeer } from '../types';
import { formatStyle } from '../../../utils';
import { left } from '../style.module.scss';

const Cell = ({ row }: { row: TableBeer }): ReactElement => <div className={left}>{formatStyle(row._original)}</div>

export const style = {
  Header: 'Style',
  accessor: 'style',
  minWidth: 40,
  Cell,
};
