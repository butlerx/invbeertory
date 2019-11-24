import { Beer } from '../types';

export function purchasesByBrewery(beers: Beer[]): { [string]: number } {
  const res = {};
  beers.forEach(beer => {
    if (beer.brewery in res) {
      res[beer.brewery] += parseInt(beer.purchased, 10);
    } else {
      res[beer.brewery] = parseInt(beer.purchased, 10);
    }
  });
  return res;
}
