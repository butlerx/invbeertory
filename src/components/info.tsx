import React from 'react';

import { Beer } from '../types';
import { Card } from './card';
import { inObject, formatStyle } from '../utils';

interface Props {
  beer: Beer;
}

const formatBrewer = (beer: Beer): string =>
  inObject('collaborator', beer)
    ? `${beer.brewery} in collaboration with ${beer.collaborators}`
    : beer.brewery;

const formatTitle = (beer: Beer): string => `${beer.name} (${beer.year})`;

export const Info: React.FC<Props> = ({ beer }) => (
  <Card
    title={formatTitle(beer)}
    meta={{ purchased: beer.purchased, stock: beer.stock, abv: `${beer.abv}%` }}
  >
    {beer.name} by {formatBrewer(beer)} is a {formatStyle(beer)}.
  </Card>
);
