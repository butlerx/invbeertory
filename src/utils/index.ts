import { purchasesBy, stockBy } from './occurences';
import { Beer } from '../types';

export function filterCaseInsensitive(
  filter: { value: string; id: string; pivotId: string },
  row: Beer,
): boolean {
  const content = row[filter.pivotId || filter.id];
  if (typeof content !== 'undefined') {
    // filter by text in the table or if it's a object, filter by key
    return typeof content === 'object' && content !== null && content.key
      ? String(content.key).toLowerCase().includes(filter.value.toLowerCase())
      : String(content).toLowerCase().includes(filter.value.toLowerCase());
  }
  return true;
}

export const makeUrl = ({ brewery, year, name }: Beer): string =>
  `/${[brewery, year, name]
    .map((uri: string | number) =>
      uri
        .toString()
        .replace(/(\s+|#)/g, '_')
        .toLowerCase(),
    )
    .join('/')}`;

interface IntDict {
  [key: number]: unknown;
}

export const sortIntKeys = (dict: IntDict): IntDict =>
  Object.keys(dict)
    .map(parseFloat)
    .sort((a, b) => a - b)
    .map((key: string) => [key.toFixed(1), dict[key]])
    .reduce((p: IntDict, c) => {
      const [key, value] = c;
      p[key] = value;
      return p;
    }, {});
