import { Beer } from '../types';

const fieldByField = (fieldA: string) => (fieldB: string) => (
  beers: Beer[],
): { [key: string]: number } => {
  const res: { [key: string]: number } = {};
  beers.forEach(beer => {
    res[beer[fieldB]] = parseInt(beer[fieldA], 10) + (beer[fieldB] in res ? res[beer[fieldB]] : 0);
  });
  return res;
};

export const purchasesBy = fieldByField('purchased');
export const stockBy = fieldByField('stock');
