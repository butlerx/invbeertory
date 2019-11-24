import { purchasesBy, stockBy } from './occurences';
export * from './dates';

export const colours = [...Array(255).keys()].map(
  brewery => '#' + Math.floor(Math.random() * 16777215).toString(16),
);

export const purchasesByStyle = purchasesBy('style');
export const purchasesByBrewery = purchasesBy('brewery');
export const stockByAbv = stockBy('abv');
