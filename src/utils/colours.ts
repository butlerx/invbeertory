export const colours = [...Array(255).keys()].map(
  brewery => '#' + Math.floor(Math.random() * 16777215).toString(16),
);
