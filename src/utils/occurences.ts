import { Beer } from '../types';

const fieldByField =
  (fieldA: string) =>
  (fieldB: string) =>
  (beers: Beer[]): { [key: string]: number } => {
    const res: { [key: string]: number } = {};
    beers.forEach((beer: Beer) => {
      res[beer[fieldB]] =
        parseInt(beer[fieldA], 10) + (beer[fieldB] in res ? res[beer[fieldB]] : 0);
    });
    return res;
  };

export const purchasesBy = fieldByField('purchased');
export const stockBy = fieldByField('stock');

interface Dataset {
  label: string;
  data: number[];
}

export function breweryByStyle(beers: Beer[]): { labels: string[]; datasets: Dataset[] } {
  const breweries: { [brewery: string]: { [style: string]: number } } = {};
  beers.forEach((beer: Beer) => {
    if (!(beer.brewery in breweries)) breweries[beer.brewery] = {};
    if (!(beer.style in breweries[beer.brewery])) breweries[beer.brewery][beer.style] = 0;
    breweries[beer.brewery][beer.style] += beer.purchased;
  });

  const labels = Object.keys(breweries);
  const datasets: Dataset[] = [];
  Object.entries(breweries).forEach(([brewery, styles]) => {
    Object.entries(styles).forEach(([style, count]) => {
      let index = Object.values(datasets).findIndex((dataset: Dataset) => dataset.label === style);
      if (index === -1) {
        datasets.push({ label: style, data: Array(labels.length).fill(0) });
        index = datasets.length - 1;
      }
      datasets[index].data[labels.indexOf(brewery)] = count;
    });
  });
  return {
    labels,
    datasets,
  };
}
