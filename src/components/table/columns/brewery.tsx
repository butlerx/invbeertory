import React from 'react';
import { Beer } from '../../../types';
import { inObject } from '../../../utils';
import { left } from '../style.module.scss';

const printBrewer = (beer: Beer): string => {
  return beer.brewery + (inObject('collaborator', beer) ? ` with ${beer.collaborator}` : '');
};

export const brewery = ({ row }: { row: Beer }) => <div className={left}>{printBrewer(row)}</div>;
