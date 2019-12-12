import { Beer } from '../types';

export const makeUrl = (beer: Beer): string =>
  [beer.brewery, beer.year, beer.name].map(uri => uri.toString().toLowerCase()).join('/');
