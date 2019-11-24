import { Beer } from '../types';

const fieldByField = (fieldA: string) => (fieldB: string) => (
  beers: Beer[],
): { [string]: number } => {
  const res = {};
  beers.forEach(beer => {
    res[beer[fieldB]] = parseInt(beer[fieldA], 10) + (beer[fieldB] in res ? res[beer[fieldB]] : 0);
  });
  return res;
};

export const purchasesBy = fieldByField('purchased');
export const stockBy = fieldByField('stock');
