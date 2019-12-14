import { Beer } from '../types';

export const makeUrl = ({ brewery, year, name }: Beer): string =>
  [brewery, year, name]
    .map(uri =>
      uri
        .toString()
        .replace(/\s+/g, '_')
        .toLowerCase(),
    )
    .join('/');
