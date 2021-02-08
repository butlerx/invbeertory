import { Beer } from '../types';

export const makeUrl = ({ brewery, year, name }: Beer): string =>
  `/${[brewery, year, name]
    .map((uri: string | number) =>
      uri
        .toString()
        .replace(/(\s+|#)/g, '_')
        .toLowerCase(),
    )
    .join('/')}`;
