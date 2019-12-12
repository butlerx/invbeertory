import { inObject } from './object';
import { Beer } from '../types';

const barrelAged = (beer: Beer): string => (beer.barrelAged ? 'Barrel Aged' : '');

const brewedIn = (beer: Beer): string =>
  inObject('barrelType', beer) ? `in ${beer.barrelType} Barrels` : '';

const brewedWith = (beer: Beer): string =>
  inObject('brewedWith', beer) ? `with ${beer.brewedWith}` : '';

export const formatStyle = (beer: Beer): string =>
  `${barrelAged(beer)} ${beer.style} ${brewedWith(beer)} ${brewedIn(beer)}`;
