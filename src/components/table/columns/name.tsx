import React, { ReactElement } from 'react';
import { Link } from 'gatsby';
import { Beer } from '../../../types';
import { makeUrl } from '../../../utils';
import { path } from '../../styles/layout.module.scss';

const Cell = ({ row }: { row: Beer }): ReactElement => (
  <Link className={path} to={makeUrl(row)}>
    {row.name}
  </Link>
);

export const name = {
  Header: 'Name',
  accessor: 'name',
  minWidth: 50,
  Cell,
};
