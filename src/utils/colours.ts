export const colours = [...Array(255).keys()].map(
  brewery => '#' + Math.floor(Math.random() * 16777215).toString(16),
);

export const baseColours = {
  '00': '#2d2d2d',
  '01': '#393939',
  '02': '#515151',
  '03': '#747369',
  '04': '#a09f93',
  '05': '#d3d0c8',
  '06': '#e8e6df',
  '07': '#f2f0ec',
  '08': '#f2777a',
  '09': '#f99157',
  '0a': '#fc6',
  '0b': '#9c9',
  '0c': '#6cc',
  '0d': '#69c',
  '0e': '#c9c',
  '0f': '#d27b53',
};

export const base = (num: string) => baseColours[num];
