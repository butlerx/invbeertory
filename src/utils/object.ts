/* eslint-disable @typescript-eslint/no-explicit-any */

interface IntDict {
  [key: number]: any;
}

interface StrDict {
  [key: string]: any;
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

export const inObject = (key: string, obj: StrDict): boolean =>
  key in obj && obj[key] !== undefined && typeof obj[key] !== 'undefined' && obj[key] !== null;
