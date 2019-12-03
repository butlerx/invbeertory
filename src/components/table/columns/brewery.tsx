import React from 'react';
import { Beer } from '../../../types';
import { TableBeer } from '../types';
import { inObject } from '../../../utils';
import { left } from '../style.module.scss';

const printBrewer = (beer: Beer): string =>
  beer.brewery + (inObject('collaborator', beer) ? ` with ${beer.collaborator}` : '');

export const brewery = ({ row }: { row: TableBeer }) => (
  <div className={left}>{printBrewer(row._original)}</div>
);
