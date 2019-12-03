export const sortIntKeys = (dict: { [key: number]: any }): { [key: number]: any } =>
  Object.keys(dict)
    .map(parseFloat)
    .sort((a, b) => a - b)
    .map(key => [key.toFixed(1), dict[key]])
    .reduce((p, c) => {
      p[c[0]] = c[1];
      return p;
    }, {});
